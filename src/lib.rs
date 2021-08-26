use chrono::prelude::*;
use serde_derive::Deserialize;
use std::{fmt::Debug, io::Read};
use LogType::*;

/*
로그 / 설정 가져오기 / 명령어 로그 등을 관리하는 lib.rs입니다.
*/

pub enum LogType {
    Info,  // 일반
    Warn,  // 경고
    Error, // 오류
}

/// log(LogType, format!("Text"))
pub fn log<T: ?Sized + Debug>(logtype: LogType, text: &T) {
    match logtype {
        Info => {
            println!(
                "[Info] [{}]: {:?}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                &text
            );
        }
        Warn => {
            println!(
                "[Warn] [{}]: {:?}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                &text
            );
        }
        Error => {
            println!(
                "[Error] [{}]: {:?}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                &text
            );
        }
    }
}

pub fn cmdlog<T, E>(author: &T, cmd: &E)
where
    T: ?Sized + Debug,
    E: ?Sized + Debug,
{
    println!(
        "[Command] [{}] [{:?}]: {:?}",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        author,
        cmd
    );
}

#[derive(Debug, Deserialize)]
pub struct Config {
    config: Option<BotConfig>,
}

#[derive(Debug, Deserialize)]
pub struct BotConfig {
    pub token: Option<String>,
    pub prefix1: Option<String>,
    pub prefix2: Option<String>,
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
