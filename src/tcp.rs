use crate::connection::handle_connection;
use std::net;

pub fn listen(ip: net::IpAddr, port: u16) -> std::io::Result<()> {
    let address = net::SocketAddr::new(ip, port);
    let listener = net::TcpListener::bind(&address)?;

    let address = if port == 0 {
        // TODO: Error handling
        listener.local_addr().unwrap()
    } else {
        address
    };

    println!("Listening at http://{}", &address);

    for stream in listener.incoming() {
        // TODO: Error handling
        let stream = stream.unwrap();

        // TODO: multithreading
        handle_connection(stream);
    }
    Ok(())
}
