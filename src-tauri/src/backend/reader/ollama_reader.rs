use core::str;

use bytes::Bytes;
use tokio::sync::mpsc::{Sender, Receiver, channel};

use crate::backend::{chat::ChatResponse, llm::{PromptEvent, PromptResponse}};

pub enum PromptData {
    Data(Bytes),
    End
}

pub struct PromptReader {
    tx_data: Option<Sender<PromptData>>,
    tx_events: Option<Sender<PromptEvent>>,
    rx_events: Receiver<PromptEvent>
}

impl PromptReader {
    pub fn new() -> Self {
        let (tx, mut rx) = channel(1024);
        let (tx_ev, rx_ev) = channel(256);
        let obj = PromptReader {
            tx_data: Some(tx),
            tx_events: Some(tx_ev.clone()),
            rx_events: rx_ev
        };

        tokio::spawn(async move {
            let mut buffer: Vec<PromptData> = Vec::new();
            let mut json_buffer = String::new();

            'outer: loop {
                if rx.is_closed() {
                    break;
                }

                // Buffer data until we have a complete JSON object
                // We send the object right after parsing it and clear the JSON buffer
                let _ = rx.recv_many(&mut buffer, 512).await;
                for data in &buffer {
                    match data {
                        PromptData::End => break 'outer,
                        PromptData::Data(data) => {
                            // Search for newline
                            let delimiter_idx = data.iter().position(|b| *b == b'\n');
                            if let Some(delimiter_idx) = delimiter_idx {
                                let slice = data.slice(0..delimiter_idx);
                                json_buffer.push_str(str::from_utf8(&slice).unwrap());

                                // We have a full JSON object
                                let value: ChatResponse = serde_json::from_str(&json_buffer).unwrap();
                                let _ = tx_ev.send(PromptEvent::Message(value)).await;

                                // Append the remaining data
                                json_buffer.clear();
                                let slice = data.slice(delimiter_idx..);
                                json_buffer.push_str(str::from_utf8(&slice).unwrap());
                            } else {
                                json_buffer.push_str(str::from_utf8(&data).unwrap());
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

    pub fn data_intake(&self) -> Sender<PromptData> {
        self.tx_data.as_ref().unwrap().clone()
    }
}

impl PromptResponse for PromptReader {
    fn get_prompts(&self) -> &Receiver<PromptEvent> {
        &self.rx_events
    }

    fn get_control(&mut self) -> Sender<PromptEvent> {
        self.tx_events.take().unwrap()
    }
}
