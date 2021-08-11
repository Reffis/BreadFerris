@echo off

:: Cargo.toml
::
::[profile.dev]
::opt-level = 0 # 일반적으로 사용시에는 opt-level을 0으로 설정하는것이 좋습니다.
::[profile.release]
::opt-level = 3 # 릴리즈 빌드시에는, opt-level을 3으로 설정하는것이 좋습니다.

::opt-level은 러스트가 프로젝트를 최적화 하는 단계를 말합니다. (0 ~ 3)
::최적화 단계가 올라갈수록, 컴파일 속도가 느려지며, 프로그램의 성능이 좋아집니다.

::0은 일반적인 단계입니다. 컴파일 속도가 일반적이며, 성능 또한 일반적입니다.

::3은 릴리즈 단계로, 컴파일 속도가 느립니다. 하지만, 프로그램의 성능이 좋아집니다.

::일반적인 빌드
::::: cargo build

::릴리즈 빌드
::::: cargo build --release

:: 참고: https://doc.rust-lang.org/book/ch14-01-release-profiles.html

cargo build --release

pause