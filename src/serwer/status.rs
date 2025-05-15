#[derive(Clone)]
pub struct Status;

impl Status {
    pub const OK: &'static str = "HTTP/1.1 200 OK";
    pub const CREATED: &'static str = "HTTP/1.1 201 Created";
    pub const ACCEPTED: &'static str = "HTTP/1.1 202 Accepted";
    pub const NO_CONTENT: &'static str = "HTTP/1.1 204 No Content";

    pub const MOVED_PERMANENTLY: &'static str = "HTTP/1.1 301 Moved Permanently";
    pub const FOUND: &'static str = "HTTP/1.1 302 Found";
    pub const SEE_OTHER: &'static str = "HTTP/1.1 303 See Other";
    pub const NOT_MODIFIED: &'static str = "HTTP/1.1 304 Not Modified";

    pub const BAD_REQUEST: &'static str = "HTTP/1.1 400 Bad Request";
    pub const UNAUTHORIZED: &'static str = "HTTP/1.1 401 Unauthorized";
    pub const FORBIDDEN: &'static str = "HTTP/1.1 403 Forbidden";
    pub const NOT_FOUND: &'static str = "HTTP/1.1 404 Not Found";
    pub const METHOD_NOT_ALLOWED: &'static str = "HTTP/1.1 405 Method Not Allowed";

    pub const INTERNAL_SERVER_ERROR: &'static str = "HTTP/1.1 500 Internal Server Error";
    pub const NOT_IMPLEMENTED: &'static str = "HTTP/1.1 501 Not Implemented";
    pub const BAD_GATEWAY: &'static str = "HTTP/1.1 502 Bad Gateway";
    pub const SERVICE_UNAVAILABLE: &'static str = "HTTP/1.1 503 Service Unavailable";
}