use chrono::prelude::*;
use LogType::*;

pub enum LogType {
    Info,
    Warn,
    Error
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
