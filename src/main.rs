mod connection;
mod http;
mod tcp;

use std::net;

use clap::Parser;

/// A modern webserver
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP address
    #[arg(short, long, default_value_t = net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)))]
    ip: net::IpAddr,

    /// Port number
    #[arg(short, long, default_value_t = 0)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    match tcp::listen(args.ip, args.port) {
        Err(error) => println!("Error: {}", error),
        _ => (),
    }
}
