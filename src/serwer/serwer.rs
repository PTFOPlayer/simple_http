use std::{
    fs,
    io::{self},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use dashmap::DashMap;
use log::warn;

use crate::{
    threading::threadpool::ThreadPool,
    utils::{
        content_type::ContentType, err404, request::parse_request, response::Response,
        status::Status,
    },
};

use super::{Method, SerwerTrait, spa_serwer::SpaSerwer};

#[derive(Clone)]
struct Endpoint {
    path: &'static str,
    handler: fn(Response) -> (),
}

pub struct Serwer {
    addr: String,
    endpoints: DashMap<Method, Vec<Endpoint>>,
    path_search: Option<String>,
}

impl Serwer {
    pub fn new() -> Self {
        let endpoints = DashMap::new();
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
    endpoints: Arc<DashMap<Method, Vec<Endpoint>>>,
    mut stream: TcpStream,
) {
    let request = parse_request(&mut stream);

    let list = &endpoints.get(&Method::from_str(&request.method)).unwrap();

    if let Some(endpoint) = list.iter().find(|endpoint| endpoint.path == request.url) {
        (endpoint.handler)(Response {
            stream: &mut stream,
            status: Status::OK,
            content_type: ContentType::TextPlain,
        });
        return;
    }

    let Err(err) = search_path(path_search, &mut stream, &request.url) else {
        return;
    };

    warn!(
        "Method: {}, Request: {}, Error: {}",
        request.method, request.url, err
    );
    err404::err404(&mut stream);
}

fn search_path(
    path_search: Arc<Option<String>>,
    stream: &mut TcpStream,
    path: &str,
) -> Result<(), io::Error> {
    let Some(path_search) = path_search.as_ref() else {
        return Err(io::Error::other("search_path is off"));
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
