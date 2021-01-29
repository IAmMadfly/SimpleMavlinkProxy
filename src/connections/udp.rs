use std::net;

use connection::{Connection, ConnectionError};

use super::connection;

pub struct UdpConnection {
    socket:     net::UdpSocket
}

impl connection::Connection for UdpConnection {

    fn start(address_info: &str) -> Result<Box<Self>, ConnectionError> {
        let stream_result = net::UdpSocket::bind(address_info);

        if let Ok(socket) = stream_result {
            let client = UdpConnection {
                socket:     socket
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
