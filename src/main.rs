// A library for the Meshtastic Client API
// Install with cargo: "cargo install meshtacean"

use clap::{ArgAction, Args, Parser, Subcommand};
use std::error::Error;

#[path = "client/mesh_client.rs"]
pub mod client;

pub mod node;

pub mod utils;

pub mod protobufs {
    include!("protobufs/meshtastic.rs");
    // include!("protobufs/nanopb.rs");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    #[command(flatten)]
    connection: ConnectionArgs,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct ConnectionArgs {
    #[command(flatten)]
    bluetooth: BluetoothArgs,

    #[command(flatten)]
    tcp: TcpArgs,

    #[command(flatten)]
    serial: SerialArgs,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct BluetoothArgs {
    #[arg(
        short,
        long,
        help = "Connect to a bluetooth device, by address or name"
    )]
    bluetooth: Option<String>,

    #[arg(long, help = "Scan for bluetooth devices")]
    scan: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct SerialArgs {
    #[arg(long, help = "Port of device to connect to using serial")]
    port: Option<String>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct TcpArgs {
    #[arg(long, help = "Connect to device using TCP")]
    host: Option<String>,

    #[arg(long, help = "Port of device to connect to using TCP")]
    port: Option<u16>,
}

#[derive(Args, Debug)]
struct ConfigArgs {
    #[arg(short, long, help = "Get a config value", value_parser = valid_config_key)]
    get: Option<String>,

    #[arg(short, long, help = "Set a config value", value_parser = valid_config_key)]
    set: Option<String>,
}

#[derive(Args, Debug)]
struct LocalArgs {
    #[arg(short, long, help = "Get info about the local device")]
    info: bool,

    #[arg(short, long, help = "Get a list of nodes")]
    nodes: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Connect(ConnectionArgs),
    Config(ConfigArgs),
    Local(LocalArgs),
    // Channel(ChannelArgs),
    // Log(LogArgs),
    // Power(PowerArgs),
    // Admin(AdminArgs),
    // Tunnel(TunnelArgs),
}

fn valid_config_key(s: &str) -> Result<String, String> {
    //TODO validate config key
    Ok(s.to_string())
}

fn connect(args: &ConnectionArgs) -> Result<Box<dyn client::MeshClient>, Box<dyn Error>> {
    if args.bluetooth.scan {
        //TODO scan for bluetooth devices
        utils::todo()?;
    } else if let Some(_bluetooth) = &args.bluetooth.bluetooth {
        //TODO connect to bluetooth device
        // let client = client::bluetooth_client::BluetoothClient::new(bluetooth);

        utils::todo()?;
    }

    if let Some(_port) = &args.serial.port {
        //TODO connect to serial device
        utils::todo()?;
    }

    if let Some(_host) = &args.tcp.host {
        //TODO connect to tcp device
        utils::todo()?;
    }

    // No connection method provided, try localhost
    let client = client::tcp_client::TcpClient::default();
    Ok(Box::new(client))
}

fn main() {
    let args = Cli::parse();
    if args.verbose > 0 {
        println!("Args: {:?}", args);
    }
    println!("Hello, world!");
}
