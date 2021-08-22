use chrono::prelude::*;
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

pub fn loadconfig(name: String) -> String {
    let mut file = std::fs::File::open("config/config.json").unwrap();
    let mut c = String::new();
    file.read_to_string(&mut c).unwrap();
    json::parse(c.as_str()).unwrap()[name].to_string()
}
