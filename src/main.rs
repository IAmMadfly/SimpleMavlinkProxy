use std::{env, net};

extern crate mavlink;
extern crate clap;
use clap::{
    App,
    Arg
};

fn main() {
    println!("Hello, world!");

    let args =  App::new("Simple Proxy")
                        .author("Karl Runge, karl.spartacus@gmail.com")
                        .version("0.1.0")
                        .about("Simple proxy, with ability to record Mavlink messages to a file if required")
                        .arg(
                            Arg::with_name("Ports")
                                .short("p")
                                .long("ports")
                                .help("Sets what inputs should be considered to proxy to each other")
                                .required(true)
                                .takes_value(true)
                        ).arg(
                            Arg::with_name("Logging")
                                .short("l")
                                .long("logging")
                                .help("Sets if logging will be used to record all messages passed")
                        ).get_matches();
    
    println!("Arguments: {:?}", args);
}
