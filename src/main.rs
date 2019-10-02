#![deny(rust_2018_idioms)]

use std::str::FromStr;

use structopt::StructOpt;
use tokio::runtime::Runtime;

use crate::{
    error::KikiError,
    protocol::Connection
};
use std::net::SocketAddr;

mod error;
mod protocol;

#[derive(Debug, StructOpt)]
struct App {
    #[structopt(subcommand)]
    mode: Mode,
    #[structopt(long)]
    address: SocketAddr,
    #[structopt(default_value = "tcp")]
    protocol: Protocol,
}

#[derive(Debug, StructOpt)]
enum Mode {
    Send { message: String },
    Listen,
}

#[derive(Debug, StructOpt)]
pub enum Protocol {
    Tcp,
    Udp,
}

impl FromStr for Protocol {
    type Err = KikiError;
    fn from_str(protocol: &str) -> Result<Self, Self::Err> {
        match protocol {
            "tcp" => Ok(Protocol::Tcp),
            "udp" => Ok(Protocol::Udp),
            _ => Err(KikiError::UnknownProtocol(protocol.to_string())),
        }
    }
}

fn main() -> Result<(), KikiError> {
    let app: App = App::from_args();

    let connection: &dyn Connection = match app.protocol {
        Protocol::Tcp => &protocol::tcp::Tcp,
        Protocol::Udp => &protocol::udp::Udp,
    };

    let runtime = Runtime::new().unwrap();
    match app.mode {
        Mode::Send { message } => {
            let future = connection.send(&app.address, &message);
            runtime.block_on(future)
        },
        Mode::Listen => {
            let future = connection.listen(&app.address);
            runtime.block_on(future)
        }
    }?;

    Ok(())
}
