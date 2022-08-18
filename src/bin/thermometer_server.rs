use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use clap::Parser;
use tokio::net::UdpSocket;

use otus_hw::{
    devices::{thermometer::Thermometer, Device},
    error::{Error, Result},
    network::{
        constants::{
            COMMAND_THERMOMETER_SET_TEMP, COMMAND_THERMOMETER_STATUS, DEFAULT_UDP_SERV_ADDR,
        },
        recv_str_from_udp, send_str_to_udp,
    },
};

/// Smart Thermometer server
#[derive(Parser)]
struct Args {
    /// Binding server address
    #[clap(default_value_t = DEFAULT_UDP_SERV_ADDR.parse().unwrap(), value_parser)]
    addr: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let thermometer = Arc::new(Mutex::new(Thermometer::default()));
    let socket = Arc::new(UdpSocket::bind(args.addr).await?);

    println!("Server running on {}", socket.local_addr()?);

    loop {
        let (cmd, peer_addr) = recv_str_from_udp(&socket).await?;

        let thermometer = Arc::clone(&thermometer);
        let socket = Arc::clone(&socket);

        tokio::spawn(async move {
            println!("{peer_addr}: new client connected");
            match cmd.as_str() {
                COMMAND_THERMOMETER_STATUS => {
                    let status = thermometer
                        .lock()
                        .unwrap()
                        .info()
                        .unwrap_or_else(|err| err.into());

                    send_str_to_udp(status, &socket, peer_addr)
                        .await
                        .map_err(|err| {
                            eprintln!("{peer_addr}: send_str_to_udp error: {err}");
                            err
                        })?;
                }
                set_temp_cmd if set_temp_cmd.starts_with(COMMAND_THERMOMETER_SET_TEMP) => {
                    let new_temp = parse_set_temp_cmd(set_temp_cmd).map_err(|err| {
                        eprintln!("{peer_addr}: bad command: '{set_temp_cmd}'");
                        err
                    })?;
                    thermometer.lock().unwrap().set_temperature(new_temp);
                }
                unknown => {
                    eprintln!("{peer_addr}: unknown command {unknown}");
                }
            };

            Ok::<(), Error>(())
        });
    }
}

fn parse_set_temp_cmd(cmd: &str) -> Result<f32> {
    let cmd = &cmd[COMMAND_THERMOMETER_SET_TEMP.len()..];
    let result = cmd.parse().map_err(|_| Error::BadCommand)?;
    Ok(result)
}
