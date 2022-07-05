use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;

use clap::Parser;

use otus_hw::devices::thermometer::Thermometer;
use otus_hw::devices::Device;
use otus_hw::error::Error;
use otus_hw::network::constants::{
    COMMAND_THERMOMETER_SET_TEMP, COMMAND_THERMOMETER_STATUS, DEFAULT_UDP_SERV_ADDR,
};
use otus_hw::network::{recv_str_from_udp, send_str_to_udp};

/// Smart Thermometer server
#[derive(Parser)]
struct Args {
    /// Binding server address
    #[clap(default_value_t = DEFAULT_UDP_SERV_ADDR.parse().unwrap(), value_parser)]
    addr: SocketAddr,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let thermometer = Arc::new(Mutex::new(Thermometer::default()));
    let socket = Arc::new(UdpSocket::bind(args.addr)?);

    println!("Server running on {}", socket.local_addr()?);

    loop {
        let (cmd, peer_addr) = recv_str_from_udp(&socket)?;

        let thermometer = Arc::clone(&thermometer);
        let socket = Arc::clone(&socket);

        thread::spawn(move || -> Result<(), Error> {
            println!("{peer_addr}: new client connected");
            match cmd.as_str() {
                COMMAND_THERMOMETER_STATUS => {
                    let status = thermometer
                        .lock()
                        .unwrap()
                        .info()
                        .unwrap_or_else(|err| err.into());

                    send_str_to_udp(status, &socket, peer_addr).map_err(|err| {
                        eprintln!("{peer_addr}: send_str_to_udp error: {err}");
                        err
                    })?;
                }
                set_temp_cmd if set_temp_cmd.starts_with(COMMAND_THERMOMETER_SET_TEMP) => {
                    let new_temp = parse_set_temp_cmd(set_temp_cmd)?;
                    thermometer.lock().unwrap().set_temperature(new_temp);
                }
                unknown => {
                    eprintln!("{peer_addr}: unknown command {unknown}");
                }
            }

            Ok(())
        });
    }
}

fn parse_set_temp_cmd(cmd: &str) -> Result<f32, Error> {
    let cmd = &cmd[COMMAND_THERMOMETER_SET_TEMP.len()..];
    let result = cmd.parse().map_err(|_| Error::BadCommand)?;
    Ok(result)
}
