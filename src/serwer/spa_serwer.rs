use std::{
    fs,
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use log::warn;

use crate::{
    serwer::{content_type::ContentType, parse_request},
    threading::threadpool::ThreadPool,
};

use super::{SerwerTrait, Status, err404, response::Response};

pub struct SpaSerwer {
    addr: String,
    entry: String,
}

impl SpaSerwer {
    pub fn set_entry_point(&mut self, entry: String) {
        self.entry = entry;
    }

    pub fn new() -> Self {
        Self {
            addr: "127.0.0.1:3000".to_owned(),
            entry: ".".to_owned(),
        }
    }
}

impl SerwerTrait for SpaSerwer {
    fn with_addr(&mut self, addr: String) {
        self.addr = addr;
    }

    fn listen(&mut self, threads: Option<usize>) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        let pool = ThreadPool::new(threads.unwrap_or(4));

        let entry = Arc::new(self.entry.clone());
        for stream in listener.incoming() {
            let entry = Arc::clone(&entry);
            let stream = stream.unwrap();
            pool.execute(move || handle_request(stream, entry));
        }
    }
}

fn handle_request(mut stream: TcpStream, entry: Arc<String>) {
    let request = parse_request(&mut stream);

    let content_type = ContentType::from_file_ext_or(&request.url, ContentType::TextHtml);
    let (entry, content_type) = if content_type == ContentType::TextHtml {
        (entry.to_string() + "/index.html", content_type)
    } else {
        let entry = entry.to_string() + &request.url;
        (entry, content_type)
    };

    if let Ok(contents) = fs::read(&entry) {
        let mut res = Response {
            stream: &mut stream,
            content_type,
            status: Status::OK,
        };

        res.send(&contents);
        return;
    }

    warn!("Not found: \n {}", entry);
    err404(&mut stream);
}
