use async_trait::async_trait;
use log::error;
use serde::Deserialize;
use core::str;

use bytes::Bytes;
use tokio::sync::mpsc::{Sender, Receiver, channel};

use crate::{backend::{chat::ChatResponse, llm::PromptResponse}, errors};

pub(crate) enum NdJsonData<T> {
    Data(T),
    End
}

/// Reads raw responses as ND-JSON (like from Ollama) and parses them into
/// a token stream. We expect UTF-8 data (the default using Ollama).
pub(crate) struct NdJsonReader<T> {
    tx_data: Sender<NdJsonData<Bytes>>,
    rx_events: Option<Receiver<NdJsonData<T>>>
}

impl<T> NdJsonReader<T>
where
    T: for<'de> Deserialize<'de> + Send + 'static
{
    pub fn new() -> Self {
        const BUFFER_SIZE: usize = 1024;
        let (tx, mut rx) = channel(BUFFER_SIZE);
        let (tx_ev, rx_ev) = channel(256);
        let obj = NdJsonReader {
            tx_data: tx,
            rx_events: Some(rx_ev)
        };

        tokio::spawn(async move {
            let mut buffer: Vec<NdJsonData<Bytes>> = Vec::with_capacity(BUFFER_SIZE);
            let mut json_buffer = String::new();

            'outer: loop {
                if rx.is_closed() {
                    break;
                }

                // Buffer data until we have a complete JSON object
                // We send the object right after parsing it and clear the JSON buffer
                let _ = rx.recv_many(&mut buffer, BUFFER_SIZE).await;
                for data in &buffer {
                    match data {
                        NdJsonData::End => break 'outer,
                        NdJsonData::Data(data) => {
                            // Buffer the data as string
                            match str::from_utf8(&data) {
                                Ok(str) => json_buffer.push_str(str),
                                Err(e) => {
                                    error!("Unexpected Ollama string encoding: {:?}", e);
                                    let _ = tx_ev.send(NdJsonData::End).await;
                                    break 'outer;
                                }
                            };

                            // Search for newline
                            let Some((json_slice, slice)) = json_buffer.split_once('\n') else {
                                continue;
                            };

                            // We have a full JSON object
                            match serde_json::from_str(&json_slice) {
                                Ok(value) => {
                                    let _ = tx_ev.send(NdJsonData::Data(value)).await;
                                },
                                Err(e) => {
                                    error!("Invalid JSON received from Ollama chat completion: {:?}", e);
                                    error!("Value: {json_slice}");
                                    let _ = tx_ev.send(NdJsonData::End).await;
                                    break 'outer;
                                }
                            }

                            // Keep the remaining data without the newline
                            json_buffer = slice.to_owned();
                        }
                    }
                }
                buffer.clear(); // Clear for the next chunk of data
            }
            let _ = tx_ev.send(NdJsonData::End);
        });

        return obj;
    }

    /// Sender for queueing data received from Ollama
    pub fn sender(&self) -> Sender<NdJsonData<Bytes>> {
        self.tx_data.clone()
    }

    pub fn receiver(&mut self) -> Option<Receiver<NdJsonData<T>>> {
        self.rx_events.take()
    }

    pub fn start_reading_response(&self, mut res: reqwest::Response) {
        let sink = self.sender();
        tokio::spawn(async move {
            while let Ok(chunk) = res.chunk().await {
                if let Some(chunk) = chunk {
                    if let Err(_) = sink.send(NdJsonData::Data(chunk)).await {
                        break;
                    }
                } else {
                    // We don't remove the data from the stream using "chunk".
                    // So if we don't have any more data, we need to break manually
                    break;
                }
            }
            let _ = sink.send(NdJsonData::End).await;
        });
    }

    pub fn start_unwrapping_data(&mut self, buffer_size: usize) -> Result<Receiver<T>, errors::Error> {
        let (sender, receiver) = channel::<T>(buffer_size);
        let mut data_receiver = self.receiver().ok_or(errors::internal("Data is already being unwrapped"))?;
        tokio::spawn(async move {
            while let Some(data) = data_receiver.recv().await {
                if let NdJsonData::Data(data) = data {
                    let _ = sender.send(data).await;
                } else {
                    break;
                }
            }
        });
        Ok(receiver)
    }
}

#[async_trait]
impl PromptResponse for NdJsonReader<ChatResponse> {
    fn get_prompts(&mut self) -> Result<Receiver<ChatResponse>, errors::Error> {
        self.start_unwrapping_data(64)
    }

    async fn abort(&self) {
        let _ = self.tx_data.send(NdJsonData::End).await;
    }
}
