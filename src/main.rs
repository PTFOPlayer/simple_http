pub mod serwer;
pub mod threading;

use serwer::{serwer::Serwer, spa_serwer::SpaSerwer, Method, SerwerTrait};

fn main() {
    let mut args = std::env::args();
    args.next();
    let Some(arg) = args.next() else {
        serwer();
        return;
    };

    match arg.as_str() {
        "--spa" | "-s" => spa(),
        _ => serwer(),
    }


}

fn serwer() {
    let mut serwer = Serwer::new();

    serwer.set_path_search(Some("web"));

    serwer.add_endpoint(Method::Get, "/", |mut res| {
        let contents = "lorem";
        res.send(contents.as_bytes());
    });

    serwer.listen(Some(4));
}

fn spa() {
    let mut spa_serwer = SpaSerwer::new();

    spa_serwer.set_entry_point("./spa/dist");
    spa_serwer.listen(None);
}


