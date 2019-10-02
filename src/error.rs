use failure::Fail;
use std::net::SocketAddr;

#[derive(Debug, Fail)]
pub enum KikiError {
    #[fail(display = "Unsupported protocol '{}'", _0)]
    UnknownProtocol(String),
    #[fail(display = "Could not connect to '{}'", _0)]
    AddressConnectionError(SocketAddr),
}
