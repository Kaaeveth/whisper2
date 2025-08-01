use async_trait::async_trait;
use core::str;

use bytes::Bytes;
use tokio::sync::mpsc::{Sender, Receiver, channel};

use crate::backend::llm::{PromptEvent, PromptResponse};

pub(crate) enum NdJsonData {
    Data(Bytes),
    End
}

/// Reads raw responses as ND-JSON (like from Ollama) and parses them into
/// a token stream. We expect UTF-8 data (the default using Ollama).
pub(crate) struct NdJsonReader {
    tx_data: Sender<NdJsonData>,
    rx_events: Option<Receiver<PromptEvent>>
}

impl NdJsonReader {
    pub fn new() -> Self {
        const BUFFER_SIZE: usize = 1024;
        let (tx, mut rx) = channel(BUFFER_SIZE);
        let (tx_ev, rx_ev) = channel(256);
        let obj = NdJsonReader {
            tx_data: tx,
            rx_events: Some(rx_ev)
        };

        tokio::spawn(async move {
            let mut buffer: Vec<NdJsonData> = Vec::with_capacity(BUFFER_SIZE);
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
                                    eprintln!("Unexpected Ollama string encoding: {:?}", e);
                                    let _ = tx_ev.send(PromptEvent::Stop).await;
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
                                    let _ = tx_ev.send(PromptEvent::Message(value)).await;
                                },
                                Err(e) => {
                                    eprintln!("Invalid JSON received from Ollama chat completion: {:?}", e);
                                    eprintln!("Value: {json_slice}");
                                    let _ = tx_ev.send(PromptEvent::Stop).await;
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
            let _ = tx_ev.send(PromptEvent::Stop);
        });

        return obj;
    }

    /// Sender for queueing data received from Ollama
    pub fn data_intake(&self) -> Sender<NdJsonData> {
        self.tx_data.clone()
    }
}

#[async_trait]
impl PromptResponse for NdJsonReader {
    fn get_prompts(&mut self) -> Option<Receiver<PromptEvent>> {
        self.rx_events.take()
    }

    async fn abort(&self) {
        let _ = self.tx_data.send(NdJsonData::End).await;
    }
}
