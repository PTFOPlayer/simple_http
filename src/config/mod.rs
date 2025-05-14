use std::{fs, io};

use model::Config;

pub mod model;

pub fn parse_config(path: String) -> Result<Config, io::Error> {
    let file = fs::read_to_string(path)?;
    toml::from_str(&file).map_err(|err| io::Error::new(io::ErrorKind::Unsupported, err.message()))
}
