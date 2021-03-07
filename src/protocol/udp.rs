use tokio::net::UdpSocket;

use async_trait::async_trait;

use crate::{error::KikiError, protocol::Connection};
use std::net::SocketAddr;

pub struct Udp;

#[async_trait]
impl Connection for Udp {
    async fn listen(&self, address: &SocketAddr) -> Result<(), KikiError> {
        let listener = UdpSocket::bind(address)
            .await
            .map_err(|_| KikiError::AddressConnectionError(address.clone()))?;

        loop {
            let mut buffer = vec![0; 1024];
            match listener.recv_from(&mut buffer).await.expect("Read") {
                (0, _) => break,
                (n, _) => n,
            };
            let message = String::from_utf8(buffer).expect("Parse");
            println!("{}", message);
        }

        Ok(())
    }

    async fn send(&self, address: &SocketAddr, message: &str) -> Result<(), KikiError> {
        let local_address: SocketAddr = if address.is_ipv4() {
            "0.0.0.0:0"
        } else {
            "[::]:0"
        }
        .parse()
        .expect("Parse");

        let socket = UdpSocket::bind(&local_address)
            .await
            .map_err(|_| KikiError::AddressConnectionError(address.clone()))?;
        socket
            .send_to(message.as_bytes(), address)
            .await
            .expect("Write");
        Ok(())
    }
}
