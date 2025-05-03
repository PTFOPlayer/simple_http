use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{
    serwer::{Status, err404},
    threading::threadpool::ThreadPool,
};

use super::{Method, SerwerTrait, content_type_from_file, response_from_file};

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
    path_search: Option<&'static str>,
}

impl Serwer {
    pub fn set_path_search(&mut self, path: Option<&'static str>) {
        self.path_search = path;
    }

    pub fn add_endpoint(&mut self, metod: Method, path: &'static str, handler: fn(Response) -> ()) {
        self.endpoints
            .get_mut(&metod)
            .unwrap()
            .push(Endpoint { path, handler });
    }
}

impl SerwerTrait for Serwer {
    fn new() -> Self {
        Serwer::with_addr("127.0.0.1:3000")
    }

    fn with_addr(addr: &'static str) -> Self {
        let mut endpoints = HashMap::new();
        endpoints.insert(Method::Get, vec![]);
        endpoints.insert(Method::Post, vec![]);
        Self {
            addr,
            endpoints,
            path_search: None,
        }
    }

    fn listen(&mut self, threads: Option<usize>) {
        let listener = TcpListener::bind(self.addr).unwrap();

        let pool = ThreadPool::new(threads.unwrap_or(4));

        let endpoints = Arc::new(self.endpoints.clone());
        let path_search = self.path_search;

        for stream in listener.incoming() {
            let endpoints = Arc::clone(&endpoints);
            pool.execute(move || handle_request(path_search, endpoints, stream.unwrap()));
        }
    }
}

fn handle_request(
    path_search: Option<&'static str>,
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

    let Err(err) = fallback(path_search, &mut stream, path) else {
        return;
    };

    println!("Method: {}, Request: {}, Error: {}", method, path, err);
    err404(&mut stream);
}

fn fallback(
    path_search: Option<&'static str>,
    stream: &mut TcpStream,
    path: &str,
) -> Result<(), io::Error> {
    let Some(path_search) = path_search else {
        return Err(io::Error::other("fallback is off"));
    };

    let path = path_search.to_owned() + path;
    let response = response_from_file(Status::OK, content_type_from_file(&path, "text/plain"), &path)?;
    stream.write_all(&response)?;

    Ok(())
}
