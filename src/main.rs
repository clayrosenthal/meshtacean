// A library for the Meshtastic Client API
// Install with cargo: "cargo install meshtacean"

use clap::{ArgAction, Args, Parser, Subcommand};

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
#[group(id = "connection", required = true, multiple = false)]
struct ConnectionArgs {
    #[command(flatten)]
    bluetooth: BluetoothArgs,

    #[command(flatten)]
    tcp: TcpArgs,

    #[command(flatten)]
    serial: SerialArgs,
}

#[derive(Args, Debug)]
#[group(id = "bluetooth", required = false, multiple = false)]
struct BluetoothArgs {
    #[arg(
        short,
        long,
        help = "Connect to a bluetooth device, by address or name"
    )]
    bluetooth: String,

    #[arg(long, help = "Scan for bluetooth devices")]
    scan: bool,
}

#[derive(Args, Debug)]
#[group(id = "serial", required = false, multiple = false)]
struct SerialArgs {
    #[arg(short, long, help = "Port of device to connect to using serial")]
    port: String,
}

#[derive(Args, Debug)]
#[group(id = "tcp", required = false, multiple = false)]
struct TcpArgs {
    #[arg(short, long, help = "Connect to device using TCP")]
    host: String,
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

fn main() {
    let args = Cli::parse();
    if args.verbose > 0 {
        println!("Args: {:?}", args);
    }
    println!("Hello, world!");
}
