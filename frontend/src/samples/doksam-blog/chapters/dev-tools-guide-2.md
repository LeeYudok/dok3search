# 요즘 공부하는 것...들 2탄 — 터미널로 옮겨 온 도구, 함정은 셸과 판 차이에서 났다

> 편집기 대신 터미널에서 일하면 무엇을 쓰고, 어디서 걸릴까?

답부터 적습니다. 에이전트 하나와 그 에이전트가 부르는 작은 명령 열 남짓이면 됩니다. 걸린 곳은 도구의 결함이 아니라 환경 차이였습니다. 대화형 셸에서만 드는 별칭, zsh 가 패턴으로 읽는 `?`, 로컬과 CI 에서 다른 pnpm 판, 원격 셸의 자기 매칭이 그것입니다.
하루 뒤 다시 확인하니 목록은 그대로였고 Claude Code 만 한 판 올랐습니다.

> [그림] 왼쪽 공구판에 rg, fd, jq, gh, glab, pnpm, uv, podman 이 걸려 있고 그 아래에 셸 별칭, zsh 의 물음표, pnpm 판이라고 적힌 주의 표지가 서 있다. 오른쪽에서는 로봇이 터미널 앞에서 명령을 부른다

그림으로 줄이면 이렇습니다. 일은 터미널의 에이전트가 하고, 에이전트는 공구판의 작은 명령을 집어 씁니다. 발에 걸린 것은 공구가 아니라 바닥의 표지판들이었습니다.

[1탄](https://blog.doksam.com/blog/dev-tools-guide/)은 Windows 와 폐쇄망의 GUI 도구였습니다. 여덟 달 사이 손이 가는 곳이 편집기에서 터미널로 옮겨 와 2탄을 씁니다. 버전은 2026-11-17 에 제 맥(M4 Pro)에서 `--version` 으로 읽었습니다. 2026-11-18 에 한 번 더 읽었습니다. 셸 설정은 제 dotfiles([agents-scaffold](https://github.com/LeeYudok/agents-scaffold) 의 `dotfiles/`)와 대조했습니다.

## 매일 쓰는 도구는 열 가지이고, 하루 사이 바뀐 것은 한 판입니다

| 도구 | 2026-11-17 | 2026-11-18 | 용도 |
|---|---|---|---|
| Claude Code | 2.1.325 | 2.1.326 | 터미널에서 도는 AI 코딩 에이전트 |
| Git | 2.54.0 | 같음 | 버전 관리 |
| gh / glab | 2.96.0 / 1.99.0 | 같음 | GitHub, GitLab 을 터미널에서 다룬다 |
| ripgrep (`rg`) / fd | 15.2.0 / 10.4.2 | 같음 | 내용 검색 / 파일 이름 찾기 |
| jq | 1.7.1 | 같음 | JSON 가공 |
| pnpm / Bun | 12.21.0 / 1.4.2 | 같음 | JS 패키지 관리와 실행 |
| uv | 0.12.5 | 같음 | 파이썬 도구를 설치 없이 실행 |
| Podman | 6.1.4 | 같음 | 컨테이너 |
| Rust (`rustc`, `cargo`) | 1.100.0 | 같음 | 메모리를 적게 쓰는 서버와 CLI |

있으면 편한 도구도 같은 날 확인했습니다. 처음 올린 글의 표에서 비어 있던 `eza` 와 `shellcheck` 의 버전을 채웠습니다.

| 도구 | 2026-11-18 | 용도 |
|---|---|---|
| `eza` / `bat` | 0.23.4 / 0.26.1 | `ls` 와 `cat` 대신 |
| `fzf` / `zoxide` | 0.74.4 / 0.9.9 | 목록에서 골라 찾기 / 자주 가는 폴더로 이동 |
| `ccusage` | 20.0.20 | Claude Code 토큰 사용량 집계 |
| `codegraph` | 1.6.2 | 저장소의 심볼과 호출 관계 색인 |
| `age` / `autossh` | 1.3.2 / 1.4g | 파일 암호화 / 끊기면 다시 붙는 SSH 터널 |
| `shellcheck` | 0.11.0 | 셸 스크립트 검사 |

## Claude Code 는 규칙을 파일에, 막을 것은 훅에 둡니다

1탄에서는 VS Code 의 권장 확장이었습니다. 지금은 터미널에서 CLI 로 씁니다. 저장소에서 바로 읽고, 고치고, 빌드하고, 올립니다. 써 보고 정착한 방식은 셋입니다.

- 전역 규칙은 `~/.claude/CLAUDE.md` 에, 저장소 규칙은 `AGENTS.md` 에 적습니다. 세션이 바뀌어도 같은 규칙으로 시작합니다.
- force push 나 `reset --hard` 는 실행 전 훅이 거부합니다. 문서에 "하지 마라"고 적는 것으로는 부족했습니다. 자세한 내용은 [AI 코딩 에이전트가 규칙을 못 어기게 만드는 법](https://blog.doksam.com/blog/agents-scaffold-commit-gate/)에 썼습니다.
- 끝났다는 보고에는 실행 링크와 응답 코드를 같이 받습니다.

에이전트는 틀린 내용도 맞는 내용과 같은 말투로 말합니다. 그래서 수치, 날짜, 출처는 원문과 대조합니다. 이 글을 옮기면서도 버전 하나(Claude Code)가 하루 만에 낡아 있었습니다.

## 함정 여섯은 셸과 판, 캐시의 차이에서 났습니다

| 함정 | 증상 | 대처 |
|---|---|---|
| 셸 별칭 | 스크립트 안의 `cat` 은 별칭이 아니라 원래 `cat` 이다 | 터미널에서 원래 명령이 필요하면 `command cat` |
| zsh 의 `?` | 따옴표 없는 `gh api …commits?per_page=10` 이 `zsh: no matches found` 로 끝난다 | 주소를 따옴표로 감싼다 |
| pnpm 판 | 로컬 pnpm 12 는 `ERR_PNPM_IGNORED_BUILDS` 로 실패, CI 의 pnpm 9 는 그대로 설치 | CI 에 판을 적고, 결과가 다르면 판부터 본다 |
| 원격 `pkill -f` | 원격 셸의 명령줄에도 찾는 글자가 있어 셸이 같이 죽는다 | `"[v]llm serve"` 처럼 대괄호로 자기 매칭을 피한다 |
| 이미지 캐시 | `ENV X=${ARG}` 로 넣은 버전이 바뀌어도 캐시된 층이 쓰인다 | 버전은 파일에 적고 `COPY` 로 넣는다 |
| `ccusage` 수치 | 실행 중인 세션이 로그를 계속 써서 두 번 돌린 값이 다르다 | 로그 폴더 사본을 읽힌다 |

pnpm 은 근거를 다시 찾았습니다. 2026-11-18 의 [pnpm 문서](https://pnpm.io/settings/build)는 `strictDepBuilds` 의 기본값을 `true` 로 적습니다. 검토하지 않은 빌드 스크립트가 있으면 0 이 아닌 종료 코드로 끝난다는 설명도 있습니다. 블로그 저장소의 배포 설정은 지금도 `pnpm/action-setup` 에 `version: 9` 를 적고 있습니다.

이미지 캐시는 이 사이트의 `Dockerfile` 주석에도 같은 내용이 있습니다. "buildah 는 ENV 줄을 글자 그대로 캐시해 값이 안 바뀐다."

별칭은 dotfiles 의 `zsh/zshrc.macos` 에 있는 그대로입니다. 2탄을 쓸 때 옮긴 세 줄과 같습니다. 하나 더 있는 것은 `zoxide init zsh --cmd cd` 입니다. 그래서 이 맥에서는 `cd` 가 zoxide 로 바뀌어 있습니다.

```bash
alias ls="eza --icons --group-directories-first"
alias ll="eza -l --icons --git --sort=modified"
command -v bat >/dev/null && alias cat="bat --paging=never"
```

원격 `pkill` 과 `ccusage` 사본은 명령으로 보는 편이 빠릅니다.

```bash
ssh myserver 'pkill -f "vllm serve"; echo done'      # 원격 셸이 같이 죽는다
ssh myserver 'pkill -f "[v]llm serve"; echo done'    # 대괄호로 자기 매칭을 피한다

cp -R ~/.claude /tmp/claude-snapshot
CLAUDE_CONFIG_DIR=/tmp/claude-snapshot ccusage daily
```

## 서버는 Quadlet 과 sops 로 다시 만들 수 있게 둡니다

서버의 컨테이너는 루트 권한 없이 Podman 으로 돌리고, 서비스 등록은 Quadlet 으로 합니다. `.container` 파일을 두면 systemd 서비스가 됩니다.

```ini
# ~/.config/containers/systemd/myapp.container
[Container]
Image=localhost/myapp:local
PublishPort=127.0.0.1:8080:8080
EnvironmentFile=%h/app/.env

[Service]
Restart=always

[Install]
WantedBy=default.target
```

`.env` 를 서버에만 두면 서버를 다시 만들 때 값을 잃습니다. 그래서 `sops` 와 `age` 로 값만 암호화한 파일을 저장소에 둡니다. 키 이름은 평문으로 남아 무엇이 바뀌었는지 diff 로 보입니다. 개인 키는 저장소에 넣지 않고 따로 백업합니다. 이 키를 잃으면 암호화한 파일을 모두 열 수 없습니다.

```bash
sops -e --input-type dotenv --output-type dotenv .env > deploy/secrets/app.env.sops
autossh -M 0 -N -L 5432:127.0.0.1:5432 myserver   # 집 서버 DB 는 터널로만 붙는다
```

## 정적 사이트는 다음 날 Hugo 로 옮겼습니다

2탄을 쓴 날 두 정적 사이트의 빌드 시간을 쟀습니다.

| | 쪽 수 | 빌드 시간 (2026-11-17) |
|---|---|---|
| Astro 5.16.6 (옛 블로그) | 80쪽 | 0.95초 |
| Hugo 0.170.0 (테마 예제 사이트) | 45쪽 | 0.021초 |

내용과 쪽 수가 달라서 같은 조건의 비교가 아닙니다. 자릿수가 다르다는 정도만 읽으면 됩니다. 이 값은 다시 재지 않았습니다.

그날 저녁 이 사이트가 Hugo 와 작은 Go 서버로 바뀌었습니다. 테마를 따로 떼어 낸 과정은 [테마는 별도 저장소에, 사이트에는 글과 설정만](https://blog.doksam.com/blog/astro-theme-as-integration/)에 적었습니다. 옛 블로그는 Pagefind 로 검색을 만듭니다. 이 사이트는 검색을 넣지 않아서 컨테이너 빌드에 Pagefind 단계가 없습니다.

## Rust 서버는 Java 보다 메모리 바닥이 30배 넘게 낮았습니다

메모리값이 오르면 Java 는 부자만 쓴다는 농담이 있습니다. 배경은 TrendForce 의 2026-01-05 [전망](https://www.trendforce.com/presscenter/news/20260105-12860.html)입니다. 2026년 1분기 범용 D램 계약 가격이 전 분기보다 55~60% 오른다는 내용입니다. 전망치이고 계약 가격이라 소매 가격과는 다릅니다.

요청을 받으면 `ok` 를 돌려주는 HTTP 서버를 두 언어의 표준 라이브러리로 썼습니다. 2026-11-17 에 3회씩 쟀고, 메모리는 `ps` 의 RSS(프로세스가 실제로 잡은 메모리)입니다.

| | 대기 중 | 요청 2,000회 뒤 | 첫 응답까지 |
|---|---|---|---|
| Rust 1.100.0 (`rustc -O`) | 1.5MB | 1.5MB | 5~10ms |
| Java 21.0.11 (기본 옵션) | 51.4~51.8MB | 126.9~130.1MB | 77~95ms |
| Java 21.0.11 (`-Xmx64m -XX:+UseSerialGC -Xss256k`) | 49.2~49.3MB | 80.1~82.6MB | 87~102ms |

대기 중에는 약 34배, 요청을 받은 뒤에는 약 85배 차이가 났습니다. 해석은 이렇습니다. Java 가 그만큼 필요한 것은 아닙니다. 기본 JVM 은 힙 상한을 물리 메모리의 1/4(이 맥에서 12GB)로 잡아 치우지 않고 쌓아 둡니다. 그래도 아무 일도 안 하는 JVM 이 약 50MB 를 잡는 바닥은 남습니다. 512MB 서버라면 위 값으로 JVM 4~6개가 들어가고, Rust 서버는 수십 개가 들어갑니다.

이 비교가 틀리는 조건도 적어 둡니다. 프레임워크를 올리지 않았고, 요청을 하나씩 보내 동시 처리량은 재지 않았습니다. 팀이 이미 Java 로 일한다면 메모리값보다 사람의 시간이 비쌉니다. Rust 와 Go 를 어디에 쓰는지는 [Go 와 Rust 를 쓰면 좋은 점](https://blog.doksam.com/blog/why-go-and-rust/)에 따로 정리했습니다.

## 1탄에서 바뀐 것과 지금 남은 것

2026-11-18 의 이 맥과 dotfiles 로 다시 나눴습니다.

| 구분 | 도구 | 근거 |
|---|---|---|
| 지금도 쓴다 | 위 표의 도구 전부 | 2026-11-18 `--version`. dotfiles `install.sh` 가 eza·bat·zoxide·fzf·fd·jq 를 깔고 `zshrc.macos` 가 별칭을 건다 |
| 자리를 내줬다 | VS Code 확장으로 쓰던 Claude Code | 확장(2.1.323)은 남아 있지만 일은 CLI(2.1.326)로 한다 |
| 자리를 내줬다 | TortoiseGit 같은 GUI, DBeaver 로 SQLite 조회 | `gh`·`glab`, SSH 터널로 서버 DB 에 붙는다 |
| 새로 쓴다 | starship 1.26.0, zsh 자동 제안·구문 강조 플러그인 | dotfiles `install.sh` 1단계가 깐다 |
| 새로 쓴다 | Codex CLI 0.169.0, Antigravity CLI(`agy`) 1.5.10 | dotfiles 가 두 도구의 상태 표시줄을 맞춘다. [Codex 가이드](https://blog.doksam.com/blog/codex-guide-01-getting-started/), [agy 가이드](https://blog.doksam.com/blog/agy-guide-01-getting-started/) |
| 새로 쓴다 | Go 1.26.2, Hugo 0.170.0 | 이 사이트의 서버와 화면 |

해석은 이렇습니다. 2탄 표에는 에이전트가 Claude Code 하나뿐입니다. 그런데 dotfiles 는 2026-10-24 부터 Codex 와 agy 의 상태 표시줄도 맞추고 있었습니다. 공구판은 그대로이고, 그것을 집는 손은 글보다 먼저 늘어 있었습니다.

이 판단이 틀리는 경우도 있습니다.

- Windows 와 폐쇄망에서는 1탄의 GUI 도구가 맞습니다. 이 글은 인터넷이 되는 맥의 목록입니다.
- 버전 비교는 하루 간격입니다. "바뀐 것이 한 판"이라는 말은 이틀의 사실일 뿐, 도구가 안정됐다는 근거는 아닙니다.
- 빌드 시간과 메모리는 2026-11-17 하루 측정값입니다.

## 정리

터미널에서 일하면 에이전트 하나와 작은 명령 열 남짓으로 충분합니다. 시간을 잡아먹은 것은 도구가 아니라 셸, 판, 캐시의 차이였습니다.

다음에 볼 것은 하나입니다. 블로그 배포 설정의 pnpm 9 를 로컬과 같은 판으로 맞춥니다. 그때 로컬의 설치 실패가 CI 에서도 똑같이 나는지 확인합니다.

이 글은 2026-11-17 에 옛 블로그에 올린 글을 2026-11-18 에 다시 확인하고 고쳐 쓴 것입니다.
