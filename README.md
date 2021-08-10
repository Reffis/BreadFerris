# Bread Ferris

러스트로 작성된 디스코드 봇입니다.

[초대 (관리자 권한 O)](https://discord.com/api/oauth2/authorize?client_id=785702034388287518&permissions=8&scope=bot)

[초대 (관리자 권한 X)](https://discord.com/api/oauth2/authorize?client_id=785702034388287518&permissions=9&scope=bot)

## 실행

다음 명령어를 입력한후,

```shell
$ git clone https://github.com/fn79/BreadFerris.git
$ cd BreadFerris
```

`config/config.json`를 생성한다음, `config/config.json.example`을 참고하여 설정을 완료합니다.


```shell
$ ./build.bat

or

$ cargo run
```

<br>

> env 사용시, `src/main.rs` 수정 바람. 


# 사용법

접두사: `ferris`

<br>

**Utility**

`ferris ping`: API Latency를 표시합니다.

`ferris help`: 도움말을 표시합니다.

`ferris support [Arg]`: 문의

`ferris run [Code]`: 코드를 실행합니다. 오너 카테고리에 있는 `eval`은 로컬에서 실행되는 반면, 해당 명령어는 러스트 플레이그라운드에서 실행됩니다.

**Owner**

`ferris quit`: 봇 종료

`ferris eval [Arg]`: 로컬에서 러스트 코드 실행 (사용시 주의 바람)

`ferris status [Message]`: 봇 게임 상태를 지정합니다.

`ferris dev`: 개발자의 정보 / 개발 환경을 표시합니다.

**Image**

`ferris fox`: 여우들의 사진을 보여줍니다.

`ferris shiba`: 시바견의 사진을 보여줍니다.

`ferris cat`: 고양이의 사진을 보여줍니다.

`ferris meme`: 밈을 보여줍니다.

**Moderator** (특정 권한 또는 관리자만 사용가능)

`ferris ban [id 또는 멘션] [사유]`: 유저를 차단합니다.

`ferris kick [id 또는 멘션] [사유]`: 유저를 추방합니다.

`ferris unban [id 또는 멘션]`: 차단된 유저를 차단 해제합니다.

**Fun**

`ferris nitro`: 랜덤적으로 니트로 코드를 출력합니다. (fake)

`ferris choice [item, item, . . .]`: 아이템을 고릅니다. (`,` 으로 분간)

> (더 추가될 예정)

<br><br>

> **호스팅은 안할건가요?**
> 
> * **호스팅은 생각 없습니다.** 

 
> **봇이 응답하지 않아요.**
> 
> * **명령어를 잘못 입력했거나, 권한 등이 부족할 경우 봇이 응답하지 않습니다.**
> 명령어를 제대로 입력하고, 권한도 있는데 봇이 응답하지 않을경우, 이슈를 넣어주시기 바랍니다.

<br>

> **소스의 일부분을 가져가실때는 반드시 원작자 표시를 해주시기 바랍니다.**