use std::sync::Arc;
use tokio::net::{TcpListener, UdpSocket};
use tracing::{error, info};

/// Background task that spawns TR-143 UDP & TCP Echo reflection servers
pub async fn start_tr143_echo_server(udp_port: u16, tcp_port: u16) {
    // 1. Start UDP Echo Reflection Daemon
    tokio::spawn(async move {
        let addr = format!("0.0.0.0:{}", udp_port);
        match UdpSocket::bind(&addr).await {
            Ok(socket) => {
                info!("TR-143 UDP Echo server listening on {}", addr);
                let socket = Arc::new(socket);
                let mut buf = vec![0u8; 65535];

                loop {
                    match socket.recv_from(&mut buf).await {
                        Ok((len, peer)) => {
                            let s = socket.clone();
                            let data = buf[..len].to_vec();
                            tokio::spawn(async move {
                                let _ = s.send_to(&data, peer).await;
                            });
                        }
                        Err(e) => {
                            error!("TR-143 UDP Echo receive error: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to bind TR-143 UDP Echo socket on {}: {:?}", addr, e);
            }
        }
    });

    // 2. Start TCP Echo Reflection Daemon
    tokio::spawn(async move {
        let addr = format!("0.0.0.0:{}", tcp_port);
        match TcpListener::bind(&addr).await {
            Ok(listener) => {
                info!("TR-143 TCP Echo server listening on {}", addr);
                loop {
                    if let Ok((mut socket, _)) = listener.accept().await {
                        tokio::spawn(async move {
                            let (mut reader, mut writer) = socket.split();
                            let _ = tokio::io::copy(&mut reader, &mut writer).await;
                        });
                    }
                }
            }
            Err(e) => {
                error!("Failed to bind TR-143 TCP Echo socket on {}: {:?}", addr, e);
            }
        }
    });
}
