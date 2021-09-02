use serde_derive::Deserialize;
use std::{fmt::Debug, io::Read};
#[derive(Debug, Deserialize)]

pub struct Config {
    config: Option<BotConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub token: Option<String>,

    pub prefix: Option<Vec<String>>,
    pub support_channel: Option<u64>,
    pub application_id: Option<u64>,
}

pub fn loadconfig() -> BotConfig {
    let mut file = std::fs::File::open("config/config.toml").unwrap();
    let mut c = String::new();
    file.read_to_string(&mut c).unwrap();
    let decoded: Config = toml::from_str(c.as_str()).unwrap();
    decoded.config.unwrap()
}
