// This is a Client that communicates with a meshtastic device over TCP

use std::error::Error;
use std::fmt;

use crate::client::{MeshClient, MeshClientError};
use crate::node::Node;
use crate::utils;
use crate::ConnectionArgs;

const DEFAULT_TCP_PORT: u16 = 4403;
const DEFAULT_TCP_HOSTNAME: &str = "localhost";

pub struct TcpClient {
    node: Option<Node>,
    hostname: String,
    port: u16,
    socket: Option<std::net::TcpStream>,
}

impl MeshClient for TcpClient {
    fn new(args: &ConnectionArgs) -> Self {
        let hostname = args
            .tcp
            .host
            .clone()
            .unwrap_or(DEFAULT_TCP_HOSTNAME.to_string());
        let port = args.tcp.port.unwrap_or(DEFAULT_TCP_PORT);
        TcpClient {
            node: None,
            hostname,
            port,
            socket: None,
        }
    }

    fn get_node(&self) -> &Node {
        self.node.as_ref().unwrap()
    }

    fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        utils::todo()
    }

    fn send(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        utils::todo()
    }

    fn receive(&self) -> Result<(), Box<dyn std::error::Error>> {
        utils::todo()
    }
}

impl Default for TcpClient {
    fn default() -> Self {
        TcpClient {
            node: None,
            hostname: DEFAULT_TCP_HOSTNAME.to_string(),
            port: DEFAULT_TCP_PORT,
            socket: None,
        }
    }
}
