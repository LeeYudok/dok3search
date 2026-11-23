# agy 실전 가이드 4편 — MCP 와 훅

agy 실전 가이드 4편입니다. 3편까지는 agy 가 읽는 지침·스킬·에이전트를 다뤘습니다. 이번 편은 agy 바깥과 이어지는 두 장치를 다룹니다. 하나는 외부 도구를 붙이는 MCP(Model Context Protocol)입니다. 다른 하나는 도구 호출 앞뒤에서 명령을 돌리는 훅(hook)입니다.

이번 편의 질문은 **agy 에 외부 도구와 강제 장치를 붙일 때 어디를 조심해야 하나**입니다. 답부터 적습니다. MCP 는 `mcp_config.json` 한 파일로 끝나고, 도구를 필요할 때 불러와서 부담이 적습니다. 조심할 쪽은 훅입니다. 출력 형식을 하나 틀리거나 스크립트가 없으면 모든 도구가 막힙니다. 저장소에 든 훅은 비대화형 실행에서 신뢰 확인 없이 돌았습니다.

## MCP 설정은 Gemini CLI 와 다른 파일에 있습니다

제 맥에서 `agy mcp list` 를 치면 서버가 둘 나옵니다.

```text
$ agy mcp list
NAME        TYPE   STATUS   COMMAND/URL
playwright  stdio  enabled  npx -y @playwright/mcp@latest --headless
(리버스 엔지니어링용 stdio 서버 한 줄 생략)
```

이 둘은 `~/.gemini/config/mcp_config.json` 에 적힌 서버와 같습니다. 같은 폴더의 옛 `~/.gemini/settings.json` 에는 Gemini CLI 시절 등록한 서버 4개가 더 있지만 목록에 없습니다. 1편에서 본 것처럼 agy 는 그 파일을 읽지 않습니다.

공식 문서 기준 위치는 세 곳입니다. 전역은 `~/.gemini/config/mcp_config.json`, 워크스페이스는 `.agents/mcp_config.json` 입니다. 플러그인 폴더의 `mcp_config.json` 도 서버를 더합니다. 원격 서버 주소 키는 `serverUrl` 이고 `url` 도 받습니다. Gemini CLI 의 `httpUrl` 과 옛 HTTP+SSE 방식은 지원하지 않습니다.

## 등록은 `agy mcp add` 로 합니다

로컬 `--help` 에는 `agy mcp` 하위 명령 다섯 개가 있습니다.

```bash
agy mcp add fs npx -y @modelcontextprotocol/server-filesystem /work
agy mcp add --env GITHUB_TOKEN=xxx gh -- docker run -i ghcr.io/github/mcp
agy mcp list | enable <이름> | disable <이름> | remove <이름>
```

도움말이 적은 주의점은 셋입니다. 플래그는 서버 이름보다 앞에 둡니다. 이름 뒤에 둔 플래그는 거부됩니다. `http`·`https` 주소는 자동으로 원격 서버로 잡혀 `--type` 이 거의 필요 없습니다. `-` 로 시작하는 인자를 넘길 때는 `--` 를 앞에 둡니다.

재미있는 점은 공식 MCP 문서에 이 명령이 없다는 것입니다. 문서는 대화형 `/mcp` 관리 화면만 설명합니다. 스크립트로 서버를 등록하려면 로컬 `--help` 를 기준으로 삼아야 합니다. 도움말 예시처럼 토큰을 명령줄에 바로 적으면 셸 기록에 남습니다. 환경 변수로 넘기는 편이 낫습니다.

## 도구는 필요할 때 불러옵니다

공식 문서 기준으로 MCP 도구는 기본이 지연 로딩(lazy loading)입니다. 도구 정의를 디스크에 두었다가 필요할 때 읽어 컨텍스트를 아낍니다. 도구별로 `eager` 를 켜면 세션 시작부터 등록합니다. 기본 시한은 도구 호출 180초, 연결 300초입니다.

`eager` 에는 함정이 있었습니다. changelog 1.5.8 을 보면 그 전에는 `eager` 도구에 deny·ask 규칙이 듣지 않았습니다. `mcp(server/tool)` 꼴로 막아도 소용이 없었습니다. 막아 둔 도구가 미리 불러온 도구라는 이유로 그냥 돌았다는 뜻입니다. 1.5.8 보다 낮은 버전을 쓴다면 위험한 MCP 도구에는 `eager` 를 켜지 않습니다.

## 훅 파일은 이름으로 감싼 형식입니다

훅은 `.agents/hooks.json`(워크스페이스)과 `~/.gemini/config/hooks.json`(전역)에 둡니다. 플러그인의 `hooks.json` 도 읽습니다. 공식 문서는 `antigravity-cli/settings.json` 에 적은 훅은 읽지 않는다고 못 박습니다. 이벤트는 여섯 가지입니다.

| 이벤트 | 시점 |
|---|---|
| `SessionStart` | 세션이 시작될 때 한 번 |
| `PreInvocation` / `PostInvocation` | 모델 호출 앞뒤 |
| `PreToolUse` / `PostToolUse` | 도구 실행 앞뒤 |
| `Stop` | 실행이 끝날 때 |

형식은 최상위에 훅 이름을 두고 그 안에 이벤트를 적습니다. 제 스캐폴드 저장소의 `.agents/hooks.json` 을 줄이면 이렇습니다.

```json
{
  "agents-scaffold": {
    "PreInvocation": [
      {"type": "command", "command": "sh .../personal-gate.sh agy prompt", "timeout": 10}
    ],
    "PreToolUse": [
      {"matcher": ".*", "hooks": [
        {"type": "command", "command": "sh .../personal-gate.sh agy pre-tool", "timeout": 10}
      ]}
    ]
  }
}
```

도구 이벤트만 `matcher` 와 `hooks` 배열을 쓰고, 나머지는 핸들러 배열을 바로 씁니다. 이름으로 감싸지 않은 평면 형식은 1.4.2 실측에서 한 번도 실행되지 않았습니다. 형식이 틀려도 화면에는 아무것도 나오지 않았습니다. `--log-file` 로 지정한 로그에만 `command hook must specify 'command'` 가 남았습니다.

## 입력은 camelCase, 판정은 `decision` 입니다

훅은 표준 입력으로 JSON 을 받습니다. 키는 camelCase 입니다. `conversationId`, `workspacePaths`, `toolCall{name, args}` 이 기본입니다. 셸 명령이면 `toolCall.name` 이 `run_command` 이고 명령 문자열은 `args.CommandLine` 에 옵니다. 1.2.16 실측에서 `PostToolUse` 에는 명령 출력이 오지 않았고, 훅의 작업 폴더는 `.agents/` 였습니다. 1.5.4 부터는 `hookEventName`, `cwd`, `toolCall.id` 가 더해졌습니다.

출력은 공식 문서 기준으로 `PreToolUse` 가 `decision` 을 냅니다. 값은 `allow`, `deny`, `ask` 등 여섯 가지입니다. `Stop` 은 `{"decision":"continue","reason":...}` 로 한 턴 더 돌게 합니다. `SessionStart`·`PreInvocation` 은 `injectSteps` 로 메시지를 끼워 넣습니다. 1.5.0 실측에서 `SessionStart` 는 `systemMessage`, `PreInvocation` 은 `ephemeralMessage` 로 넣은 내용이 모델에 전달됐습니다.

## 훅이 실패하면 모든 도구가 막힙니다

2026-11-04 에 1.5.0 으로 훅을 시험하다 두 가지를 겪었습니다.

첫째, `PreToolUse` 훅이 `{}` 를 내면 agy 는 그 도구 호출을 거부했습니다. 메시지는 `tool call denied by pre-tool hook:` 뒤에 빈 사유였습니다. 허용하려면 아무것도 출력하지 않거나 `{"decision":"allow"}` 를 내야 했습니다. 다른 시스템에서 "통과"를 `{}` 로 내던 게이트를 그대로 옮기자 agy 의 도구가 전부 막혔습니다.

둘째, 훅 명령 자체가 실패해도 모든 도구가 막혔습니다. 스크립트 경로가 틀려 종료 코드 127 이 나자 어떤 도구도 실행되지 않았습니다.

해석은 이렇습니다. agy 의 `PreToolUse` 훅은 실패하면 닫히는 쪽(fail-closed)으로 동작합니다. 안전에는 좋지만, 경로 오타 하나가 에이전트를 멈춥니다. 그래서 훅을 커밋하기 전에 견본 JSON 을 표준 입력으로 넣어 출력과 종료 코드를 직접 확인합니다. 다른 이벤트도 같은지는 시험하지 않았습니다.

## 저장소의 훅은 신뢰 확인 없이 돌았습니다

1.2.16 과 1.4.2 를 비대화형으로 실행했습니다. 워크스페이스 `.agents/` 의 에이전트·규칙·`hooks.json` 은 신뢰 확인 없이 로드됐습니다. 훅 네 이벤트도 모두 실행됐습니다. 공식 훅 문서에는 신뢰에 관한 내용이 없습니다. changelog 1.1.1 에는 "폴더를 신뢰한 뒤 워크스페이스 훅이 로드되지 않던 문제"를 고쳤다는 항목이 있습니다. 대화형에는 신뢰 단계가 따로 있는 듯합니다. 대화형에서 신뢰 전에 훅이 도는지는 확인하지 못했습니다.

결론은 단순합니다. 남의 저장소를 클론해 agy 를 돌리기 전에 `.agents/hooks.json` 부터 읽습니다. 그 안의 `command` 는 내 계정 권한으로 실행되는 셸 명령입니다.

## Claude Code 와 다른 점

| 항목 | Claude Code | agy |
|---|---|---|
| MCP 설정 | 프로젝트 `.mcp.json` 등 | `.agents/mcp_config.json`, `~/.gemini/config/mcp_config.json` |
| 훅 위치 | `settings.json` 의 `hooks` | `.agents/hooks.json`, `~/.gemini/config/hooks.json` |
| 셸 도구 이름 | `Bash` | `run_command` |
| 파일 쓰기 도구 | `Write`·`Edit` | `write_to_file` 등 |
| 한 턴 더 돌게 하기 | `Stop` 에서 `block` | `Stop` 에서 `continue` |

같은 훅 스크립트를 두 도구에 쓰려면 입력 키와 도구 이름, 판정 값을 바꿔 주는 층이 필요합니다. 제 스캐폴드는 Claude Code 의 `settings.json` 훅을 원본으로 두고, 어댑터 스크립트가 agy 용 `hooks.json` 을 생성합니다.

## 실사례 — 사용자를 확인하기 전에는 도구를 막는 게이트

1편에서 개인 지침 레이어를 소개했습니다. 그 규칙을 훅으로 강제한 것이 `personal-gate.sh` 입니다. 스크립트 하나를 세 하네스가 함께 부르고, 첫 인자로 하네스 이름(`agy`)을 받습니다. 세션이 시작되면 사용자가 누구인지 확인하라는 지시를 넣습니다. 확인 전에는 질문 도구와 확인 명령 말고 모든 도구 호출을 거부합니다. 확인한 뒤에는 그 사람의 개인 지침 본문을 컨텍스트에 넣습니다.

agy 1.5.0 에서 "notes.txt 를 읽어 줘"를 물었습니다. agy 는 파일을 읽지 않고 먼저 누구인지 물었습니다. 답을 받아 확인 명령을 실행하자 게이트가 열리고 파일을 읽었습니다. 개인 지침도 따랐습니다. 다만 개인 지침은 파일 경로만 알려 줘서는 따르지 않았고, 본문을 컨텍스트에 직접 넣어야 했습니다. 위의 `{}` 거부 문제도 이 게이트를 옮기다가 찾았습니다.

측정할 때 주의할 점도 하나 있었습니다. 제 맥에는 작업 상태를 터미널 도구에 알리는 전역 훅(`~/.gemini/config/hooks.json`)이 있습니다. 전역 훅과 저장소 훅이 함께 돌면 어느 쪽 효과인지 섞입니다. 훅을 잴 때는 전역 훅이 있는지부터 봅니다.

## 정리

4편의 답은 이렇습니다. MCP 는 `mcp_config.json` 과 `agy mcp` 명령으로 간단히 붙습니다. 훅은 형식 오류는 조용히 무시되고, 출력 오류와 실행 실패는 모든 도구를 막습니다. 저장소 훅은 신뢰 확인 없이 돌았으니 남의 저장소에서는 먼저 읽습니다.

다음에 확인할 것은 내 `hooks.json` 의 `PreToolUse` 출력입니다. 통과일 때 `{}` 를 내고 있지는 않은지 보십시오. 마지막 5편에서는 agy 를 스크립트와 다른 에이전트에서 부르는 법을 다룹니다.
