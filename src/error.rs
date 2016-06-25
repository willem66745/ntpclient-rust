use std::error;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnexpectedSize(usize, usize),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedSize(expected_size, size) => {
                write!(f,
                       "Unexpected number of bytes in NTP datagram (expected:{}; actual:{})",
                       expected_size,
                       size)
            }
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnexpectedSize(_, _) => "Unexpected number of bytes in NTP datagram",
            Error::Io(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::UnexpectedSize(_, _) => None,
            Error::Io(ref err) => err.cause(),
        }
    }
}
