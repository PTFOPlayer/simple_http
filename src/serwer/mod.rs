use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use log::info;

pub mod response;
pub mod serwer;
pub mod spa_serwer;
pub mod content_type;

pub trait SerwerTrait {
    fn with_port(&mut self, port: u16) {
        self.with_addr(format!("127.0.0.1:{}", port))
    }

    fn with_addr(&mut self, addr: String);

    fn listen(&mut self, threads: Option<usize>);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Method {
    Get,
    Post,
}

impl Method {
    pub fn from_str(method: &str) -> Self {
        match method {
            "Get" | "GET" | "get" => Method::Get,
            "Post" | "POST" | "post" => Method::Post,
            _ => panic!("not supported method"),
        }
    }
}

#[derive(Clone)]
pub struct Status;

impl Status {
    pub const OK: &'static str = "HTTP/1.1 200 OK";
    pub const NOT_FOUND: &'static str = "HTTP/1.1 404 NOT FOUND";
}

pub fn err404(stream: &mut TcpStream) {
    let site404 = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>Simple HTTP</h1>
    <p>404</p>
</body>
</html>"#;
    let response = format!(
        "{}\nContent-Length: {}\n\n{}",
        Status::NOT_FOUND,
        site404.len(),
        site404
    );

    stream.write_all(response.as_bytes()).unwrap();
}

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
