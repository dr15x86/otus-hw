use std::net::{SocketAddr, TcpStream};

use clap::{Parser, Subcommand};

use otus_hw::error::Error;
use otus_hw::tcp::{recv_str, send_str, COMMAND_OFF, COMMAND_ON, COMMAND_STATUS, DEFAULT_ADDR};

/// Smart Socket client
#[derive(Parser)]
struct Args {
    /// Server address
    #[clap(short, long, default_value_t = DEFAULT_ADDR.parse().unwrap(), value_parser)]
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
    println!("CLient connect to server: {}", args.address);

    match args.command {
        Commands::On => {
            send_str(COMMAND_ON, &mut stream)?;
        }
        Commands::Off => {
            send_str(COMMAND_OFF, &mut stream)?;
        }
        Commands::Status => {
            send_str(COMMAND_STATUS, &mut stream)?;
            println!("socket status: {}", recv_str(&mut stream)?);
        }
    }

    Ok(())
}
