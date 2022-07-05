use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Bad utf8 encoding")]
    BadEncoding,

    #[error("Bad command")]
    BadCommand,

    #[error("Bad SocketAddr values")]
    BadSocketAddr,

    #[error("Print usage and exit")]
    PrintUsageAndExit,
}
