pub mod config;
pub mod serwer;
pub mod threading;

use std::process::exit;

use config::parse_config;
use log::{error, info, warn};
use serwer::{SerwerTrait, serwer::Serwer};

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    info!("Serwer starting...");

    let mut args = std::env::args().skip(1);

    let mut config = String::from("./simple_http_conifg.toml");
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--config" | "-c" => config = args.next().unwrap(),
            _ => panic!("unknown arg"),
        }
    }

    let config = match parse_config(config) {
        Ok(res) => res,
        Err(err) => {
            error!("{}", err);
            exit(-1);
        }
    };

    let mut serwer = Serwer::new();
    serwer.with_port(config.serwer.listen);
    serwer.set_path_search(Some(config.serwer.root));

    let mut serwer: Box<dyn SerwerTrait> = if let Some(spa) = config.serwer.spa {
        warn!("Moving to spa mode");
        let mut serwer = serwer.into_spa();
        serwer.set_entry_point(spa);
        Box::new(serwer)
    } else {
        Box::new(serwer)
    };

    serwer.listen(config.serwer.threads);
}
