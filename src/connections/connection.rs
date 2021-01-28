use std::error::Error;
use std::fmt;
use std::io::Read;

#[derive(Debug)]
pub struct ConnectionError {}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connection Error occured!")
    }
}

impl Error for ConnectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl ConnectionError {
    pub fn new() -> ConnectionError {
        ConnectionError{}
    }
}

pub trait Connection {
    fn start(address_info: &str) -> Result<Box<Self>, ConnectionError>;

    fn write(&self);

    fn read(&self);
}

