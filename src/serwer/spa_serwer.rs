use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::threading::threadpool::ThreadPool;

use super::{SerwerTrait, Status, content_type_from_file, err404, response_from_file};

pub struct SpaSerwer {
    addr: &'static str,
    entry: &'static str,
}

impl SpaSerwer {
    pub fn set_entry_point(&mut self, entry: &'static str) {
        self.entry = entry;
    }
}

impl SerwerTrait for SpaSerwer {
    fn new() -> Self {
        Self::with_addr("127.0.0.1:3000")
    }

    fn with_addr(addr: &'static str) -> Self {
        Self { addr, entry: "." }
    }

    fn listen(&mut self, threads: Option<usize>) {
        let listener = TcpListener::bind(self.addr).unwrap();

        let pool = ThreadPool::new(threads.unwrap_or(4));

        let entry = self.entry;
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            pool.execute(move || handle_request(stream, entry));
        }
    }
}

fn handle_request(mut stream: TcpStream, entry: &str) {
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

    if let Ok(response) = response_from_file(Status::OK, content_type, &entry) {
        stream.write_all(&response).unwrap();
        return;
    }

    err404(&mut stream);
}
