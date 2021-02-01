use connections::connection::Connection;
use rusb::{Device, GlobalContext};


extern crate rusb;
extern crate mavlink;
extern crate clap;

mod connections;


fn get_devices() -> Vec<Device<GlobalContext>> {
    let devices = rusb::devices().unwrap();
    let mut vec_devices = Vec::new();

    for device in devices.iter() {

        if let Ok(_) = device.device_descriptor() {
            vec_devices.push(device);
        } else {
            continue;
        }
    }

    vec_devices
}

fn main() {
    let args =  clap::App::new("Simple Proxy")
                        .author("Karl Runge, karl.spartacus@gmail.com")
                        .version("0.1.0")
                        .about("Simple proxy, with ability to record Mavlink messages to a file if required")
                        .arg(
                            clap::Arg::with_name("Ports")
                                .short("p")
                                .long("ports")
                                .help("Sets what inputs should be considered to proxy to each other")
                                .required(true)
                                .takes_value(true)
                        ).arg(
                            clap::Arg::with_name("Logging")
                                .short("l")
                                .long("log")
                                .help("Sets if logging will be used to record all messages passed")
                        ).arg(
                            clap::Arg::with_name("Debugging")
                                .short("d")
                                .long("debug")
                                .help("Adds additional printouts for debugging information")
                        ).get_matches();
    
    let _devices = get_devices();
    let mut connections = Vec::new();

    let ports: Vec<&str> = args
        .value_of("Ports")
        .unwrap()
        .split(",")
        .collect();

    for port in ports {
        let port_info = port.split(":");
        if port_info.clone().count() != (3 as usize) {
            println!("Invalid port given! Port: {}", port);
            continue;
        }
        let port_info_vec: Vec<&str> = port_info.collect();

        let mut connection_info = port_info_vec.clone();
        connection_info.remove(0);

        let connection_info = connection_info.join(":");
        println!("Port: {}, Connection info: {}", port, connection_info);

        match port_info_vec[0] {
            "tcpin" => println!("TCP Server!"),
            "tcpout" => println!("TCP Connection!"),
            "udp" => {
                
                let new_connection = connections::udp::UdpConnection::start(
                    connection_info
                );
                if let Ok(conn) = new_connection {
                    connections.push(conn);
                }

            },
            _ => println!("Unexpected port type!")
        }
    }

    loop {
        let mut buffer = [0u8; 1024];
        for conn in &connections {
            let res = conn.lock().unwrap().read(&mut buffer);
            
            match res {
                Ok(size) => {
                    let data: Vec<u8> = {
                        let mut vec = Vec::with_capacity(size);
                        for i in 0..size {
                            vec.push(buffer[i]);
                        }
                        vec
                    };
                    println!("Read: {}, Data: {}", size, String::from_utf8(data).unwrap());
                },
                Err(er) => {
                    println!("Failed to get data: {}", er);
                }
            }
        }
    }
}
