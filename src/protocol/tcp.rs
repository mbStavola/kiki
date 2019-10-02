use tokio::{
    net::{TcpListener, TcpStream},
    prelude::*
};

use async_trait::async_trait;

use crate::{
    error::KikiError,
    protocol::Connection
};
use std::net::SocketAddr;

pub struct Tcp;

#[async_trait]
impl Connection for Tcp {
    async fn listen(&self, address: &SocketAddr) -> Result<(), KikiError> {
        let mut listener = TcpListener::bind(address).map_err(|_| {
            KikiError::AddressConnectionError(address.clone())
        })?;

        loop {
            let (mut socket, _) = listener.accept().await.expect("Accept");
            let mut buffer = String::new();
            match socket.read_to_string(&mut buffer).await.expect("Read") {
                0 => break,
                n => n,
            };
            println!("{}", &buffer);
        }

        Ok(())
    }

    async fn send(&self, address: &SocketAddr, message: &str) -> Result<(), KikiError> {
        let mut stream = TcpStream::connect(address).await.map_err(|_| {
            KikiError::AddressConnectionError(address.clone())
        })?;
        stream.write_all(message.as_bytes()).await.expect("Write");
        Ok(())
    }
}
