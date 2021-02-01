use std::{
    sync
};

pub trait Connection {
    fn start(address_info: String) -> std::io::Result<sync::Arc<sync::Mutex<Self>>>;

    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize>;

    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize>;
}

