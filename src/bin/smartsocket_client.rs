use std::net::{SocketAddr, TcpStream};

use clap::{Parser, Subcommand};

use otus_hw::error::Error;
use otus_hw::network::constants::{
    COMMAND_SOCKET_OFF, COMMAND_SOCKET_ON, COMMAND_SOCKET_STATUS, DEFAULT_TCP_ADDR,
};
use otus_hw::network::{recv_str, send_str};

/// Smart Socket client
#[derive(Parser)]
struct Args {
    /// Server address
    #[clap(short, long, default_value_t = DEFAULT_TCP_ADDR.parse().unwrap(), value_parser)]
    address: SocketAddr,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get smart socket status
    Status,
    /// Turn on smart socket
    On,
    /// Turn off smart socket
    Off,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let mut stream = TcpStream::connect(args.address)?;
    println!("Client connect to server: {}", args.address);

    match args.command {
        Commands::On => {
            send_str(COMMAND_SOCKET_ON, &mut stream)?;
        }
        Commands::Off => {
            send_str(COMMAND_SOCKET_OFF, &mut stream)?;
        }
        Commands::Status => {
            send_str(COMMAND_SOCKET_STATUS, &mut stream)?;
            println!("socket status: {}", recv_str(&mut stream)?);
        }
    }

    Ok(())
}
