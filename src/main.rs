pub mod serwer;
pub mod threading;

use serwer::{Method, Serwer};

fn main() {
    let mut serwer = Serwer::new();

    serwer.add_endpoint(Method::Get, "/", |mut res| {
        let contents = "lorem";
        res.send(contents.as_bytes());
    });

    serwer.listen(Some(4));
}