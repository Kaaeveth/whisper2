use core::str;

use bytes::Bytes;
use tokio::sync::mpsc::{Sender, Receiver, channel};

use crate::backend::llm::{PromptEvent, PromptResponse};

pub(crate) enum OllamaPromptData {
    Data(Bytes),
    End
}

pub(crate) struct OllamaPromptReader {
    tx_data: Option<Sender<OllamaPromptData>>,
    tx_events: Option<Sender<PromptEvent>>,
    rx_events: Receiver<PromptEvent>
}

impl OllamaPromptReader {
    pub fn new() -> Self {
        let (tx, mut rx) = channel(1024);
        let (tx_ev, rx_ev) = channel(256);
        let obj = OllamaPromptReader {
            tx_data: Some(tx),
            tx_events: Some(tx_ev.clone()),
            rx_events: rx_ev
        };

        tokio::spawn(async move {
            let mut buffer: Vec<OllamaPromptData> = Vec::with_capacity(1024);
            let mut json_buffer = String::new();

            'outer: loop {
                if rx.is_closed() {
                    break;
                }

                // Buffer data until we have a complete JSON object
                // We send the object right after parsing it and clear the JSON buffer
                let _ = rx.recv_many(&mut buffer, 1024).await;
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
                            let delimiter_idx = json_buffer.chars().into_iter().position(|b| b == '\n');
                            let Some(delimiter_idx) = delimiter_idx else {
                                continue;
                            };

                            // We have a full JSON object
                            let json_slice = &json_buffer[0..delimiter_idx];
                            match serde_json::from_str(&json_slice) {
                                Ok(value) => {
                                    let _ = tx_ev.send(PromptEvent::Message(value)).await;
                                },
                                Err(e) => {
                                    eprintln!("Invalid JSON received from Ollama chat completion: {:?}", e);
                                    let _ = tx_ev.send(PromptEvent::Stop).await;
                                    break 'outer;
                                }
                            }

                            // Keep the remaining data without the newline
                            if delimiter_idx < json_buffer.len()-1 {
                                let slice = json_buffer[delimiter_idx+1..].to_owned();
                                json_buffer = slice;
                            } else {
                                json_buffer.clear();
                            }
                        }
                    }
                }
                buffer.clear(); // Clear for the next chunk of data
            }
            let _ = tx_ev.send(PromptEvent::Stop);
        });

        return obj;
    }

    pub fn data_intake(&self) -> Sender<OllamaPromptData> {
        self.tx_data.as_ref().unwrap().clone()
    }
}

impl PromptResponse for OllamaPromptReader {
    fn get_prompts(&self) -> &Receiver<PromptEvent> {
        &self.rx_events
    }

    fn get_control(&mut self) -> Sender<PromptEvent> {
        self.tx_events.take().unwrap()
    }
}
