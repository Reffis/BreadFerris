# Bread Ferris

러스트로 작성된 디스코드 봇입니다.

[초대 (관리자 권한 O)](https://discord.com/api/oauth2/authorize?client_id=785702034388287518&permissions=8&scope=bot)

[초대 (관리자 권한 X)](https://discord.com/api/oauth2/authorize?client_id=785702034388287518&permissions=0&scope=bot)

## 실행

---
다음 명령어를 입력한후,

```shell
$ git clone https://github.com/Reffis/BreadFerris.git
$ cd BreadFerris
```
---
`config/config.toml`를 생성한다음, `config/config.toml.example`을 참고하여 설정을 완료합니다. 
---
**빌드:**

```shell
$ ./build.bat

or

$ cargo run

$ cargo run -- --readylog # 실행시, 다양한 로그를 표시합니다.
```
---
> env 사용시, `src/main.rs` 수정 바람. 

# 사용법

접두사: `ferris`

<br>

**Utility**
---

`ferris ping`: API Latency를 표시합니다.

`ferris help`: 도움말을 표시합니다.

`ferris support [Arg]`: 문의

`ferris rust [Code]`: 러스트 코드를 실행합니다.

`ferris go [Code]`: 고랭(go) 코드를 실행합니다.

`ferris dev`: 개발자의 정보 / 개발 환경을 표시합니다.

`ferris userinfo [멘션 또는 id]`: 유저의 정보를 표시합니다.

`ferris server_emoji`: 서버에 있는 이모지를 가져옵니다. (주의: 이모지가 너무 많으면 출력되지 않을수도 있습니다.)

`ferris invite`: 봇 초대링크를 표시합니다.

**Owner**
---

`ferris quit`: 봇 종료

`ferris status [Message]`: 봇 게임 상태를 지정합니다.

`ferris nick [Name]`: 서버에 있는 봇의 이름을 변경합니다.

`ferris msg_del [id]`: 해당 채널에 있는 봇의 메세지를 삭제합니다.

`ferris announcements [Message]`: 채널 주제에 `-FerrisAnnouncements` 가 포함되어있다면, 해당 채널에 공지 메세지를 보냅니다.

`ferris status_update`: 상태를 업데이트 합니다.

**Image**
---

`ferris fox`: 여우의 사진을 보여줍니다.

`ferris shiba`: 시바견의 사진을 보여줍니다.

`ferris cat`: 고양이의 사진을 보여줍니다.

`ferris meme`: 밈을 보여줍니다.

`ferris neko [NekoType]`: `nekos.life`에서 이미지를 가져옵니다. `(nsfw)`, `NekoTypep`은 `neko help`를 통해 확인하세요.

`ferris corgi`: 웰시 코기 이미지를 보여줍니다.

**Moderator** (특정 권한 또는 관리자만 사용가능)
---

`ferris ban [id 또는 멘션] [사유]`: 유저를 차단합니다.

`ferris kick [id 또는 멘션] [사유]`: 유저를 추방합니다.

`ferris unban [id 또는 멘션]`: 차단된 유저를 차단 해제합니다.

**Fun** 카테고리는 입력하지 않을예정입니다.
* 명령어를 확인하려면, `src/commandss/mod.rs`를 참고해주세요.
---

**기타** (명령어가 아닌, 채널 주제 등으로 사용하는 기능) **(Betas)**
---

`-NoInviteLink`: 초대 링크를 보내면, 자동으로 삭제합니다. (메세지 관리 권한 필요)

---
> (더 추가될 예정)

<br><br>

---
> **호스팅은 안할건가요?**
> 
> * **호스팅은 생각 없습니다.** 
---
> **봇이 응답하지 않아요.**
> 
> * **명령어를 잘못 입력했거나, 권한 등이 부족할 경우 봇이 응답하지 않습니다.**
> 명령어를 제대로 입력하고, 권한도 있는데 봇이 응답하지 않을경우, 이슈를 넣어주시기 바랍니다.
---
> **소스의 일부분을 가져가실때는 반드시 원작자 표시를 해주시기 바랍니다.**
---