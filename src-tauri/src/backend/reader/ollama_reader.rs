use core::str;
use std::sync::{atomic::AtomicBool, Arc};

use bytes::Bytes;
use tokio::sync::mpsc::{Sender, Receiver, channel};

use crate::backend::llm::{PromptEvent, PromptResponse};

pub(crate) enum OllamaPromptData {
    Data(Bytes),
    End
}

/// Reads raw responses from Ollama and parses them into
/// a token stream. We expect UTF-8 data from Ollama (the default).
pub(crate) struct OllamaPromptReader {
    tx_data: Option<Sender<OllamaPromptData>>,
    rx_events: Option<Receiver<PromptEvent>>,
    abort: Arc<AtomicBool>
}

impl OllamaPromptReader {
    pub fn new() -> Self {
        let (tx, mut rx) = channel(1024);
        let (tx_ev, rx_ev) = channel(256);
        let obj = OllamaPromptReader {
            tx_data: Some(tx),
            rx_events: Some(rx_ev),
            abort: Arc::new(AtomicBool::new(false))
        };

        let abort = obj.abort.clone();
        tokio::spawn(async move {
            const BUFFER_SIZE: usize = 1024;
            let mut buffer: Vec<OllamaPromptData> = Vec::with_capacity(BUFFER_SIZE);
            let mut json_buffer = String::new();

            'outer: loop {
                if rx.is_closed() || abort.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }

                // Buffer data until we have a complete JSON object
                // We send the object right after parsing it and clear the JSON buffer
                let _ = rx.recv_many(&mut buffer, BUFFER_SIZE).await;
                for data in &buffer {
                    match data {
                        OllamaPromptData::End => break 'outer,
                        OllamaPromptData::Data(data) => {
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
    pub fn data_intake(&self) -> Sender<OllamaPromptData> {
        self.tx_data.as_ref().unwrap().clone()
    }
}

impl PromptResponse for OllamaPromptReader {
    fn get_prompts(&mut self) -> Option<Receiver<PromptEvent>> {
        self.rx_events.take()
    }

    fn abort(&self) {
        self.abort.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
