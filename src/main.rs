use connections::connection::Connection;


extern crate rusb;
extern crate mavlink;
extern crate clap;

mod connections;

fn main() {
    println!("Hello, world!");

    let devices = rusb::devices().unwrap();
    println!("Devices Found:");
    for device in devices.iter() {
        let device_desc;

        if let Ok(potential_device_desc) = device.device_descriptor() {
            device_desc = potential_device_desc;
        } else {
            continue;
        }

        let prod_id = device_desc.product_id();
        let vend_id = device_desc.vendor_id();

        println!("\tDevice id: {}:{}", vend_id, prod_id);
    }

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
                                .long("logging")
                                .help("Sets if logging will be used to record all messages passed")
                        ).get_matches();

    let ports: Vec<&str> = args
        .value_of("Ports")
        .unwrap()
        .split(",")
        .collect();

    for port in ports {
        println!("Port: {}", port);
        let port_info = port.split(":");
        if port_info.clone().count() != (3 as usize) {
            println!("Invalid port given! Port: {}", port);
            continue;
        }
        let port_info_vec: Vec<&str> = port_info.collect();

        let new_connection;

        match port_info_vec[0] {
            "tcpin" => println!("TCP Server!"),
            "tcpout" => println!("TCP Connection!"),
            "udp" => {
                
                new_connection = connections::udp::UdpConnection::start(port_info_vec[1])
            },
            _ => println!("Unexpected port type!")
        }
    }
}
