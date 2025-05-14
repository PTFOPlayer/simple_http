use std::{
    collections::HashMap,
    fs,
    io::{self},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use log::warn;

use crate::{
    serwer::{content_type::ContentType, err404, parse_request, Status},
    threading::threadpool::ThreadPool,
};

use super::{
    Method, SerwerTrait, response::Response, spa_serwer::SpaSerwer,
};

#[derive(Clone)]
struct Endpoint {
    path: &'static str,
    handler: fn(Response) -> (),
}

pub struct Serwer {
    addr: String,
    endpoints: HashMap<Method, Vec<Endpoint>>,
    path_search: Option<String>,
}

impl Serwer {
    pub fn new() -> Self {
        let mut endpoints = HashMap::new();
        endpoints.insert(Method::Get, vec![]);
        endpoints.insert(Method::Post, vec![]);
        Self {
            addr: "127.0.0.1:80".to_owned(),
            endpoints,
            path_search: None,
        }
    }
    pub fn set_path_search(&mut self, path: Option<String>) {
        self.path_search = path;
    }

    pub fn add_endpoint(&mut self, metod: Method, path: &'static str, handler: fn(Response) -> ()) {
        self.endpoints
            .get_mut(&metod)
            .unwrap()
            .push(Endpoint { path, handler });
    }

    pub fn into_spa(self) -> SpaSerwer {
        let mut spa = SpaSerwer::new();
        spa.with_addr(self.addr);

        spa
    }
}

impl SerwerTrait for Serwer {
    fn with_addr(&mut self, addr: String) {
        self.addr = addr
    }

    fn listen(&mut self, threads: Option<usize>) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        let pool = ThreadPool::new(threads.unwrap_or(4));

        let endpoints = Arc::new(self.endpoints.clone());
        let path_search = Arc::new(self.path_search.clone());

        for stream in listener.incoming() {
            let endpoints = Arc::clone(&endpoints);
            let path_search = Arc::clone(&path_search);
            pool.execute(move || handle_request(path_search, endpoints, stream.unwrap()));
        }
    }
}

fn handle_request(
    path_search: Arc<Option<String>>,
    endpoints: Arc<HashMap<Method, Vec<Endpoint>>>,
    mut stream: TcpStream,
) {
    let request = parse_request(&mut stream);

    let list = &endpoints[&Method::from_str(&request.method)];

    if let Some(endpoint) = list.iter().find(|endpoint| endpoint.path == request.url) {
        (endpoint.handler)(Response {
            stream: &mut stream,
            status: Status::OK,
            content_type: ContentType::TextPlain,
        });
        return;
    }

    let Err(err) = fallback(path_search, &mut stream, &request.url) else {
        return;
    };

    warn!(
        "Method: {}, Request: {}, Error: {}",
        request.method, request.url, err
    );
    err404(&mut stream);
}

fn fallback(
    path_search: Arc<Option<String>>,
    stream: &mut TcpStream,
    path: &str,
) -> Result<(), io::Error> {
    let Some(path_search) = path_search.as_ref() else {
        return Err(io::Error::other("fallback is off"));
    };

    let path = path_search.to_owned() + path;

    let mut res = Response {
        stream,
        content_type: ContentType::from_file_ext(&path),
        status: Status::OK,
    };

    let contents = fs::read(path)?;

    res.send(&contents);

    Ok(())
}
