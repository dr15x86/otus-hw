use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use clap::Parser;
use tokio::net::TcpListener;

use otus_hw::{
    devices::{
        socket::{Socket as SmartSocket, SocketState as SmartSocketState},
        Device,
    },
    error::Error,
    network::{
        constants::{
            COMMAND_SOCKET_OFF, COMMAND_SOCKET_ON, COMMAND_SOCKET_STATUS, DEFAULT_TCP_ADDR,
        },
        recv_str, send_str,
    },
};

/// Smart Socket server
#[derive(Parser)]
struct Args {
    /// Binding server address
    #[clap(default_value_t = DEFAULT_TCP_ADDR.parse().unwrap(), value_parser)]
    addr: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let smart_socket = Arc::new(Mutex::new(SmartSocket::default()));

    let args = Args::parse();

    let listener = TcpListener::bind(args.addr).await?;

    println!("Server running on {}", listener.local_addr()?);

    loop {
        let (mut net_socket, peer_addr) = match listener.accept().await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Can't establish connection: {e}");
                continue;
            }
        };

        let smart_socket = Arc::clone(&smart_socket);

        tokio::spawn(async move {
            println!("{peer_addr}: new client connected");

            let cmd = recv_str(&mut net_socket).await.map_err(|err| {
                eprintln!("{peer_addr}: recv_str error: {err}");
                err
            })?;

            match cmd.as_str() {
                COMMAND_SOCKET_ON => {
                    smart_socket.lock().unwrap().set_state(SmartSocketState::On);
                }
                COMMAND_SOCKET_OFF => {
                    smart_socket
                        .lock()
                        .unwrap()
                        .set_state(SmartSocketState::Off);
                }
                COMMAND_SOCKET_STATUS => {
                    let status = smart_socket
                        .lock()
                        .unwrap()
                        .info()
                        .unwrap_or_else(|err| err.into());

                    send_str(status, &mut net_socket).await.map_err(|err| {
                        eprintln!("{peer_addr}: send_str error: {err}");
                        err
                    })?;
                }
                unknown => {
                    eprintln!("{peer_addr}: unknown command {unknown}");
                }
            }

            Ok::<(), Error>(())
        });
    }
}
