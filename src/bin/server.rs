use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;

use clap::Parser;

use otus_hw::devices::socket::{Socket as SmartSocket, SocketState as SmartSocketState};
use otus_hw::devices::Device;
use otus_hw::error::Error;
use otus_hw::tcp::{recv_str, send_str, COMMAND_OFF, COMMAND_ON, COMMAND_STATUS, DEFAULT_ADDR};

/// Smart Socket server
#[derive(Parser)]
struct Args {
    /// Binding server address
    #[clap(default_value_t = DEFAULT_ADDR.parse().unwrap(), value_parser)]
    addr: SocketAddr,
}

fn main() -> Result<(), Error> {
    let socket = Arc::new(Mutex::new(SmartSocket::default()));

    let args = Args::parse();

    let listener = TcpListener::bind(args.addr)?;

    println!("Server running on {}", listener.local_addr()?);

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Can't establish connection: {e}");
                continue;
            }
        };

        let smart_socket = Arc::clone(&socket);

        thread::spawn(move || -> Result<(), Error> {
            let peer_addr = match stream.peer_addr() {
                Ok(addr) => addr.to_string(),
                Err(_) => "unknown".to_string(),
            };

            println!("{peer_addr}: new client connected");

            let cmd = recv_str(&mut stream).map_err(|err| {
                eprintln!("{peer_addr}: recv_str error: {err}");
                err
            })?;

            match cmd.as_str() {
                COMMAND_ON => {
                    smart_socket.lock().unwrap().set_state(SmartSocketState::On);
                }
                COMMAND_OFF => {
                    smart_socket
                        .lock()
                        .unwrap()
                        .set_state(SmartSocketState::Off);
                }
                COMMAND_STATUS => {
                    let status = smart_socket
                        .lock()
                        .unwrap()
                        .info()
                        .unwrap_or_else(|err| err.into());

                    send_str(status, &mut stream).map_err(|err| {
                        eprintln!("{peer_addr}: send_str error: {err}");
                        err
                    })?;
                }
                unknown => {
                    eprintln!("{peer_addr}: unknown command {unknown}");
                }
            }
            Ok(())
        });
    }

    Ok(())
}
