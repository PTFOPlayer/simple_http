use std::{io::Write, net::TcpStream};

pub struct Response<'a, 'b, 'c> {
    pub stream: &'a mut TcpStream,
    pub content_type: &'b str,
    pub status: &'c str,
}

impl<'a, 'b, 'c> Response<'a, 'b, 'c> {
    pub fn send(&mut self, contents: &[u8]) {
        let length = contents.len();

        let response = format!(
            "{}\nContent-Length: {}\nContent-Type:{}\n\n",
            self.status, length, self.content_type
        )
        .as_bytes()
        .to_vec();

        self.stream.write(&response).unwrap();
        self.stream.write(&contents).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn set_status_line(&mut self, status: &'c str) {
        self.status = status;
    }

    pub fn set_status_content_type(&mut self, status: &'c str) {
        self.status = status;
    }
}
