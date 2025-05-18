pub mod serwer;
pub mod spa_serwer;

pub trait SerwerTrait {
    fn with_port(&mut self, port: u16) {
        self.with_addr(format!("127.0.0.1:{}", port))
    }

    fn with_addr(&mut self, addr: String);

    fn listen(&mut self, threads: Option<usize>);
}

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
