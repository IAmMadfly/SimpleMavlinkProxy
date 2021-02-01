use std::{
    io::{Read, Write}, 
    net, 
    thread,
    sync
};

use net::{TcpListener, TcpStream};

use super::connection;

struct TcpClient {
    socket:         net::TcpStream
}

struct TcpServer {
    listener:       net::TcpListener,
    client:         Option<net::TcpStream>
}

impl connection::Connection for TcpClient {

    fn start(address_info: String) -> std::io::Result<sync::Arc<sync::Mutex<Self>>> {
        let stream_result = TcpStream::connect(address_info);

        match stream_result {
            Ok(tcp_stream) => {
                return Ok(
                    sync::Arc::new(
                        sync::Mutex::new(
                            TcpClient {
                                socket: tcp_stream
                            }
                        )
                    )
                );
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

    fn start(address_info: String) -> std::io::Result<sync::Arc<sync::Mutex<Self>>> {
        let mut server = sync::Arc::new(
            sync::Mutex::new(
                TcpServer {
                    listener:   TcpListener::bind(address_info)?,
                    client:     None
                }
            )
        );

        let mut server_thread = sync::Arc::clone(&server);

        thread::spawn(move || {
            let mut locked_server = server_thread.lock().unwrap();
            match locked_server.listener.accept() {
                Ok((sock, addr)) => {
                    locked_server.client = Some(sock);
                },
                Err(er) => {

                }
            }
        });

        return Ok(server);
    }

    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

