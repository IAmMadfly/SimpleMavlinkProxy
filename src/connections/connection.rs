use std::{
    error::Error,
    sync
};

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
    fn start(address_info: &str) -> std::io::Result<sync::Arc<sync::Mutex<Self>>>;

    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize>;

    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize>;
}

