use std::{fs, io::prelude::*, path};

pub fn handle_http_request(mut stream: impl Read + Write, http_request: Vec<String>) {
    let control_data = if http_request.len() > 0 {
        &http_request[0]
    } else {
        ""
    };

    // TODO: extract request_method, request_target, and protocol_version
    let control_data_parts = control_data.split_whitespace().collect::<Vec<&str>>();
    // TODO: err check length
    let request_method = control_data_parts[0];
    let request_target = control_data_parts[1];
    let protocol_version = control_data_parts[2];
    println!(
        "request_method: {}\nrequest_target: {}\nprotocol_version: {}",
        request_method, request_target, protocol_version
    );

    const OK_STATUS: &str = "HTTP/1.1 200 OK";
    const NOT_FOUND_STATUS: &str = "HTTP/1.1 400 NOT FOUND";
    const INTERNAL_SERVER_ERR_STATUS: &str = "HTTP/2.2 500 INTERNAL SERVER ERROR";

    let path = path::Path::new(&request_target[..]);
    println!("Path: {}", path.display());
    let read_result = fs::read_to_string(&request_target[..]);

    let (contents, status_line) = match read_result {
        Ok(contents) => (contents, OK_STATUS),
        Err(_) => {
            let (contents, status_line) = if &request_target[..] == "/" {
                let (contents, status_line) = match fs::read_to_string("defaults/index.html") {
                    Ok(contents) => (contents, OK_STATUS),
                    Err(_) => (String::from(""), INTERNAL_SERVER_ERR_STATUS),
                };
                (contents, status_line)
            } else {
                let (contents, status_line) = match fs::read_to_string("defaults/404.html") {
                    Ok(contents) => (contents, NOT_FOUND_STATUS),
                    Err(_) => (String::from(""), INTERNAL_SERVER_ERR_STATUS),
                };
                (contents, status_line)
            };
            (contents, status_line)
        }
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // println!("Request: {:#?}", http_request);
}
