# Codex 실전 가이드 4편 — 스킬·서브에이전트·훅

Codex 실전 가이드 4편입니다. 1편은 AGENTS.md, 2편은 샌드박스와 승인 정책, 3편은 설정과 컨텍스트를 다뤘습니다. 이번 편은 Codex 에 반복 절차와 역할을 붙이는 세 장치, 스킬·서브에이전트·훅입니다. Claude Code 가이드 3편의 서브에이전트·병렬 이야기와 짝을 이룹니다.

이번 편이 답하는 질문은 이것입니다. **스킬·서브에이전트·훅을 만들었는데 실제로 켜졌는지 어떻게 아나.** 답은 세 장치 모두 "파일을 만들었다"와 "켜졌다"가 다르다는 것입니다. 스킬은 심링크한 `SKILL.md` 를 조용히 건너뜁니다. 저장소의 서브에이전트와 훅은 신뢰한 프로젝트에서만 읽고, 훅은 정의 자체를 한 번 더 신뢰받아야 돕니다. 확인 수단은 이번에도 `codex debug prompt-input` 과 `/hooks` 입니다.

## 스킬은 목록만 먼저 들어갑니다

공식 문서 기준으로 스킬 위치는 세 군데입니다. 사용자 스킬은 `$HOME/.agents/skills`, 저장소 스킬은 작업 폴더에서 저장소 루트까지 각 폴더의 `.agents/skills` 입니다. 관리자용으로 `/etc/codex/skills` 가 있습니다. `SKILL.md` 앞머리에는 `name` 과 `description` 이 꼭 있어야 합니다.

prompt-input 에서 본 실제 탐색 경로는 더 많았습니다. `~/.codex/skills`, `~/.agents/skills`, Codex 내장 스킬 폴더, 플러그인 캐시 폴더 셋, 그리고 저장소의 `.agents/skills` 입니다. 각 경로는 `r0`~`r6` 같은 짧은 별칭으로 바뀌어 목록에 들어갑니다.

목록 한 줄에는 이름, 설명, 파일 위치만 있습니다. 본문은 들어가지 않습니다. 기본 지시문은 사용자가 `$스킬이름` 이나 이름으로 스킬을 부르면 그 스킬을 작업 계획에 넣으라고 적습니다. 부르지 않아도 작업이 설명과 맞으면 모델이 판단해 씁니다. 이때 모델은 목록의 위치를 보고 `SKILL.md` 를 파일로 직접 엽니다. Claude Code 처럼 스킬 전용 도구를 거치지 않습니다. CLI 에서 목록을 보는 명령은 `/skills` 입니다(공식 문서 기준).

그래서 설명문이 곧 호출 조건입니다. 모델이 처음 보는 것은 설명 한 줄뿐이기 때문입니다. 3편에서 잰 대로 제 맥의 스킬 목록은 65개, 22,073자였습니다. 설명이 길수록 매 세션 비용이 커지고, 짧을수록 엉뚱한 때 불리거나 안 불립니다.

## 심링크한 SKILL.md 는 조용히 빠집니다

스킬을 여러 에이전트에 나눠 쓰려면 원본 한 곳에 두고 심링크를 거는 방법이 편합니다. Codex 에서는 링크를 거는 위치가 중요합니다. 빈 저장소에 스킬 두 개를 두고 재 보았습니다(0.169.0, 2026-11-18).

| 스킬 | 구조 | prompt-input 목록 |
|---|---|---|
| `probe-real` | 폴더와 `SKILL.md` 모두 실제 파일 | 들어감 |
| `probe-link` | 폴더는 실제, `SKILL.md` 만 다른 파일로 가는 심링크 | 빠짐 |

반대로 스킬 폴더 자체가 심링크인 경우는 문제가 없었습니다. 제 `~/.agents/skills` 에는 폴더 링크가 28개 있고, 28개 모두 목록에 들어갔습니다. 빠질 때 오류도 경고도 없다는 점이 가장 곤란합니다. 스킬을 설치했으면 `codex debug prompt-input | grep <스킬이름>` 으로 한 번 확인합니다.

## 서브에이전트는 TOML 한 장이고 같은 폴더를 씁니다

공식 문서 기준으로 서브에이전트 정의는 `~/.codex/agents/`(개인용)나 `.codex/agents/`(저장소용)에 두는 TOML 파일입니다. 필수 키는 `name`·`description`·`developer_instructions` 셋입니다. `model`·`model_reasoning_effort` 등은 고를 수 있고, `sandbox_mode`·`mcp_servers` 는 부모 설정을 덮지 못합니다.

```toml
# ~/.codex/agents/go_expert.toml
name = "go_expert"
description = "Go 코드를 작성·리뷰하거나 동시성·테스트를 다룰 때 사용한다."
developer_instructions = '''
`go-expert` 스킬을 작업 계약의 원본으로 쓴다. 절차는 이 파일에 복제하지 않는다.
'''
```

이렇게 둔 파일은 prompt-input 에서 `spawn_agent` 도구의 `agent_type` 후보로 나타납니다. 저장소의 `.codex/agents/` 는 신뢰한 프로젝트에서만 읽습니다(0.160.0 실측).

prompt-input 의 서브에이전트 안내문에는 운영 조건이 그대로 적혀 있습니다. 0.169.0 에서 확인한 내용입니다.

- 도구는 `spawn_agent`·`followup_task`·`send_message`·`wait_agent`·`interrupt_agent`·`list_agents` 입니다. 하위 에이전트도 자기 하위 에이전트를 띄울 수 있습니다.
- 동시 실행 칸은 4개이고, 메인 에이전트도 한 칸을 씁니다.
- 모든 에이전트가 같은 폴더와 같은 작업 디렉터리를 씁니다. 한 에이전트의 수정이 다른 에이전트에게 바로 보입니다.
- 사용자나 AGENTS.md·스킬이 명시하지 않으면 서브에이전트를 띄우지 않습니다.

마지막 두 줄이 실무에서 중요합니다. 병렬로 맡기고 싶으면 프롬프트나 AGENTS.md 에 그렇게 적어야 합니다. 그리고 여러 에이전트가 같은 파일을 고치면 서로의 변경이 섞입니다. Claude Code 가이드 3편의 "1세션 = 1worktree" 규칙은 Codex 에서 세션 단위로 지킵니다. 서브에이전트로는 폴더를 나눌 수 없기 때문입니다. 0.169.0 에는 세션을 새 git worktree 에서 여는 `--worktree` 옵션이 있습니다.

## 훅은 두 번 신뢰받아야 돕니다

공식 문서 기준으로 훅 설정은 `~/.codex/hooks.json`, 신뢰한 프로젝트의 `.codex/hooks.json`, `/etc/codex/hooks.json` 에 둡니다. `config.toml` 안에 `[hooks]` 로 적어도 됩니다. 형식은 Claude Code 의 settings.json 훅과 거의 같습니다.

```json
{"hooks": {"Stop": [{"hooks": [{"type": "command", "command": "python3 ~/.codex/my-hook.py", "timeout": 10}]}]}}
```

공식 문서가 나열한 이벤트는 14개입니다. SessionStart, SessionEnd, UserPromptSubmit, PreToolUse, PermissionRequest, PostToolUse, PreCompact, PostCompact, Stop, SubagentStart, SubagentStop, PreModelRequest, PostModelRequest, Interrupt 입니다.

훅이 실제로 돌려면 신뢰가 두 겹 필요합니다. 첫째, 저장소 훅이면 그 프로젝트를 신뢰해야 합니다. 둘째, 훅 정의 자체를 `/hooks` 화면에서 검토하고 신뢰해야 합니다. 신뢰는 `config.toml` 의 `[hooks.state."<파일>:<이벤트>:..."]` 에 `trusted_hash` 로 남습니다. 정의를 한 글자라도 바꾸면 해시가 달라져 다시 신뢰받을 때까지 실행되지 않습니다. 0.160.0 에서 잰 결과는 이렇습니다.

| 조건 | SessionStart·UserPromptSubmit·Stop 실행 |
|---|---|
| 신뢰한 프로젝트 + `--dangerously-bypass-hook-trust` | 3/3 |
| 신뢰한 프로젝트, 훅 정의 신뢰 없음 | 0/3 |
| 신뢰하지 않은 프로젝트 + 우회 옵션 | 0/3 |

훅 입력은 Claude Code 와 비슷합니다. 셸 명령은 `Bash`, 파일 편집은 `apply_patch` 라는 도구 이름으로 들어옵니다. Stop 훅이 `{"decision":"block","reason":...}` 를 돌려주면 Codex 는 한 턴 더 일합니다. 0.166.0 부터는 도구 관련 훅 입력의 `cwd` 가 바뀌었습니다. PreToolUse·PermissionRequest·PostToolUse 에서 이 값은 이제 그 명령의 작업 폴더입니다. 세션 루트를 `cwd` 로 계산하던 훅은 이때 경로가 어긋났습니다.

## Claude Code 와 다른 점

| 항목 | Claude Code | Codex 0.169.0 |
|---|---|---|
| 스킬 위치 | `~/.claude/skills`, `.claude/skills` | `~/.agents/skills`, `.agents/skills` 등 |
| 스킬 호출 | 스킬 도구 | `$이름` 또는 모델 판단, `SKILL.md` 를 파일로 읽음 |
| 심링크한 `SKILL.md` | 읽음 | 건너뜀 |
| 서브에이전트 정의 | `.claude/agents/*.md` | `.codex/agents/*.toml` |
| 서브에이전트 격리 | `isolation: worktree` 지정 가능 | 같은 폴더 공유, 세션 단위 `--worktree` |
| 훅 실행 조건 | 설정 파일에 적으면 실행 | 프로젝트 신뢰 + 훅 정의 해시 신뢰 |

두 도구 모두 훅은 강제선이 아닙니다. 훅은 그 세션이 그 도구로 그 일을 할 때만 발동합니다. 사람이 터미널에서 직접 커밋하면 지나갑니다. 결정적인 강제는 `.git/hooks` 와 CI 에 둡니다.

## 실사례 — 원본 하나를 세 하네스에 까는 저장소

제가 관리하는 스캐폴드 저장소는 스킬·서브에이전트·훅을 원본 한 벌로 둡니다. 설치할 때 Claude Code·Codex·Antigravity 용으로 나눠 깝니다.

스킬은 원본 폴더에서 세 에이전트의 스킬 경로로 심링크합니다. 처음에는 설치 스크립트가 만든 폴더 안의 `SKILL.md` 가 심링크였습니다. Claude Code 와 Antigravity 는 이 구조를 읽었고 Codex 만 목록에서 뺐습니다. 2026-11-16 에 Codex 쪽만 원본 폴더를 직접 가리키게 바꿨습니다.

서브에이전트는 `.claude/agents/*.md` 를 원본으로 두고 Codex 용 TOML 을 생성합니다. `developer_instructions` 는 TOML 리터럴 여러 줄 문자열(`'''`)로 넣습니다. 그러면 본문의 백슬래시와 따옴표가 그대로 전달됩니다. 이름에 하이픈이 들어가도 등록됐습니다.

훅은 Claude Code 의 settings.json 에서 Codex 용 `hooks.json` 을 생성합니다. 여기서 앞 절의 `cwd` 변화를 겪었습니다. 지금은 훅 스크립트가 입력의 `cwd` 대신 프로세스 작업 폴더의 git 루트를 먼저 씁니다. 이 셋을 실제 모델 호출로 재는 검사 스크립트를 두었습니다. 0.166.0 기준 7개 항목 중 5개가 통과였습니다. 부분 통과 2개는 경로 조건부 지침과 훅입니다(2026-11-04). 그래서 이 저장소는 Codex 를 "기본 지원" 등급으로 분류합니다.

## 정리

스킬·서브에이전트·훅은 모두 파일을 만드는 것으로 끝나지 않습니다. 스킬은 심링크한 `SKILL.md` 를 건너뜁니다. 서브에이전트는 같은 폴더를 공유합니다. 훅은 프로젝트와 정의를 둘 다 신뢰해야 돕니다.

다음에 확인할 것은 하나입니다. 스킬을 깔거나 훅을 고친 날, prompt-input 과 `/hooks` 에서 실제로 켜졌는지 보는 것입니다. 마지막 5편에서는 MCP·플러그인·`codex exec` 로 Codex 를 바깥 도구와 다른 에이전트에 이어 붙이는 이야기를 합니다.
