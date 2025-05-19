use std::{
    fs,
    io::{self},
    net::{TcpListener, TcpStream},
    path::Path,
    process::Command,
    sync::Arc,
};

use dashmap::DashMap;
use log::warn;

use crate::{
    threading::threadpool::ThreadPool,
    utils::{
        content_type::ContentType,
        err404,
        request::{Request, parse_request},
        response::Response,
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
    endpoints: Arc<DashMap<Method, Vec<Endpoint>>>,
    path_search: Option<String>,
    allow_php_exec: bool,
}

impl Serwer {
    pub fn new() -> Self {
        let endpoints = Arc::new(DashMap::new());
        endpoints.insert(Method::Get, vec![]);
        endpoints.insert(Method::Post, vec![]);
        Self {
            addr: "127.0.0.1:8080".to_owned(),
            endpoints,
            path_search: None,
            allow_php_exec: true,
        }
    }
    pub fn set_path_search(&mut self, path: Option<String>) {
        self.path_search = path;
    }

    pub fn set_allow_php(&mut self, allow: bool) {
        self.allow_php_exec = allow;
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

        let path_search = Arc::new(self.path_search.clone());

        for stream in listener.incoming() {
            let endpoints = Arc::clone(&self.endpoints.clone());
            let allow_php_exec = self.allow_php_exec;
            let path_search = Arc::clone(&path_search);
            pool.execute(move || {
                RequestHandler {
                    path_search,
                    endpoints,
                    stream: stream.unwrap(),
                    allow_php_exec: allow_php_exec,
                }
                .handle_request()
            });
        }
    }
}

struct RequestHandler {
    path_search: Arc<Option<String>>,
    endpoints: Arc<DashMap<Method, Vec<Endpoint>>>,
    stream: TcpStream,
    allow_php_exec: bool,
}

impl RequestHandler {
    pub fn handle_request(&mut self) {
        let request = parse_request(&mut self.stream);

        {
            let list = self
                .endpoints
                .get(&Method::from_str(&request.method))
                .unwrap();

            if let Some(endpoint) = list.iter().find(|endpoint| endpoint.path == request.url) {
                (endpoint.handler)(Response::new_ok(&mut self.stream, ContentType::TextPlain));
                return;
            }
        }

        let Err(err) = self.search_path(&request) else {
            return;
        };

        warn!(
            "Method: {}, Request: {}, Error: {}",
            request.method, request.url, err
        );
        err404::err404(&mut self.stream);
    }

    fn search_path(&mut self, request: &Request) -> Result<(), io::Error> {
        let Some(path_search) = self.path_search.as_ref() else {
            return Err(io::Error::other("search_path is off"));
        };

        let path = path_search.to_owned() + &request.url;

        let ext = Path::new(&path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        if ext == "php" && self.allow_php_exec {
            if self.try_send_php(&path) {
                return Ok(());
            }

            return Err(io::Error::other("Access to PHP files is forbidden"));
        }

        let contents = fs::read(&path)?;
        let mut res = Response {
            stream: &mut self.stream,
            content_type: ContentType::from_file_ext(&path),
            status: Status::OK,
        };
        res.send(&contents);
        Ok(())
    }

    fn try_send_php(&mut self, path: &str) -> bool {
        let Ok(content) = execute_php_script(path) else {
            warn!("Failed executing PHP: {}", path);
            return false;
        };

        let mut res = Response::new_ok(&mut self.stream, ContentType::TextHtml);
        res.send(&content);

        true
    }
}

fn execute_php_script(path: &str) -> Result<Vec<u8>, io::Error> {
    let output = Command::new("php").arg(path).output()?;

    if !output.status.success() {
        let stderr_msg = String::from_utf8_lossy(&output.stderr);
        log::warn!("PHP script execution failed: {}", stderr_msg);
        return Err(io::Error::new(io::ErrorKind::Other, "PHP script failed"));
    }

    Ok(output.stdout)
}
