use chrono::prelude::*;
use std::io::Read;
use LogType::*;

pub enum LogType {
    Info,
    Warn,
    Error,
}

/// log(LogType, format!("Text"))
///
pub fn log(logtype: LogType, text: String) {
    match logtype {
        Info => {
            println!(
                "[Info] [{}]: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                text
            );
        }
        Warn => {
            println!(
                "[Warn] [{}]: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                text
            );
        }
        Error => {
            println!(
                "[Error] [{}]: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                text
            );
        }
    }
}

pub fn cmdlog(author: String, cmd: String) {
    println!(
        "[Command] [{}] [{}]: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        author,
        cmd
    );
}

pub fn loadconfig(name: String) -> String {
    let mut file = std::fs::File::open("config/config.json").unwrap();
    let mut c = String::new();
    file.read_to_string(&mut c).unwrap();
    json::parse(c.as_str()).unwrap()[name].to_string()
}
