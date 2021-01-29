use std::net;

use net::{TcpListener, TcpStream};

use super::connection;

struct TcpClient {
    socket:     net::TcpStream
}

// struct TcpServer {
//     socket:     net::TcpListener
// }

impl connection::Connection for TcpClient {

    fn start(address_info: &str) -> Result<Box<Self>, connection::ConnectionError> {
        let stream_result = TcpStream::connect(address_info);

        if let Ok(tcp_stream) = stream_result {
            let client = TcpClient {
                socket:     tcp_stream
            };
            return Ok(Box::new(client));
        } else {
            return Err(connection::ConnectionError::new());
        }
    }

    fn read(&self) {
        
    }

    fn write(&self) {

    }

}

