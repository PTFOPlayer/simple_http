use std::{
    fs,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use log::warn;

use crate::threading::threadpool::ThreadPool;

use super::{SerwerTrait, Status, content_type_from_file, err404, response::Response};

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

        for stream in listener.incoming() {
            let entry = self.entry.clone();
            let stream = stream.unwrap();
            pool.execute(move || handle_request(stream, entry));
        }
    }
}

fn handle_request(mut stream: TcpStream, entry: String) {
    let buf_reader = BufReader::new(&stream);
    let mut req_lines = buf_reader.lines();
    let binding = req_lines.next().unwrap().unwrap();

    let mut request_line = binding.split(' ');
    let _ = request_line.next().unwrap();
    let path = request_line.next().unwrap();

    let content_type = content_type_from_file(path, "text/html");
    let entry = if content_type == "text/html" {
        entry.to_string() + "/index.html"
    } else {
        entry.to_string() + path
    };

    if let Ok(contents) = fs::read(&entry) {
        let mut res = Response {
            stream: &mut stream,
            content_type: content_type_from_file(&entry, "text/plain"),
            status: Status::OK,
        };

        res.send(&contents);
        return;
    }


    warn!("Not found: \n {}", entry);
    err404(&mut stream);
}
