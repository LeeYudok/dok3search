# agy 실전 가이드 3편 — 스킬·서브에이전트·플러그인

agy 실전 가이드 3편입니다. 1편은 지침 파일, 2편은 실행 모드와 컨텍스트를 다뤘습니다. 이번 편은 상황별 절차를 담는 스킬(skill)과 일을 나눠 맡는 서브에이전트를 다룹니다. 이 둘을 묶어 나르는 플러그인도 함께 봅니다.

이번 편의 질문은 **Claude Code 용 스킬과 서브에이전트를 agy 에서 그대로 쓸 수 있는가**입니다. 답은 반반입니다. 스킬은 거의 그대로 됩니다. `SKILL.md` 형식이 같고 심링크도 따라 읽습니다. 다만 놓는 경로가 다르고, 24KB 를 넘는 본문은 잘립니다. 서브에이전트는 머리말을 agy 형식으로 바꾸지 않으면 조용히 빠집니다.

## 스킬 경로는 문서가 실측을 따라왔습니다

2026-10-05 에 본 공식 문서는 CLI 전역 스킬 경로를 `~/.gemini/antigravity-cli/skills/` 로 적었습니다. 그런데 당시 agy 1.2.16 은 `~/.gemini/config/skills/` 를 읽었습니다. 스킬 하나를 여러 경로에 걸고 agy 에게 어디서 읽었는지 물었더니 `config/skills` 쪽을 댔습니다. 이것은 모델의 자기 보고라 파일 시스템 확인은 아니었습니다.

2026-11-18 에 다시 본 공식 문서는 `~/.gemini/config/skills/` 를 App·CLI·IDE 공통 전역 경로로 적습니다. CLI 는 `~/.gemini/antigravity-cli/skills/` 와 `~/.gemini/skills/` 도 읽는다고 덧붙였습니다. 결과적으로 실측이 맞았고, 문서가 뒤따라 고쳐졌습니다.

이 이야기가 실무에서 중요한 이유는 외부 설치 도구 때문입니다. 오픈소스 스킬 설치기 `npx skills` 1.7.6 을 2026-11-05 에 확인했습니다. 이 도구는 Antigravity 용 스킬을 `~/.gemini/antigravity/skills`(IDE)와 `antigravity-cli/skills`(CLI)에 깔았습니다. `~/.gemini/antigravity/skills` 는 agy CLI 가 찾지 않는 경로입니다. 남이 만든 설치기를 쓰면 깔린 위치부터 확인합니다.

워크스페이스 스킬은 `.agents/skills/<이름>/` 입니다. 옛 `.agent/skills/` 도 읽습니다. Claude Code 의 `.claude/skills/` 는 읽지 않습니다. 같은 폴더에 두 경로의 시험용 스킬을 두고 물었을 때, 1.2.16 은 `.agents/skills` 쪽만 목록에 올렸습니다.

## 원본 하나에 심링크 셋을 겁니다

저는 스킬 원본을 한 저장소에 두고, 설치 스크립트가 세 곳에 심링크를 겁니다.

| 에이전트 | 전역 스킬 경로 |
|---|---|
| Claude Code | `~/.claude/skills/` |
| Codex | `~/.agents/skills/` |
| agy | `~/.gemini/config/skills/` |

2026-11-18 기준 `~/.gemini/config/skills/` 에는 39개 항목이 있고 그중 32개가 심링크입니다. agy 와 Claude Code 는 심링크된 `SKILL.md` 도 읽습니다. Codex 는 달랐습니다. codex-cli 0.169.0 은 `SKILL.md` 파일 자체가 심링크면 목록에서 뺐습니다(2026-11-16). 그래서 Codex 쪽만 원본 폴더를 직접 가리키게 걸었습니다.

심링크에는 지울 때 함정이 있습니다. 링크 안에서 파일을 지우면 원본이 지워집니다. 링크만 없애려면 `unlink <링크>` 를 씁니다.

## 스킬은 슬래시 명령이 됩니다

공식 문서 기준으로 agy 는 스킬을 점진 공개(progressive disclosure) 방식으로 씁니다. 매 턴 스킬 이름과 `description` 만 목록으로 보고, 관련이 있어 보이면 `SKILL.md` 전문을 읽습니다. 머리말에서 `description` 은 필수이고 `name` 은 생략하면 폴더 이름을 씁니다.

CLI 에서는 스킬마다 슬래시 명령이 자동으로 생깁니다. `deploy-staging` 스킬이 있으면 `/deploy-staging` 을 칠 수 있습니다. 세션 도중에 스킬을 더했으면 `/skills reload` 로 다시 읽힙니다.

## 24KB 를 넘는 스킬은 잘립니다

공식 스킬 문서에는 크기 한도가 없습니다. 하지만 changelog 1.4.1 에 따르면 `agy plugin validate` 는 본문이 24KB 한도를 넘는 스킬을 경고합니다. 2026-11-04 에 1.5.0 으로 검사했습니다. 26.9KB·48KB 짜리 스킬 두 개에 "넘는 부분은 잘린다"는 경고가 나왔습니다.

2026-11-18 에 제 전역 스킬 39개의 `SKILL.md` 를 다시 쟀습니다. 24,000바이트를 넘는 파일이 5개였습니다. 가장 큰 것은 45,671바이트였습니다. 다섯 가운데 넷은 본문이 한국어이고, 하나는 영어입니다.

한국어 스킬은 이 한도에 더 빨리 닿습니다. UTF-8 에서 한글 한 글자는 3바이트입니다. 영어 스킬은 43,600자가 43,608바이트로 글자 수와 바이트 수가 거의 같았습니다. 한국어 스킬 넷은 코드와 영문이 섞여 있는데도 바이트 수가 글자 수의 1.7~1.9배였습니다. 그래서 한국어 스킬은 약 1만 2천~1만 4천 자에서 24KB 에 닿습니다. 긴 스킬은 본문에 절차의 뼈대만 두고 세부는 `references/` 아래 파일로 뺍니다. 에이전트가 필요할 때 그 파일을 따로 읽게 합니다.

## 머리말은 엄격하게 씁니다

스킬과 에이전트의 머리말은 YAML 입니다. 1.3.5 이하에서는 따옴표 없는 값 안에 `: ` 가 있으면 파싱에 실패했습니다. 그 스킬은 조용히 빠졌고, 이유는 `--log-file` 로 지정한 로그에만 `mapping values are not allowed` 로 남았습니다. 제 스킬 하나가 이렇게 한동안 빠져 있었습니다. 1.3.6 에서 `description` 의 콜론 문제는 고쳐졌습니다. 닫히지 않은 `[` 는 1.5.0 에서도 그 스킬을 건너뛰게 했습니다.

1.5.10 에서도 비슷한 문제가 하나 고쳐졌습니다. 머리말에 `name` 이 없는 스킬은 모두 이름이 `SKILL` 이 되어 첫 번째 것만 올라갔습니다. 지금은 폴더 이름을 씁니다. 머리말 값은 따옴표로 감싸고, `name` 을 적어 두는 편이 안전합니다.

## 서브에이전트는 머리말을 agy 형식으로 바꿔야 합니다

사용자 정의 서브에이전트는 `.agents/agents/<이름>.md` 에 둡니다. 전역은 `~/.gemini/config/agents/`, 플러그인은 그 안의 `agents/` 입니다. 공식 문서는 기본 제공 서브에이전트로 `research`, `browser`, `self` 를 꼽습니다. 머리말의 `model` 값은 `inherit`, `flash`, `pro` 가운데 하나입니다.

Claude Code 의 에이전트 파일을 그대로 옮기면 문제가 생깁니다. 머리말에 Claude 식 `tools: Read, Grep`, `model: sonnet`, `memory` 가 있는 에이전트는 목록에 오르지 않았습니다. 1.2.16(2026-10-04)과 1.4.2(2026-10-19) 모두 같았습니다. 로그에도 이유가 없었습니다. 그 이름으로 `agy --agent <이름>` 을 부르면 `error: unknown agent` 와 함께 종료 코드 1 이 났습니다. 1.3.4 전에는 기본 에이전트로 조용히 실행됐다고 changelog 에 적혀 있습니다.

그래서 제 생성기는 agy 용 에이전트 파일에 `name` 과 `description` 만 남깁니다. 도구와 모델은 agy 기본값을 물려받습니다. 공식 문서는 `tools`·`model` 필드를 지원한다고 적습니다. 그러니 문제는 필드가 아니라 Claude 식 값일 가능성이 큽니다. agy 식 값(`model: flash`)을 넣은 경우는 아직 시험하지 않았습니다.

## 플러그인으로 묶어서 나릅니다

스킬·에이전트·규칙·훅·MCP 설정을 한 폴더로 묶은 것이 플러그인입니다. 제 맥의 상태는 다음과 같습니다.

```text
$ agy plugin list
Installed plugins:
- agents-scaffold-skills-agents (enabled, 1.0.0)
    Source:     antigravity
    Components: 16 agents
```

이 플러그인의 `plugin.json` 은 `name`·`description`·`version` 세 키뿐입니다. 폴더 안 `agents/` 에 에이전트 파일 16개가 있고, `agy agents` 를 치면 그 16개가 나옵니다. 설치 스크립트가 Claude Code 용 에이전트 정의에서 이 플러그인을 만듭니다.

공식 문서 기준 플러그인 명령은 `agy plugin install <이름>@antigravity-plugins-official`, `agy plugin import claude|gemini`, `agy plugin validate` 입니다. changelog 1.5.5 를 보면 Claude Code 플러그인을 가져올 때 형식을 바꿉니다. 명령은 스킬로, 훅은 agy 형식으로 옮깁니다. 이때 matcher 의 `Bash` 는 `run_command` 로 옮겨집니다. 1.5.0 의 `validate` 는 `version` 이 없으면 마켓플레이스에 노출되지 않는다고 경고했습니다.

## Claude Code 와 다른 점

| 항목 | Claude Code | agy |
|---|---|---|
| 프로젝트 스킬 | `.claude/skills/` | `.agents/skills/` |
| 전역 스킬 | `~/.claude/skills/` | `~/.gemini/config/skills/` |
| 프로젝트 에이전트 | `.claude/agents/*.md` | `.agents/agents/*.md` |
| 에이전트 머리말 | `tools`·`model`·`memory` 등 | `name`·`description` 필수, Claude 식 값이면 조용히 빠짐 |
| 스킬 본문 한도 | 확인하지 못함 | 약 24KB, 넘으면 잘림 |

같은 `SKILL.md` 를 쓰지만 폴더 이름이 `.claude` 와 `.agents` 로 갈립니다. 그래서 원본을 한 곳에 두고 링크나 생성물로 나눠 주는 구조가 필요합니다.

## 실사례 — 한 저장소에서 세 하네스용 파일을 만듭니다

제 에이전트 지침 스캐폴드 저장소는 `.claude/` 를 원본으로 둡니다. agy 용 파일은 설치할 때 생성합니다. 경로별 규칙은 `.agents/rules/` 로, 에이전트는 `.agents/agents/` 로, 훅은 `.agents/hooks.json` 으로 만듭니다. 스킬만은 원본이 `.agents/skills/` 에 있고, Claude Code 쪽에 링크를 겁니다.

생성물이 실제로 읽히는지는 측정 스크립트로 확인합니다. 임시 폴더에 스캐폴드를 깔고 `agy -p` 로 질문한 뒤 답을 판정합니다. 답하기 전에 도구를 한 번이라도 썼으면 판정 불가로 처리합니다. 파일을 직접 열어 보고 답했을 수 있기 때문입니다.

이 스크립트가 1.4.0 에서 한 번 무너졌습니다. 처음에는 `user_input`·`agent_response` 가 아닌 단계를 모두 도구 호출로 셌습니다. 그런데 1.4.0 에 도구가 아닌 단계(`error_message` 같은 것)가 새로 생겼습니다. 그러자 모든 질문이 판정 불가가 됐습니다. 도구 이름이 붙은 단계만 세도록 고친 뒤에야 1.4.2 결과가 1.2.16 과 같게 나왔습니다. 자동 업데이트되는 도구를 잴 때는 "아닌 것"을 빼는 대신 "맞는 것"을 세야 했습니다.

## 정리

3편의 답은 이렇습니다. 스킬은 경로만 맞추면 Claude Code 용을 거의 그대로 씁니다. 단 한국어 스킬은 24KB 한도에 일찍 닿습니다. 서브에이전트는 머리말을 `name`·`description` 위주로 바꿔야 목록에 오릅니다.

다음에 확인할 것은 내 스킬의 크기입니다. `SKILL.md` 가 24,000바이트를 넘는지 재 보십시오. 4편에서는 agy 에 외부 도구를 붙이는 MCP 와, 도구 호출 앞뒤에 끼어드는 훅을 다룹니다.
