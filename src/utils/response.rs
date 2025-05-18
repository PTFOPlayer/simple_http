use std::{io::Write, net::TcpStream};

use super::content_type::ContentType;

pub struct Response<'a, 'b> {
    pub stream: &'a mut TcpStream,
    pub content_type: ContentType,
    pub status: &'b str,
}

impl<'a, 'b> Response<'a, 'b> {
    pub fn send(&mut self, body: &[u8]) {
        let length = body.len();

        let response = format!(
            "{}\nContent-Length: {}\nContent-Type:{}\n\n",
            self.status,
            length,
            self.content_type.to_string()
        );

        self.stream.write(response.as_bytes()).unwrap();
        self.stream.write(&body).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn set_status_line(&mut self, status: &'b str) {
        self.status = status;
    }

    pub fn set_status_content_type(&mut self, status: &'b str) {
        self.status = status;
    }
}
