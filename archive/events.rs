use std::{sync::Arc, time::Duration};

use bytes::BytesMut;
use futures::FutureExt;
use webrtc::{
    data_channel::{DataChannel, DataChannelEvent}, 
    peer_connection::{PeerConnectionEventHandler, RTCIceGatheringState, RTCPeerConnectionState, RTCSignalingState}, 
    runtime::{Mutex, Receiver, Runtime, Sender, sleep}
};


/* #[derive(Clone)]
pub struct CameraHandler {
    pub runtime: Arc<dyn Runtime>,
    pub gather_complete_tx: Sender<()>,
    pub connected_tx: Sender<()>,
    pub done_tx: Sender<()>,
}

#[async_trait::async_trait]
impl PeerConnectionEventHandler for CameraHandler {

    async fn on_negotiation_needed(&self) {
        println!("[NEGOTIATION]: needed");
    }

    async fn on_signaling_state_change(&self, state: RTCSignalingState) {
        match state {
            RTCSignalingState::Closed => println!("{}{}", "[SIGNALING STATE]: ".to_string(), state.to_string()),
            RTCSignalingState::HaveLocalOffer => println!("{}{}", "[SIGNALING STATE]: ".to_string(), state.to_string()),
            RTCSignalingState::HaveRemoteOffer => println!("{}{}", "[SIGNALING STATE]: ".to_string(), state.to_string()),
            RTCSignalingState::Stable => println!("{}{}", "[SIGNALING STATE]: ".to_string(), state.to_string()),
            _ => {},
        }
    }

    async fn on_ice_gathering_state_change(&self, state: RTCIceGatheringState) {
        match state {
            RTCIceGatheringState::Complete => {
                println!("{}{}", "[ICE GATHERING STATE]: ".to_string(), state.to_string());
                let _ = self.gather_complete_tx.try_send(());
            },
            _ => {
                println!("{}{}", "[ICE GATHERING STATE]: ".to_string(), state.to_string());
            }
        }
    }

    async fn on_connection_state_change(&self, state: RTCPeerConnectionState) {
        let state_info: String;
        match state {
            RTCPeerConnectionState::Failed => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                let _ = self.done_tx.try_send(());
            },
            RTCPeerConnectionState::Disconnected => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                let _ = self.done_tx.try_send(());
            },
            RTCPeerConnectionState::Connected => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                let _ = self.connected_tx.try_send(());
            },
            RTCPeerConnectionState::Closed => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                let _ = self.done_tx.try_send(());
            },
            RTCPeerConnectionState::New => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                //let _ = self.connected_tx.try_send(());
            },
            RTCPeerConnectionState::Connecting => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                //let _ = self.connected_tx.try_send(());
            },
            RTCPeerConnectionState::Unspecified => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                //let _ = self.connected_tx.try_send(());
            },
        }
        println!("{}", &state_info);
    }

    async fn on_data_channel(&self, dc: Arc<dyn DataChannel>) {
        let done_tx = self.done_tx.clone();
        self.runtime.spawn(Box::pin(async move {
            let mut opened = false;
            let mut send_timer = Box::pin(sleep(Duration::from_secs(5)));
            loop {
                if opened {
                    futures::select! {
                        event = dc.poll().fuse() => {
                            match event {
                                Some(DataChannelEvent::OnMessage(msg)) => {
                                    let text = String::from_utf8(msg.data.to_vec())
                                        .unwrap_or_default();
                                    println!("==> '{text}'");
                                }
                                Some(DataChannelEvent::OnClose) | None => {
                                    let _ = done_tx.try_send(());
                                    break;
                                }
                                _ => {}
                            }
                        }
                        _ = send_timer.as_mut().fuse() => {
                            //let message = generate_description(32);
                            //println!("<== '{message}'");
                            //let _ = dc.send(BytesMut::from(message.as_bytes())).await;
                            //send_timer = Box::pin(sleep(Duration::from_secs(5)));
                        }
                    }
                } else {
                    match dc.poll().await {
                        Some(DataChannelEvent::OnOpen) => {
                            println!("{}", "datachannel open".to_string());
                            opened = true;
                            //send_timer = Box::pin(sleep(Duration::from_secs(5)));
                        }
                        Some(DataChannelEvent::OnClose) => {
                            println!("{}", "datachannel closed".to_string());
                            let _ = done_tx.try_send(());
                            break;
                        },
                        Some(DataChannelEvent::OnClosing) => {
                            println!("{}", "datachannel closing".to_string());
                            let _ = done_tx.try_send(());
                            break;
                        },
                        Some(DataChannelEvent::OnError) => {
                            println!("{}", "datachannel error".to_string());
                            let _ = done_tx.try_send(());
                            break;
                        },
                        Some(_) => {
                            println!("{}", "datachannel other event".to_string());
                        },
                        None => {
                            println!("{}", "datachannel none event".to_string());
                            let _ = done_tx.try_send(());
                            break;
                        }
                    }
                }
            }

            //println!("exit datachannel loop");
        }));
    }
}
 */

#[derive(Clone)]
pub struct RobotHandler {
    pub runtime: Arc<dyn Runtime>,
    pub gather_complete_tx: Sender<()>,
    pub connected_tx: Sender<()>,
    pub done_tx: Sender<()>,
    pub command_tx: Sender<String>,
    pub message_rx: Arc<Mutex<Receiver<String>>>,
}

#[async_trait::async_trait]
impl PeerConnectionEventHandler for RobotHandler {

    async fn on_negotiation_needed(&self) {
        println!("[NEGOTIATION]: needed");
    }

    async fn on_signaling_state_change(&self, state: RTCSignalingState) {
        match state {
            RTCSignalingState::Closed => println!("{}{}", "[SIGNALING STATE]: ".to_string(), state.to_string()),
            RTCSignalingState::HaveLocalOffer => println!("{}{}", "[SIGNALING STATE]: ".to_string(), state.to_string()),
            RTCSignalingState::HaveRemoteOffer => println!("{}{}", "[SIGNALING STATE]: ".to_string(), state.to_string()),
            RTCSignalingState::Stable => println!("{}{}", "[SIGNALING STATE]: ".to_string(), state.to_string()),
            _ => {},
        }
    }
    async fn on_ice_gathering_state_change(&self, state: RTCIceGatheringState) {
        match state {
            RTCIceGatheringState::Complete => {
                println!("{}{}", "[ICE GATHERING STATE]: ".to_string(), state.to_string());
                let _ = self.gather_complete_tx.try_send(());
            },
            _ => {
                println!("{}{}", "[ICE GATHERING STATE]: ".to_string(), state.to_string());
            }
        }
    }

    async fn on_connection_state_change(&self, state: RTCPeerConnectionState) {
        let state_info: String;
        match state {
            RTCPeerConnectionState::Failed => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                let _ = self.done_tx.try_send(());
            },
            RTCPeerConnectionState::Disconnected => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                let _ = self.done_tx.try_send(());
            },
            RTCPeerConnectionState::Connected => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                let _ = self.connected_tx.try_send(());
            },
            RTCPeerConnectionState::Closed => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
                let _ = self.done_tx.try_send(());
            },
            RTCPeerConnectionState::New => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
            },
            RTCPeerConnectionState::Connecting => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
            },
            RTCPeerConnectionState::Unspecified => {
                state_info = format!("{}{}", "[PEER CONNECTION]: ".to_string(), state.to_string());
            },
        }
        println!("{}", &state_info);
    }


    async fn on_data_channel(&self, dc: Arc<dyn DataChannel>) {
        let done_tx = self.done_tx.clone();
        let command_tx = self.command_tx.clone();
        let message_rx = self.message_rx.clone();
        self.runtime.spawn(Box::pin(async move {
            let mut opened = false;
            let mut msg_rx = message_rx.lock().await;
            loop {
                if opened {
                    futures::select! {
                        event = dc.poll().fuse() => {
                            match event {
                                Some(DataChannelEvent::OnMessage(msg)) => {
                                    let text = String::from_utf8(msg.data.to_vec())
                                        .unwrap_or_default();
                                    println!("==> {text}");
                                    command_tx.send(text).await.unwrap_or_default();
                                }
                                Some(DataChannelEvent::OnClose) | None => {
                                    let _ = done_tx.try_send(());
                                    break;
                                }
                                _ => {}
                            }
                        },
                        message = msg_rx.recv().fuse() => {
                            match message {
                                Some(msg) => {
                                    println!("<== {msg}");
                                    let bytes = BytesMut::from(msg.as_bytes());
                                    let _ = dc.send(bytes).await;
                                },
                                None => {
                                    let _ = done_tx.try_send(());
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    match dc.poll().await {
                        Some(DataChannelEvent::OnOpen) => {
                            println!("{}", "datachannel open".to_string());
                            opened = true;
                            //send_timer = Box::pin(sleep(Duration::from_secs(5)));
                        }
                        Some(DataChannelEvent::OnClose) => {
                            println!("{}", "datachannel closed".to_string());
                            let _ = done_tx.try_send(());
                            break;
                        },
                        Some(DataChannelEvent::OnClosing) => {
                            println!("{}", "datachannel closing".to_string());
                            let _ = done_tx.try_send(());
                            break;
                        },
                        Some(DataChannelEvent::OnError) => {
                            println!("{}", "datachannel error".to_string());
                            let _ = done_tx.try_send(());
                            break;
                        },
                        Some(_) => {
                            println!("{}", "datachannel other event".to_string());
                        },
                        None => {
                            println!("{}", "datachannel none event".to_string());
                            let _ = done_tx.try_send(());
                            break;
                        }
                    }
                }
            }

            println!("exit datachannel loop");
        }));
    }
}