use std::{
    collections::HashMap,
    fs,
    hash::Hash,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::threading::threadpool::ThreadPool;

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

pub struct Response<'a, 'b> {
    stream: &'a mut TcpStream,
    status: &'b str,
}

impl<'a, 'b> Response<'a, 'b> {
    pub fn send(&mut self, contents: &[u8]) {
        let length = contents.len();

        let response = format!("{}\nContent-Length: {}\n\n", self.status, length)
            .as_bytes()
            .to_vec();

        self.stream.write(&response).unwrap();
        self.stream.write(&contents).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn set_status_line(&mut self, status: &'b str) {
        self.status = status;
    }
}

#[derive(Clone)]
struct Endpoint {
    path: &'static str,
    handler: fn(Response) -> (),
}

pub struct Serwer {
    addr: &'static str,
    endpoints: HashMap<Method, Vec<Endpoint>>,
    path_fallback: bool,
}

impl Serwer {
    pub fn new() -> Self {
        Serwer::with_addr("127.0.0.1:3000")
    }

    pub fn with_addr(addr: &'static str) -> Self {
        let mut endpoints = HashMap::new();
        endpoints.insert(Method::Get, vec![]);
        endpoints.insert(Method::Post, vec![]);
        Self {
            addr,
            endpoints,
            path_fallback: true,
        }
    }

    pub fn set_path_search_fallback(&mut self, fallback: bool) {
        self.path_fallback = fallback;
    }

    pub fn listen(&mut self, threads: Option<usize>) {
        let listener = TcpListener::bind(self.addr).unwrap();

        let pool = ThreadPool::new(threads.unwrap_or(4));

        let path_fallback = self.path_fallback;
        let endpoints = Arc::new(self.endpoints.clone());

        for stream in listener.incoming() {
            let endpoints = Arc::clone(&endpoints);
            pool.execute(move || handle_request(path_fallback, endpoints, stream.unwrap()));
        }
    }

    pub fn add_endpoint(&mut self, metod: Method, path: &'static str, handler: fn(Response) -> ()) {
        self.endpoints
            .get_mut(&metod)
            .unwrap()
            .push(Endpoint { path, handler });
    }
}

fn handle_request(
    path_fallback: bool,
    endpoints: Arc<HashMap<Method, Vec<Endpoint>>>,
    mut stream: TcpStream,
) {
    let buf_reader = BufReader::new(&stream);
    let mut req_lines = buf_reader.lines();
    let binding = req_lines.next().unwrap().unwrap();

    let mut request_line = binding.split(' ');
    let method = request_line.next().unwrap();
    let path = request_line.next().unwrap();

    let list = &endpoints[&Method::from_str(method)];

    if let Some(endpoint) = list.iter().find(|endpoint| endpoint.path == path) {
        (endpoint.handler)(Response {
            stream: &mut stream,
            status: Status::OK,
        });
        return;
    }

    let Err(err) = fallback(path_fallback, &mut stream, path) else {
        return;
    };

    println!("Method: {}, Request: {}, Error: {}", method, path, err);
    err404(&mut stream);
}

fn fallback(path_fallback: bool, stream: &mut TcpStream, path: &str) -> Result<(), io::Error> {
    if !path_fallback {
        return Err(io::Error::other("fallback is off"));
    }

    let response = response_from_file(Status::OK, path)?;
    stream.write_all(&response)?;

    Ok(())
}

fn err404(stream: &mut TcpStream) {
    let response = response_from_file(Status::NOT_FOUND, "/404.html").unwrap();
    stream.write_all(&response).unwrap();
}

fn response_from_file(status: &str, path: &str) -> Result<Vec<u8>, io::Error> {
    let content_path = "web".to_string() + path;

    let mut contents = fs::read(content_path)?;
    let length = contents.len();

    let mut response = format!("{status}\nContent-Length: {length}\n\n")
        .as_bytes()
        .to_vec();

    response.append(&mut contents);

    Ok(response)
}
