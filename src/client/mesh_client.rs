// This is a basic mesh client that can be used to connect to a meshtastic device
// Other clients can be built on top of this, such as a bluetooth client, a tcp client, etc.

use std::error::Error;
use std::fmt;

use crate::node::Node;
use crate::ConnectionArgs;

pub mod bluetooth_client;
pub mod serial_client;
pub mod tcp_client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait MeshClient {
    fn get_node(&self) -> &Node;
    fn new(args: &ConnectionArgs) -> Self
    where
        Self: Sized;
    fn connect(&self) -> Result<()>;
    fn send(&self, message: &str) -> Result<()>;
    fn receive(&self) -> Result<()>;
}

#[derive(Debug)]
pub enum MeshClientError {
    ConnectionError,
    SendError,
    ReceiveError,
}

impl fmt::Display for MeshClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeshClientError::ConnectionError => write!(f, "Failed to connect to the device"),
            MeshClientError::SendError => write!(f, "Failed to send message"),
            MeshClientError::ReceiveError => write!(f, "Failed to receive message"),
        }
    }
}

impl Error for MeshClientError {}
