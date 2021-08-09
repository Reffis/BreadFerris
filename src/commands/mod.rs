pub use breadferris::{loadconfig, log, LogType::*};

/*
image: 다양한 api를 이용하여, 이미지를 가져오는 명령어를 모아둔 카테고리입니다.
owner: 봇 주인만 쓸 수 있는 명령어를 모아둔 카테고리입니다.
util: 사실상, 메인 명령어가 있는 카테고리입니다.
other: 어떤 카테고리에 넣을지 모르거나, 애매한 명령어를 모아둔 카테고리입니다.
*/
pub mod image;
pub mod other;
pub mod owner;
pub mod util;
