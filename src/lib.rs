use chrono::prelude::*;
use std::io::Read;
use LogType::*;

/*
로그 / 설정 가져오기 / 명령어 로그 등을 관리하는 lib.rs입니다.
*/

pub enum LogType {
    Info,  // 일반 모드
    Warn,  // 경고
    Error, // 오류
}

/// log(LogType, format!("Text"))
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
