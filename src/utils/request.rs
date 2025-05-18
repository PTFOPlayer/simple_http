use std::{io::{BufRead, BufReader}, net::TcpStream};

use log::info;


pub struct Request {
    pub method: String,
    pub url: String,
}

pub fn parse_request(stream: &mut TcpStream) -> Request {
    let buf_reader = BufReader::new(stream);
    let mut req_lines = buf_reader.lines();
    let binding = req_lines.next().unwrap().unwrap();

    let mut request_line = binding.split(' ');
    let method = request_line.next().unwrap().to_owned();
    let url = request_line.next().unwrap().to_owned();

    info!("{} => {}", method, url);

    Request { method, url }
}
