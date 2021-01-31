use std::net;
use std::sync;

use connection::{Connection, ConnectionError};

use super::connection;

pub struct UdpConnection {
    socket:     net::UdpSocket,
    last_recv:  Option<net::SocketAddr>
}

impl Connection for UdpConnection {

    fn start(address_info: &str) -> std::io::Result<sync::Arc<sync::Mutex<Self>>> {
        let stream_result = net::UdpSocket::bind(address_info);

        match stream_result {
            Ok(socket) => {
                return Ok(
                    sync::Arc::new(
                        sync::Mutex::new(
                            UdpConnection {
                                socket:     socket,
                                last_recv:  None
                            }
                        )
                    )
                );
            },
            Err(er) => {
                Err(er)
            }
        }
    }

    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let result = self.socket.recv_from(buffer);

        match result {
            Ok(res) => {
                self.last_recv = Some(res.1);
                return Ok(res.0);
            },
            Err(er) => {
                return Err(er);
            }
        }
    }

    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.socket.send(buffer)
    }
}
