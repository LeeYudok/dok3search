# Codex 실전 가이드 1편 — 시작하기와 AGENTS.md

Codex 실전 가이드 1편입니다. 같은 사이트의 「Claude Code 실전 가이드」 5부작과 짝을 이루는 연재입니다. 독자는 그 글을 읽은 분으로 잡았습니다. 기준 버전은 codex-cli 0.169.0 입니다. 명령과 동작은 2026-11-18 에 맥에서 직접 실행해 확인했습니다. 공식 문서로만 확인한 내용은 따로 표시합니다.

이번 편이 답하는 질문은 하나입니다. **Codex 에게 프로젝트 규칙을 매번 설명하지 않으려면 어디에 무엇을 두어야 하나.** 답은 AGENTS.md 입니다. Codex 는 git 루트에서 지금 폴더까지 내려오며 AGENTS.md 를 찾습니다. 찾은 파일은 이어 붙여 세션 첫머리에 넣습니다. 실제로 무엇이 들어갔는지는 `codex debug prompt-input` 으로 확인합니다.

## 설치하고 버전부터 적어 둡니다

공식 문서는 macOS·Linux 에 독립 설치 스크립트를 안내합니다.

```bash
curl -fsSL https://chatgpt.com/codex/install.sh | sh   # 공식 문서 기준
codex --version                                        # codex-cli 0.169.0
```

첫 실행에서 로그인 방법을 고릅니다. 그다음 작업할 저장소로 들어가 `codex` 를 실행하면 대화형 화면(TUI)이 열립니다. `codex "이 저장소 구조 설명해줘"` 처럼 첫 질문을 같이 줄 수도 있고, `-C <폴더>` 로 작업 루트를 따로 지정할 수도 있습니다. 설치·설정·인증 상태는 `codex doctor` 가 점검하고, 갱신은 `codex update` 가 맡습니다.

버전을 먼저 적는 이유가 있습니다. Codex 는 동작이 자주 바뀝니다. 0.166.0 에서는 훅에 넘어가는 `cwd` 의 뜻이 바뀌었습니다(4편에서 다룹니다). 이 연재의 표와 수치도 0.169.0 에서만 보장합니다.

## 첫 관문은 프로젝트 신뢰입니다

Codex 는 폴더마다 신뢰 여부를 기억합니다. 기록은 `~/.codex/config.toml` 에 이런 표로 남습니다.

```toml
[projects."/Users/me/workspace/my-repo"]
trust_level = "trusted"
```

신뢰는 생각보다 많은 것을 바꿉니다. 제 설정 파일에는 `sandbox_mode` 를 적지 않았습니다. 그런데 신뢰한 저장소와 처음 보는 폴더에서 Codex 가 받는 기본값이 달랐습니다.

| 항목 | 신뢰한 저장소 | 신뢰 기록이 없는 새 폴더 |
|---|---|---|
| 기본 샌드박스 | `workspace-write` | `read-only` |
| AGENTS.md | 읽음 | 읽음 |
| `.agents/skills` 의 스킬 | 읽음 | 읽음 |
| `.codex/` 의 서브에이전트·훅·설정 | 읽음 | 건너뜀 |

위 세 줄은 두 폴더에서 `codex debug prompt-input` 을 돌려 확인했습니다. 마지막 줄은 공식 문서와 0.160.0 실측을 근거로 적었습니다. 공식 문서의 문장은 "Untrusted projects skip project-scoped `.codex/` layers" 입니다. 정리하면 지침과 스킬은 신뢰와 무관하게 들어가고, 실행에 관여하는 설정만 신뢰를 요구합니다.

신뢰 기록은 쌓이기만 합니다. 이 맥의 `config.toml` 에는 신뢰 항목이 65개 있습니다(2026-11-18). 그중 7개는 다른 세션이 쓰고 버린 임시 폴더였습니다. `codex exec --help` 는 `--trust` 를 "MCP commands outside the sandbox" 까지 포함하는 신뢰라고 설명합니다. 신뢰는 편의 설정이 아니라 권한 결정이니 가끔 정리하는 편이 낫습니다.

## AGENTS.md 는 루트부터 지금 폴더까지 이어 붙습니다

계층 규칙은 직접 재는 편이 빠릅니다. 빈 git 저장소에 표지 문자열이 서로 다른 파일 세 개를 두었습니다.

```text
probe/                       (git 저장소 루트)
├── AGENTS.md                ROOT-MARK
└── sub/
    ├── AGENTS.md            SUB-MARK
    ├── AGENTS.override.md   SUB-OVERRIDE-MARK
    └── deeper/
```

폴더를 바꿔 가며 `codex debug prompt-input` 을 실행하고 표지 문자열을 셌습니다.

| 세션을 연 폴더 | ROOT-MARK | SUB-MARK | SUB-OVERRIDE-MARK |
|---|---|---|---|
| `probe/` | 들어감 | 없음 | 없음 |
| `probe/sub/` | 들어감 | 없음 | 들어감 |
| `probe/sub/deeper/` | 들어감 | 없음 | 들어감 |

이 표에서 세 가지를 읽을 수 있습니다. 첫째, 루트 파일이 먼저 오고 가까운 폴더의 파일이 뒤에 붙습니다. 공식 문서도 뒤에 오는 가까운 파일이 앞의 지침을 덮는다고 설명합니다. 둘째, `AGENTS.override.md` 가 있으면 같은 폴더의 AGENTS.md 는 빠집니다. 셋째, 세션을 연 폴더보다 아래에 있는 파일은 읽지 않습니다.

공식 문서 기준으로 전역 지침은 `~/.codex/AGENTS.override.md`, 없으면 `~/.codex/AGENTS.md` 입니다. 지침 전체의 기본 한도는 32 KiB(`project_doc_max_bytes`)이고, 다른 파일 이름을 더 읽히려면 `project_doc_fallback_filenames` 에 적습니다.

하위 폴더 지침은 세션을 연 위치가 정합니다. 루트에서 연 세션은 하위 폴더의 파일을 읽어도 그 폴더의 지침을 불러오지 않았습니다. 0.160.0~0.166.0 에서 잰 결과입니다. AGENTS.md 안의 `@경로` import 도 펼치지 않습니다. 한 저장소의 AGENTS.md 에 적힌 `@.claude/memory/MEMORY.md` 는 글자 그대로 남았고, 대상 파일의 내용은 들어가지 않았습니다.

## 무엇이 들어갔는지는 prompt-input 으로 봅니다

`codex debug prompt-input` 은 모델에게 보일 입력 목록을 JSON 으로 출력합니다. 모델을 부르지 않으니 비용이 들지 않습니다.

```bash
cd ~/workspace/my-repo
codex debug prompt-input "hello" > /tmp/prompt.json
grep -c '방금 고친 지침의 한 줄' /tmp/prompt.json
```

출력에는 `# AGENTS.md instructions for <폴더>` 로 시작하는 지침 묶음이 있습니다. 작업 폴더·셸·날짜·시간대·쓰기 가능한 경로를 담은 환경 정보, 스킬 목록, 샌드박스 규칙도 함께 나옵니다. 이 연재의 표 대부분은 이 명령으로 얻었습니다. 지침을 고쳤다면 한 줄을 골라 grep 해 보는 습관을 권합니다.

## 초안은 /init 으로 만들고 내용은 직접 채웁니다

공식 문서의 CLI 슬래시 명령 목록에는 `/init` 이 있습니다. 현재 폴더에 AGENTS.md 뼈대를 만드는 명령이고, Claude Code 의 `/init` 과 역할이 같습니다. 다만 이 글을 쓰면서 `/init` 의 결과물을 직접 확인하지는 않았습니다.

뼈대가 어떻게 나오든 채울 내용의 기준은 Claude Code 가이드 1편과 같습니다. 행동을 바꾸는 규칙, 그리고 몰라서 틀리면 안 되는 사실만 남깁니다. Codex 에서는 기준이 하나 더 붙습니다. 전역·저장소·하위 폴더 지침을 합친 크기에 32 KiB 한도가 있습니다. 소개글과 배경 설명은 Claude Code 에서보다 더 아껴야 합니다.

## Claude Code 와 다른 점 — 파일 이름보다 로드 시점이 다릅니다

Claude Code 가이드 1편의 CLAUDE.md 와 하는 일은 같습니다. 차이는 언제, 어떤 파일을 읽느냐에 있습니다.

| 항목 | Claude Code | Codex 0.169.0 |
|---|---|---|
| 기본 지침 파일 | CLAUDE.md. AGENTS.md 도 읽지만 CLAUDE.md 가 있으면 건너뜀 | AGENTS.md |
| 전역 지침 | `~/.claude/CLAUDE.md` | `~/.codex/AGENTS.md` |
| 하위 폴더 지침 | 그 아래 파일을 읽을 때 로드 | 세션을 연 폴더까지만 |
| 경로 조건부 규칙 | `.claude/rules` 의 `paths:` | 없음 |
| `@경로` import | 펼침 | 펼치지 않음 |
| 개인용 덮어쓰기 | `CLAUDE.local.md` | `AGENTS.override.md` |

실무에서 가장 크게 갈리는 줄은 하위 폴더 지침입니다. Claude Code 는 작업 중에 하위 지침을 끌어옵니다. Codex 는 세션을 연 순간에 지침이 정해집니다. 모노레포에서 하위 패키지 규칙을 Codex 에 적용하려면 그 패키지 폴더에서 세션을 여는 편이 확실합니다.

두 도구를 함께 쓴다면 지침 원본은 AGENTS.md 하나로 두는 편이 낫습니다. CLAUDE.md 를 꼭 둬야 하면 그 파일 첫 줄에 `@AGENTS.md` 를 적습니다. Claude Code 는 이 import 를 펼칩니다. Codex 는 CLAUDE.md 를 아예 읽지 않으니 충돌할 일이 없습니다.

## 실사례 — 전역 지침이 통째로 빠져 있었습니다

이 글을 준비하면서 제 맥에서 prompt-input 을 돌렸습니다. 저장소 AGENTS.md 는 들어 있었습니다. 그런데 말투·git 규칙을 담은 전역 지침이 한 줄도 없었습니다. 확인해 보니 `~/.codex/AGENTS.md` 는 0바이트였습니다. 전역 규칙 11KB 는 `~/.codex/instructions.md` 라는 다른 이름의 파일에 들어 있었습니다. 그 파일의 첫 제목을 출력에서 찾으니 0건이었습니다. 공식 문서가 말하는 전역 파일 이름이 아니니 읽히지 않은 것입니다.

증상은 조용했습니다. 오류도 경고도 없었고, Codex 는 저장소 지침만으로 일했습니다. 사람이 "왜 내 규칙을 안 지키지" 하고 느끼기 전까지는 드러나지 않는 종류입니다. 그래서 지침 파일을 옮기거나 이름을 바꾼 날에는 prompt-input 에서 한 줄을 확인합니다.

두 번째 사례는 스캐폴드 저장소입니다. 세 에이전트(Claude Code·Codex·Antigravity)에 같은 지침을 까는 도구입니다. 이 저장소는 지침 원본을 AGENTS.md 하나로 둡니다. Codex 의 32 KiB 한도를 넘지 않게 생성 결과의 크기도 잽니다. 0.166.0 기준 생성된 AGENTS.md 는 7,756 바이트였습니다(2026-11-04). 지침이 길어질수록 다른 하네스보다 Codex 쪽 한도에 먼저 닿기 때문입니다.

## 정리

Codex 에게 규칙을 매번 설명하지 않으려면 git 루트에 AGENTS.md 를 둡니다. Codex 는 루트부터 세션을 연 폴더까지 파일을 이어 붙여 넣습니다. 같은 폴더에 `AGENTS.override.md` 가 있으면 그 파일이 AGENTS.md 를 대신합니다. 신뢰하지 않은 폴더에서도 지침은 읽히지만, 샌드박스 기본값과 `.codex/` 설정은 신뢰에 따라 달라집니다.

다음에 확인할 것은 하나입니다. 지금 쓰는 저장소에서 `codex debug prompt-input` 을 돌려 전역 지침의 한 줄이 실제로 들어가는지 보는 것입니다. 2편에서는 신뢰가 바꾼 첫 줄, 샌드박스와 승인 정책을 다룹니다.
