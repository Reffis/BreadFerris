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

`ferris eval [Arg]`: 러스트 코드 실행

`ferris status [Message]`: 봇 게임 상태를 지정합니다.

`ferris dev`: 개발자의 정보 / 개발 환경을 표시합니다.

**Image**

`ferris fox`: 여우들의 사진을 보여줍니다.

`ferris shiba`: 시바견의 사진을 보여줍니다.

`ferris cat`: 고양이의 사진을 보여줍니다.

`ferris meme`: 밈을 보여줍니다.

> (더 추가될 예정)

<br><br>

> **호스팅은 안할건가요?**
> 
> * **오픈소스를 위주로 개발중입니다. 그러므로, 호스팅은 생각 없습니다.**