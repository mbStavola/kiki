use async_trait::async_trait;

use crate::error::KikiError;
use std::net::SocketAddr;

pub mod tcp;
pub mod udp;

#[async_trait]
pub trait Connection {
    async fn listen(&self, address: &SocketAddr) -> Result<(), KikiError>;
    async fn send(&self, address: &SocketAddr, message: &str) -> Result<(), KikiError>;
}
