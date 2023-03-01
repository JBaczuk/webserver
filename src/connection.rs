use std::io::{prelude::*, BufReader};

use crate::http;

pub fn handle_connection(mut stream: impl Read + Write) {
    let buf_reader = BufReader::new(&mut stream);
    let data: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // TODO: handle no lines

    handle_recv_data(data, stream);
}

fn handle_recv_data(data: Vec<String>, stream: impl Read + Write) {
    let first_line = if data.len() > 0 { &data[0] } else { "" };

    if first_line.contains("HTTP") {
        http::handle_http_request(stream, data)
    }
}
