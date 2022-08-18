use std::{io::Cursor, mem::size_of, net::SocketAddr};

use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::UdpSocket,
};

use crate::error::{Error, Result};

type SizeType = u64;

// https://ru.wikipedia.org/wiki/UDP#Длина_датаграммы
const MAX_DATAGRAM_SIZE: usize = 0xffff;

pub mod constants {
    pub const DEFAULT_TCP_ADDR: &str = "127.0.0.1:12135";
    pub const DEFAULT_UDP_SERV_ADDR: &str = "127.0.0.1:12135";
    pub const DEFAULT_UDP_CLIENT_ADDR: &str = "127.0.0.1:0";

    pub const COMMAND_SOCKET_ON: &str = "socket on";
    pub const COMMAND_SOCKET_OFF: &str = "socket off";
    pub const COMMAND_SOCKET_STATUS: &str = "socket status";

    pub const COMMAND_THERMOMETER_SET_TEMP: &str = "thermometer set temp: ";
    pub const COMMAND_THERMOMETER_STATUS: &str = "thermometer status";
}

pub async fn send_str(str: impl AsRef<str>, writer: &mut (impl AsyncWrite + Unpin)) -> Result<()> {
    let str_bytes = str.as_ref().as_bytes();
    let len_bytes = (str_bytes.len() as SizeType).to_be_bytes();

    writer.write_all(&len_bytes).await?;
    writer.write_all(str_bytes).await?;

    Ok(())
}

pub async fn recv_str(reader: &mut (impl AsyncRead + Unpin)) -> Result<String> {
    let mut buf = [0; size_of::<SizeType>()];
    reader.read_exact(&mut buf).await?;

    let len = SizeType::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf).await?;

    String::from_utf8(buf).map_err(|_| Error::BadEncoding)
}

pub async fn send_str_to_udp(
    str: impl AsRef<str>,
    socket: &UdpSocket,
    addr: SocketAddr,
) -> Result<()> {
    let mut vec = Vec::with_capacity(size_of::<SizeType>() + str.as_ref().len());
    let mut cursor = Cursor::new(&mut vec);

    send_str(str, &mut cursor).await?;
    socket.send_to(&vec, addr).await?;

    Ok(())
}

pub async fn recv_str_from_udp(socket: &UdpSocket) -> Result<(String, SocketAddr)> {
    let mut buf = [0; MAX_DATAGRAM_SIZE];
    let (number_of_bytes, peer_addr) = socket.recv_from(&mut buf).await?;
    let mut cursor = Cursor::new(&buf[..number_of_bytes]);
    let result_str = recv_str(&mut cursor).await?;

    Ok((result_str, peer_addr))
}
