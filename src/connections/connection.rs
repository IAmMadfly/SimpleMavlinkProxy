use std::{sync, thread};

pub trait Connection {
    fn start(address_info: String) -> std::io::Result<sync::Arc<sync::Mutex<Self>>>;

    fn run_threaded(&mut self, read_channel: sync::mpsc::Sender<u8>, send_channel: sync::mpsc::Receiver<u8>);

    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize>;

    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize>;
}

