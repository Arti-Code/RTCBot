use std::{sync::Arc, time::Duration};

use bytes::BytesMut;
use futures::FutureExt;
use webrtc::{
    data_channel::{DataChannel, DataChannelEvent}, peer_connection::{PeerConnectionEventHandler, RTCIceGatheringState, RTCPeerConnectionState, RTCSignalingState}, runtime::{Mutex, Receiver, Runtime, Sender, sleep}
};


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
        println!("[SIGNALING STATE]: {}", state.to_string());
    }
    async fn on_ice_gathering_state_change(&self, state: RTCIceGatheringState) {
        match state {
            RTCIceGatheringState::Complete => {
                println!("[ICE GATHERING STATE]: {}", state.to_string());
                let _ = self.gather_complete_tx.try_send(());
            },
            _ => {
                println!("[ICE GATHERING STATE]: {}", state.to_string());
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
                /* let video_track = self.video_track.clone();
                let ssrc = self.ssrc;
                let std_listener = std::net::UdpSocket::bind(VIDEO_LISTENER).unwrap();
                let listener: Arc<dyn AsyncUdpSocket> = self.runtime.wrap_udp_socket(std_listener).unwrap().clone();
                self.runtime.spawn(Box::pin(async move {
                let mut buf = vec![0u8; 1600];
                loop {
                    match listener.recv_from(&mut buf).await {
                        Ok((n, _)) => {
                            let mut bytes = BytesMut::from(&buf[..n]);
                            match rtp::packet::Packet::unmarshal(&mut bytes) {
                                Ok(mut packet) => {
                                    packet.header.ssrc = ssrc;  // Use cloned ssrc
                                    if let Err(_) = video_track.write_rtp(packet).await {
                                        println!("write_rtp error");
                                        break;
                                    }
                                }
                                Err(err) => eprintln!("RTP unmarshal error: {err}"),
                            }
                        }
                        Err(err) => {
                            eprintln!("UDP read error: {err}");
                            break;
                        }
                    }
                }
                })); */
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
        //let message_rx = self.message_rx.clone();
        self.runtime.spawn(Box::pin(async move {
            //let mut opened = false;
            let mut send_timer = Box::pin(sleep(Duration::from_secs(5)));
            //let msg_rx = message_rx.lock().await;
            loop {
                //if opened {
                futures::select! {
                    event = dc.poll().fuse() => {
                        match event {
                            Some(DataChannelEvent::OnMessage(msg)) => {
                                let text = String::from_utf8(msg.data.to_vec()).unwrap_or_default();
                                println!("==> {text}");
                                command_tx.send(text).await.unwrap_or_default();
                            }
                            Some(DataChannelEvent::OnClose) => {
                                println!("{}", "datachannel closed".to_string());
                                let _ = done_tx.try_send(());
                            },
                            Some(DataChannelEvent::OnClosing) => {
                                println!("{}", "datachannel closing".to_string());
                            },
                            Some(DataChannelEvent::OnError) => {
                                println!("{}", "datachannel error".to_string());
                                let _ = done_tx.try_send(());
                            },
                            Some(DataChannelEvent::OnOpen) => {
                                println!("{}", "datachannel open".to_string());
                                //opened = true;
                                //send_timer = Box::pin(sleep(Duration::from_secs(5)));
                            },
                            Some(DataChannelEvent::OnBufferedAmountHigh) => {
                                println!("{}", "datachannel buffer high".to_string());
                            },
                            Some(DataChannelEvent::OnBufferedAmountLow) => {
                                println!("{}", "datachannel buffer low".to_string());
                            },
                            None => {
                                println!("{}", "datachannel none event".to_string());
                                let _ = done_tx.try_send(());
                            }
                        }
                    },
                    _ = send_timer.as_mut().fuse() => {
                        let message = "ping from robot";
                        println!("<== '{message}'");
                        let _ = dc.send(BytesMut::from(message.as_bytes())).await;
                        send_timer = Box::pin(sleep(Duration::from_secs(5)));
                    }
                }
            }
        }));
    }
}