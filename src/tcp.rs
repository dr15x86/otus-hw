use std::io::{Read, Write};
use std::mem::size_of;

use crate::error::Error;

type SizeType = u64;

pub const DEFAULT_ADDR: &str = "127.0.0.1:12135";

pub const COMMAND_ON: &str = "socket on";
pub const COMMAND_OFF: &str = "socket off";
pub const COMMAND_STATUS: &str = "socket status";

pub fn send_str(str: impl AsRef<str>, writer: &mut impl Write) -> Result<(), Error> {
    let str_bytes = str.as_ref().as_bytes();
    let len_bytes = (str_bytes.len() as SizeType).to_be_bytes();

    writer.write_all(&len_bytes)?;
    writer.write_all(str_bytes)?;

    Ok(())
}

pub fn recv_str(reader: &mut impl Read) -> Result<String, Error> {
    let mut buf = [0; size_of::<SizeType>()];
    reader.read_exact(&mut buf)?;

    let len = SizeType::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf)?;

    String::from_utf8(buf).map_err(|_| Error::BadEncoding)
}
