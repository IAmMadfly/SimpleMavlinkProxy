use std::{io::{Read, Write}, net, thread};

use net::{TcpListener, TcpStream};

use super::connection;

struct TcpClient {
    socket:         net::TcpStream
}

struct TcpServer {
    listener:       net::TcpListener,
    client:         net::TcpStream
}

impl connection::Connection for TcpClient {

    fn start(address_info: &str) -> std::io::Result<Box<Self>> {
        let stream_result = TcpStream::connect(address_info);

        match stream_result {
            Ok(tcp_stream) => {
                return Ok(Box::new(
                    TcpClient {
                        socket: tcp_stream
                    }
                ));
            },
            Err(er) => {
                return Err(er);
            }
        }
    }

    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.socket.read(buffer)
    }

    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.socket.write(buffer)
    }

}

impl connection::Connection for TcpServer {

    fn start(address_info: &str) -> std::io::Result<Box<Self>> {
        let listener = TcpListener::bind(address_info)?;

        thread::spawn(move || {
            match listener.accept() {
                Ok((sock, addr)) => {
                    self.client = sock;
                },
                Err(er) => {

                }
            }
        });
    }
}

