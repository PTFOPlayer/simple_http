use std::{io::Write, net::TcpStream};

pub mod response;
pub mod serwer;
pub mod spa_serwer;

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

fn err404(stream: &mut TcpStream) {
    let site404 = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    404
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

fn content_type_from_file<'a>(path: &str, default: &'a str) -> &'a str {
    match path {
        _ if path.ends_with(".css") => "text/css",
        _ if path.ends_with(".html") => "text/html",
        _ if path.ends_with(".xml") => "text/xml",
        _ if path.ends_with(".js") => "application/javascript",
        _ => default,
    }
}
