use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub serwer: Serwer,
}

#[derive(Serialize, Deserialize)]
pub struct Serwer {
    pub listen: u16,
    pub spa: Option<String>,
    pub root: String,
    pub threads: Option<usize>,
}
