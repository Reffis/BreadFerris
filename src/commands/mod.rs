pub use breadferris::{loadconfig, log, LogType::*};

/*
image: 다양한 api를 이용하여, 이미지를 가져오는 명령어를 모아둔 카테고리입니다.
owner: 봇 주인만 쓸 수 있는 명령어를 모아둔 카테고리입니다.
util: 사실상, 메인 명령어가 있는 카테고리입니다.
other: 어떤 카테고리에 넣을지 모르거나, 애매한 명령어를 모아둔 카테고리입니다.
moderator: 서버의 관리자들이 사용할 수 있는 명령어(킥, 밴, 언밴 등)를 모아둔 카테고리입니다.
fun: 재미있는(?) 기능을 모아둔 카테고리입니다.
*/

pub mod fun;
pub mod image;
pub mod moderator;
pub mod other;
pub mod owner;
pub mod util;

/*
owner 카테고리에 있는 eval과 util 카테고리에 있는 run 의 차이점:

eval은 로컬에서 코드가 실행됩니다. 그렇기때문에, 컴퓨터에 접근할 수 있습니다.
보안이 필요하기때문에 봇의 주인만 사용할 수 있습니다.
* 잘못 접근하면, 불이익을 가져올 수 있기때문에, 사용하지 않는것을 추천드립니다.
* * eval 코드 제공: https://github.com/stephaneyfx/everust (저작권 관련 문제가 생길시, 바로 삭제조치 하겠습니다.)

반면, run은 로컬이 아닌 러스트 플레이그라운드 (https://play.rust-lang.org/)에서 코드를 실행하기 때문에, 안전합니다.
 */

pub const HELP_UTIL: &str = r#"
> `help`, `ping`, `support [Message]`, `dev`, `run [Code]`, `userinfo [mention or id]`, `server_emoji`
"#;

pub const HELP_IMAGE: &str = r#"
> `fox`, `shiba`, `cat`, `meme`
"#;

pub const HELP_OWNER: &str = r#"
> `eval [Code]`, `quit`, `status [Message]`, `nick [Name]`, `msg_del [message id]`
"#;

pub const HELP_MODER: &str = r#"
> `ban [mention or id] [reason]`, `kick [mention or id] [reason]`, `unban [mention or id]`
"#;

pub const HELP_FUN: &str = r#"
> `nitro`, `choice [item, item, . . .]`, `bce [Text]` `say [Text]`
"#;

/*

#[command]
#[aliases("name2", "name3")]
async fn command_name(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.reply(ctx, "Hello, World!").await?;
    Ok(())
}

 */


/// 임베드에서 사용할수 있는 색깔을 모아둔 모듈입니다.
pub mod embed_colors {

}