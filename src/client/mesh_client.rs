// This is a basic mesh client that can be used to connect to a meshtastic device
// Other clients can be built on top of this, such as a bluetooth client, a tcp client, etc.

use thiserror::Error;

use crate::node::Node;
use crate::ConnectionArgs;

pub mod bluetooth_client;
pub mod serial_client;
pub mod tcp_client;

type Result<T> = std::result::Result<T, MeshClientError>;

pub trait MeshClient {
    fn get_node(&self) -> &Node;
    fn new(args: &ConnectionArgs) -> Self
    where
        Self: Sized;
    fn connect(&self) -> Result<()>;
    fn send(&self, message: &str) -> Result<()>;
    fn receive(&self) -> Result<()>;
}

#[derive(Debug, Error)]
pub enum MeshClientError {
    #[error("Failed to connect to the device")]
    ConnectionError,
    #[error("Failed to send message")]
    SendError,
    #[error("Failed to receive message")]
    ReceiveError,
}
