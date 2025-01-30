// This is a Client that communicates with a meshtastic device over serial port

use crate::client::{MeshClient, MeshClientError};
use crate::node::Node;
use crate::ConnectionArgs;
use tokio_serial::{available_ports, SerialPort, SerialStream};

const DEFAULT_SERIAL_PORT: &str = "/dev/ttyUSB0";

/// Client for serial communication with a meshtastic device
pub struct SerialClient {
    node: Option<Node>,
    port: String,
    device: Option<Box<dyn SerialPort>>,
}

impl SerialClient {
    /// Scan for available serial ports
    fn scan_ports(&self) -> Result<(), MeshClientError> {
        match available_ports() {
            Ok(ports) => {
                println!("Available ports:");
                for port in ports {
                    println!("{}", port.port_name);
                }
                Ok(())
            }
            Err(e) => {
                println!("Error scanning ports: {}", e);
                Err(MeshClientError::ConnectionError)
            }
        }
    }
}

impl MeshClient for SerialClient {
    fn new(args: &ConnectionArgs) -> Self {
        let port = args
            .serial
            .serial
            .clone()
            .unwrap_or(DEFAULT_SERIAL_PORT.to_string());
        SerialClient {
            node: None,
            port,
            device: None,
        }
    }

    fn get_node(&self) -> &Node {
        self.node.as_ref().unwrap()
    }

    fn connect(&self) -> Result<(), MeshClientError> {
        self.scan_ports()?;
        // let device = SerialStream::open(&self.port)?;
        // self.device = Some(Box::new(device));
        println!("Connecting to {}", self.port);

        Ok(())
    }

    fn send(&self, message: &str) -> Result<(), MeshClientError> {
        println!("Sending message: {}", message);
        Err(MeshClientError::SendError)
    }

    fn receive(&self) -> Result<(), MeshClientError> {
        Err(MeshClientError::ReceiveError)
    }
}

impl Default for SerialClient {
    fn default() -> Self {
        SerialClient {
            node: None,
            port: DEFAULT_SERIAL_PORT.to_string(),
            device: None,
        }
    }
}
