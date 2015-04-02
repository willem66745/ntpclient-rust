use std::error;
use std::io;
use std::fmt;
use byteorder;

#[derive(Debug)]
pub enum Error {
    UnexpectedSize(usize, usize),
    Io(io::Error),
    Byteorder(byteorder::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::Byteorder(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedSize(expected_size, size) =>
                write!(f, "Unexpected number of bytes in NTP datagram (expected:{}; actual:{})", expected_size, size),
            Error::Io(ref err) => err.fmt(f),
            Error::Byteorder(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnexpectedSize(_, _) => "Unexpected number of bytes in NTP datagram",
            Error::Io(ref err) => error::Error::description(err),
            Error::Byteorder(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::UnexpectedSize(_, _) => None,
            Error::Io(ref err) => err.cause(),
            Error::Byteorder(ref err) => err.cause(),
        }
    }
}
