use std::{net::SocketAddr, time::Duration};

use clap::{Parser, Subcommand};
use tokio::{net::UdpSocket, time::sleep};

use otus_hw::{
    error::Error,
    network::{
        constants::{
            COMMAND_THERMOMETER_SET_TEMP, COMMAND_THERMOMETER_STATUS, DEFAULT_UDP_CLIENT_ADDR,
            DEFAULT_UDP_SERV_ADDR,
        },
        recv_str_from_udp, send_str_to_udp,
    },
};

/// Smart Socket client
#[derive(Parser)]
struct Args {
    /// Server address
    #[clap(short, long, default_value_t = DEFAULT_UDP_SERV_ADDR.parse().unwrap(), value_parser)]
    address: SocketAddr,

    /// Client address
    #[clap(short, long, default_value_t = DEFAULT_UDP_CLIENT_ADDR.parse().unwrap(), value_parser)]
    client_address: SocketAddr,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get thermometer status
    Status,

    /// Watch thermometer status
    Watch {
        #[clap(short, long, default_value_t = 1, value_parser=clap::value_parser!(u32).range(1..))]
        interval: u32,
    },

    /// Set thermometer temperature
    SetTemperature {
        #[clap(value_parser)]
        temp: i32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let socket = UdpSocket::bind(args.client_address).await?;
    println!("Client bind to address: {}", socket.local_addr()?);

    match args.command {
        Commands::Status => {
            send_str_to_udp(COMMAND_THERMOMETER_STATUS, &socket, args.address).await?;
            println!(
                "thermometer status: {}",
                recv_str_from_udp(&socket).await?.0
            );
        }
        Commands::Watch { interval } => loop {
            send_str_to_udp(COMMAND_THERMOMETER_STATUS, &socket, args.address).await?;
            println!(
                "thermometer status: {}",
                recv_str_from_udp(&socket).await?.0
            );
            sleep(Duration::from_secs(interval.into())).await;
        },
        Commands::SetTemperature { temp } => {
            send_str_to_udp(
                format!("{}{}", COMMAND_THERMOMETER_SET_TEMP, temp),
                &socket,
                args.address,
            )
            .await?;
        }
    }

    Ok(())
}
