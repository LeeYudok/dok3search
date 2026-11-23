# 요즘 공부하는 것...들 — 폐쇄망 Windows 에 들고 들어가는 도구 다섯, 여덟 달 뒤

> 인터넷이 안 되는 Windows PC 에서 웹 개발을 하려면 무엇을 들고 들어가야 할까?

답부터 적습니다. 다섯 가지면 일이 됩니다. Bun, Git, VS Code, DBeaver, Beyond Compare 입니다. 고르는 기준은 반입한 뒤 인터넷 없이 돌아가는가, 되도록 설치 없이 풀어 쓰는 판이 있는가입니다.
여덟 달 뒤 다시 보니 이 목록은 폐쇄망 Windows 에서 그대로 맞습니다. 인터넷이 되는 맥에서는 터미널 도구가 그 자리를 가져갔습니다.

> [그림] 인터넷 PC 에서 받은 도구 상자 다섯 개를 로봇이 손수레에 싣고 폐쇄망 울타리 문을 지나 Windows PC 로 옮긴다. 인터넷 선은 울타리에서 끊겨 있다

그림으로 줄이면 이렇습니다. 도구는 울타리 밖 인터넷 PC 에서 받고, 손수레로 한 번에 들고 들어갑니다. 안에서는 내려받을 수 없으니 빠뜨린 것이 없어야 합니다.

이 글은 2026-03-21 에 표만 있던 목록을 고쳐 쓴 것입니다. 버전과 링크는 2026-11-18 에 공식 문서, 제 맥(M4 Pro), 제 dotfiles 저장소로 다시 확인했습니다. Windows 전용 도구는 이번에 Windows PC 에서 직접 다시 깔아 보지 않았습니다.

## 필수 다섯은 반입한 뒤 인터넷 없이 돕니다

| 도구 | 용도 | 반입 형태 | 2026-11-18 최신 |
|---|---|---|---|
| Bun | JS/TS 런타임과 패키지 관리. 백엔드 실행 | 실행 파일 하나가 든 `bun-windows-x64.zip` | 1.4.4 (2026-10-22) |
| Git | 버전 관리 | 설치형 `Git-…-64-bit.exe` 또는 `PortableGit-…7z.exe` | Git for Windows 2.56.1 (2026-11-10) |
| VS Code | 편집기 | ZIP 판을 풀고 `data` 폴더를 만들면 포터블 모드 | — |
| DBeaver Community | SQLite 조회 GUI | Windows ZIP 판 | 26.2.5 (2026-11-15) |
| Beyond Compare | 파일·폴더 비교와 병합 | 설치 파일. 포터블 판은 확인하지 못했다 | 5.2.8 (2026-11-10) |

1탄을 쓸 때 적은 최소 버전은 Bun 1.3.10, Git 2.40 이었습니다. 지금 최신과 견주면 여유가 있습니다.

VS Code 의 포터블 모드는 Windows ZIP 판에서만 됩니다. [공식 문서](https://code.visualstudio.com/docs/editor/portable)는 "Portable mode is only supported on the Windows ZIP (`.zip`) archive" 라고 적습니다. 압축을 푼 폴더 안에 `data` 폴더를 만들면 설정과 확장이 그 안에 쌓입니다. 폴더째 USB 에 담아 옮기면 확장까지 같이 갑니다.

선택 도구는 넷입니다.

| 도구 | 용도 |
|---|---|
| TortoiseSVN | SVN GUI. 탐색기 오른쪽 클릭 메뉴로 쓴다 |
| TortoiseGit | Git GUI. Git for Windows 가 먼저 깔려 있어야 한다 |
| `jq` | 명령줄에서 JSON 가공 |
| 7-Zip | 반입 파일 압축과 해제. `PortableGit` 도 7-Zip 형식 자동 해제 파일이다 |

TortoiseSVN 은 설치할 때 **command line client tools** 를 체크해야 `svn` 명령이 같이 깔립니다.

## VS Code 확장은 셋만 필수로 둡니다

| 구분 | 확장 | ID | 용도 |
|---|---|---|---|
| 필수 | Prettier | `esbenp.prettier-vscode` | 저장할 때 포맷(2칸 들여쓰기, LF) |
| 필수 | ESLint | `dbaeumer.vscode-eslint` | 프런트엔드 린트 |
| 필수 | Tailwind CSS IntelliSense | `bradlc.vscode-tailwindcss` | 클래스 자동 완성 |
| 권장 | Claude Code | `anthropic.claude-code` | 앤트로픽의 AI 코딩 에이전트 |
| 권장 | GitLens | `eamodio.gitlens` | 줄마다 마지막 커밋 보기 |
| 권장 | Error Lens | `usernamehw.errorlens` | 오류를 그 줄 끝에 표시 |
| 권장 | Active File In StatusBar | `RoscoP.ActiveFileInStatusBar` | 상태 표시줄에 현재 파일 경로 |

폐쇄망에서는 확장 마켓에 붙지 못합니다. 인터넷 PC 의 포터블 VS Code 에 확장을 깔고 `data` 폴더째 가져가면 따로 설치할 것이 없습니다. Claude Code 확장은 모델 API 에 닿아야 쓸 수 있습니다. 폐쇄망 안에 모델 서버가 없다면 이 확장은 빼고 들고 갑니다.

## DBeaver 는 서버가 켜져 있으면 읽기만 합니다

SQLite 연결은 세 단계입니다. 데이터베이스 → 새 데이터베이스 연결에서 SQLite 를 고르고, 파일 경로를 지정하고, 연결을 시험합니다.

주의할 것은 서버가 같은 파일을 쓰고 있을 때입니다. [SQLite 문서](https://www.sqlite.org/wal.html)의 두 문장이 근거입니다.

- WAL 모드에서는 읽기와 쓰기가 서로 막지 않습니다. 그러나 "there can only be one writer at a time", 쓰는 쪽은 한 번에 하나입니다. DBeaver 에서 고치는 동안 서버의 쓰기는 기다리거나 `SQLITE_BUSY` 로 실패할 수 있습니다.
- `-wal` 파일은 데이터베이스의 일부입니다. 문서는 데이터베이스 파일과 WAL 파일을 떼어 놓으면 커밋한 트랜잭션을 잃거나 파일이 깨질 수 있다고 적습니다. `doksam.db-wal`, `doksam.db-shm` 같은 파일은 지우지 않습니다.

그래서 규칙은 하나입니다. 서버가 도는 동안 DBeaver 에서는 조회만 합니다.

## Beyond Compare 는 Git 에 이름만 알려 주면 됩니다

용도는 셋입니다. 고치기 전후 파일 비교, 배포 전 빌드 결과 폴더와 서버 폴더의 동기화 확인, 병합 충돌 해결입니다.

1탄에서는 Git 설정에 실행 파일 경로까지 적었습니다. 2026-11-18 에 [Scooter Software 의 안내](https://www.scootersoftware.com/kb/vcs)를 다시 봤습니다. Git for Windows 에는 아래 두 줄만 있고 경로 줄은 없습니다.

```bash
git config --global diff.tool bc
git config --global merge.tool bc   # 3-way 병합은 Pro 판만
```

Git 2.2 보다 오래된 판에서는 `bc` 대신 `bc3` 을 씁니다. 3-way 병합은 Pro 판에서만 된다고 안내에 적혀 있습니다. 1탄에는 이 조건이 빠져 있었습니다. 실행은 `git difftool`, `git mergetool` 입니다.

TortoiseSVN 은 설정 → Diff Viewer → External 에 아래 줄을 넣습니다. 안내의 Beyond Compare 5 기준 값입니다. 1탄과 달리 왼쪽 원본을 읽기 전용으로 여는 `/leftreadonly` 가 붙어 있습니다.

```text
"C:\Program Files\Beyond Compare 5\BComp.exe" %base %mine /title1=%bname /title2=%yname /leftreadonly
```

Merge Tool 은 `%mine %theirs %base %merged` 와 제목 넷을 넘기는 1탄의 줄이 안내와 같습니다. TortoiseGit 도 같은 메뉴에서 같은 값을 넣습니다.

1탄은 "30일 평가판"이라고 적었습니다. 2026-11-18 의 내려받기 화면에는 평가판 기간이 적혀 있지 않아 그 문장은 뺐습니다. 유료 프로그램이고 평가판을 내려받을 수 있다는 점까지만 확인했습니다.

## 여덟 달 뒤, 폐쇄망 목록은 남고 맥에서는 터미널로 옮겨 갔습니다

2026-11-18 에 제 맥에서 `command -v`, `--version`, `/Applications`, VS Code 확장 폴더를 확인했습니다. dotfiles 는 [agents-scaffold](https://github.com/LeeYudok/agents-scaffold) 의 `dotfiles/` 를 봤습니다.

| 구분 | 도구 | 근거 |
|---|---|---|
| 지금도 쓴다 | Git 2.54.0, jq 1.7.1, 7-Zip(`7zz`) 26.02 | 이 맥의 `--version` |
| 지금도 쓴다 | Bun 1.4.2 | 설치돼 있다. 다만 배포하는 프로젝트의 패키지 관리는 pnpm 12.21.0 으로 옮겼다 |
| 지금도 깔려 있다 | VS Code 1.146.0, DBeaver 26.2.0 | `/Applications` 의 앱 버전 |
| 지금도 깔려 있다 | 확장 Prettier, ESLint, Error Lens, Claude Code(2.1.323) | VS Code 확장 폴더 |
| 이 맥에서는 뺐다 | Tailwind CSS IntelliSense, GitLens, Active File In StatusBar, Beyond Compare | 확장 폴더와 `/Applications` 에 없다 |
| 맥에서는 대신 쓴다 | `svn` 1.14.5 명령 | TortoiseSVN·TortoiseGit 은 Windows 전용이다 |
| 새로 쓴다 | Claude Code CLI 2.1.326, 터미널 도구 | [2탄](https://blog.doksam.com/blog/dev-tools-guide-2/)에 정리했다 |
| 새로 쓴다 | 폐쇄망 Git 설치 스크립트 `windows/install-git.ps1` | dotfiles. Git for Windows 를 관리자 권한 없이 오프라인 설치하고 Git Bash 를 맞춘다 |

해석은 이렇습니다. 폐쇄망 Windows 의 목록은 그대로 두고, 그 목록을 들고 들어가는 일을 스크립트로 바꿨습니다. dotfiles 의 Windows 설치 스크립트는 반입한 Git 설치 파일의 서명을 확인한 뒤 설치합니다. 반입 파일의 SHA-256 을 출력해 릴리스 페이지 값과 대조하게 합니다. 1탄의 "Git 2.40+" 한 줄이 반입 절차 하나로 자랐습니다.

맥 쪽은 다릅니다. 같은 Claude Code 를 VS Code 확장(2.1.323)이 아니라 터미널의 CLI(2.1.326)로 씁니다.

이 판단이 틀리는 경우도 있습니다.

- "깔려 있다"와 "쓴다"는 다릅니다. VS Code 와 DBeaver 가 이 맥에 있다는 것만 확인했고, 얼마나 자주 여는지는 재지 않았습니다.
- 폐쇄망 Windows PC 에서 다섯 도구가 지금도 같은 판으로 도는지는 이번에 확인하지 못했습니다. 그 PC 의 반입 목록을 보면 이 표는 바뀔 수 있습니다.

## 정리

폐쇄망 Windows 에서 웹 개발을 하려면 Bun, Git, VS Code, DBeaver, Beyond Compare 다섯을 반입합니다. 되도록 ZIP·포터블 판으로 가져가고, VS Code 는 확장까지 `data` 폴더째 옮깁니다. 여덟 달 동안 이 목록은 바뀌지 않았고, 바뀐 것은 맥에서 일하는 방식입니다.

다음에 볼 것은 하나입니다. 폐쇄망 PC 의 Beyond Compare 가 Pro 판인지 확인합니다. Standard 판이면 `git mergetool` 의 3-way 병합이 되지 않습니다.

## 참고

- [VS Code: Portable mode](https://code.visualstudio.com/docs/editor/portable)
- [SQLite: Write-Ahead Logging](https://www.sqlite.org/wal.html)
- [Beyond Compare: Using Beyond Compare with Version Control Systems](https://www.scootersoftware.com/kb/vcs)
- 내려받기: [VS Code](https://code.visualstudio.com/download), [DBeaver](https://dbeaver.io/download/), [Beyond Compare](https://www.scootersoftware.com/download), [Git for Windows](https://git-scm.com/downloads/win), [TortoiseSVN](https://tortoisesvn.net/downloads.html), [TortoiseGit](https://tortoisegit.org/download/), [7-Zip](https://www.7-zip.org/download.html)

이 글은 2026-03-21 에 옛 블로그에 올린 글을 2026-11-18 에 다시 확인하고 고쳐 쓴 것입니다.
