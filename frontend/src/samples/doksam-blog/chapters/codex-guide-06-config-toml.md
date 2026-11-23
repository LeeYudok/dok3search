# Codex 실전 가이드 6편 — config.toml 설정 매뉴얼 (0.170.0 기준)

Codex 실전 가이드 6편입니다. 3편에서 설정이 겹치는 순서와 프로필을 다뤘습니다. 이번 편은 `~/.codex/config.toml` 한 파일을 매뉴얼로 정리합니다. 무엇을 적을 수 있고, 적은 값이 실제로 먹히는지를 확인했습니다.

이번 편이 답하는 질문은 이것입니다. **config.toml 에 무엇을 어떻게 적어야 0.170.0 이 그대로 받아들이나.** 0.170.0 의 공식 설정 스키마에는 최상위 키가 103개 있습니다. 자주 쓰는 것은 그중 스무 개 남짓입니다. 적은 설정은 `codex --strict-config` 로 검사합니다. 이 글의 예시는 모두 0.170.0 에서 이 검사를 통과했습니다. 예전에 통하던 설정 가운데 셋은 0.170.0 에서 오류로 멈춥니다. `wire_api = "chat"`, `approval_policy = "untrusted"`, 그리고 `-p` 와 함께 쓴 `[profiles]` 표입니다.

> [그림] 로봇 옆에 config.toml 제어판이 있다. 제어판에는 모델 다이얼, 샌드박스 세 칸 슬라이더, 꺼진 web_search 스위치, MCP 와 훅 플러그 자리가 있다. 로봇은 사내 추론 서버의 /v1/responses 에 연결되고 인터넷으로 가는 선은 끊겨 있다. 돋보기가 철자가 틀린 키 modle_reasoning_effort 를 찾아 --strict-config 가 잡는다고 표시한다

## 어떻게 확인했나

2026-11-18 에 맥의 codex-cli 0.170.0 으로 확인했습니다. 키 목록과 허용값은 GitHub 태그 `rust-v0.170.0` 의 `codex-rs/core/config.schema.json` 에서 읽었습니다.

동작은 직접 돌려서 봤습니다. 빈 `CODEX_HOME` 에 설정 파일을 하나씩 쓰고, 로컬에 띄운 가짜 Responses 서버를 공급자로 붙였습니다. 그리고 `codex exec` 를 돌려 두 가지를 봤습니다. 하나는 실행할 때 찍히는 시작 배너입니다. 배너에는 실제로 적용된 모델·승인 정책·샌드박스가 나옵니다. 다른 하나는 가짜 서버가 받은 요청 본문입니다. `model`, `reasoning`, `tools` 에 무엇이 실렸는지 확인했습니다. 모델 응답의 품질은 이 글의 범위가 아닙니다.

## 1. 파일 위치와 검사

설정 파일은 `$CODEX_HOME/config.toml` 입니다. `CODEX_HOME` 을 따로 정하지 않으면 `~/.codex` 입니다. 이 파일 위에 프로필 파일, 신뢰한 프로젝트의 `.codex/config.toml`, 명령줄 `-c` 가 차례로 덮습니다. 순서는 3편에 정리했습니다.

철자가 틀린 키는 기본 동작에서 경고만 내고 무시됩니다. `model_reasoning_effort` 를 `modle_reasoning_effort` 로 잘못 쓰면 이렇게 나옵니다.

```text
warning: Codex is ignoring 1 unrecognized configuration setting. Check for typos or deprecated settings.
  user (~/.codex/config.toml): `modle_reasoning_effort` is ignored.
```

경고는 실행 기록 사이에 묻히기 쉽습니다. `--strict-config` 를 주면 같은 실수가 줄 번호와 함께 오류로 멈춥니다.

```text
$ codex --strict-config
Error loading config.toml:
~/.codex/config.toml:4:1: unknown configuration field `modle_reasoning_effort`
  |
4 | modle_reasoning_effort = "high"
  | ^^^^^^^^^^^^^^^^^^^^^^
```

값이 틀린 경우는 `--strict-config` 없이도 멈춥니다. 이 오류 메시지가 그대로 허용값 목록 노릇을 합니다.

```text
unknown variant `sandbox-bogus`, expected one of `read-only`, `workspace-write`, `danger-full-access`
```

설정 파일을 고친 날에는 `codex --strict-config` 로 한 번 열어 봅니다. 한 번만 바꿔 보려면 파일을 고치지 않고 `-c 키=값` 을 씁니다.

## 2. 모델과 공급자

최상위에 두는 모델 관련 키입니다.

| 키 | 값 | 설명 |
|---|---|---|
| `model` | 문자열 | 쓸 모델 이름 |
| `model_provider` | 문자열 | `[model_providers.<id>]` 의 id |
| `model_reasoning_effort` | 모델이 알리는 값(예: `low`·`medium`·`high`) | 추론 강도 |
| `model_reasoning_summary` | `auto`·`concise`·`detailed`·`none` | 추론 요약 |
| `model_verbosity` | `low`·`medium`·`high` | 답 길이(GPT-5 계열) |
| `model_context_window` | 정수 | 컨텍스트 창 크기(토큰) |
| `model_auto_compact_token_limit` | 정수 | 자동 압축을 시작하는 토큰 수 |
| `openai_base_url` | URL | 내장 `openai` 공급자의 주소만 바꿀 때 |

요청에 실제로 실린 값은 이랬습니다.

- `model_reasoning_effort = "high"` 를 주면 요청에 `"reasoning": {"effort": "high"}` 가 실렸습니다.
- 아무것도 주지 않아도 요청에는 `"reasoning": {"summary": "auto"}` 가 실렸습니다. `model_reasoning_summary = "none"` 을 주면 이 항목이 빠졌습니다.
- `model_verbosity = "low"` 는 Codex 모델 목록에 없는 모델 이름에서는 요청에 실리지 않았습니다.

### 공급자 표

사내 서버나 다른 회사 API 는 `[model_providers.<id>]` 표로 붙입니다. 스키마 설명대로 `openai` 같은 내장 id 는 덮어쓸 수 없으니 새 id 를 씁니다.

```toml
model_provider = "inhouse"
model = "서버가-서빙하는-모델-이름"

[model_providers.inhouse]
name = "사내 추론 서버"
base_url = "https://<추론 서버 주소>/v1"
env_key = "INHOUSE_API_KEY"        # 키 값이 아니라 키를 담은 환경변수 이름
wire_api = "responses"
http_headers = { "X-Team" = "platform" }
request_max_retries = 4
stream_max_retries = 5
stream_idle_timeout_ms = 300000
```

| 키 | 설명 |
|---|---|
| `name` | 화면에 보일 이름 |
| `base_url` | OpenAI 호환 API 주소. `/v1` 까지 적는다 |
| `env_key` | API 키를 담은 환경변수 이름. `Authorization: Bearer` 로 보낸다 |
| `wire_api` | 0.170.0 에서는 `"responses"` 만 받는다 |
| `http_headers` | 고정 헤더. 실측에서 요청에 그대로 실렸다 |
| `env_http_headers` | 헤더 값을 환경변수에서 읽는다 |
| `query_params` | 주소 뒤에 붙일 쿼리 |
| `request_max_retries`·`stream_max_retries` | 요청 재시도, 끊긴 스트림 재연결 횟수 |
| `stream_idle_timeout_ms` | 스트림이 조용할 때 끊긴 것으로 볼 시간 |
| `requires_openai_auth` | `true` 면 OpenAI 로그인 화면을 띄운다 |

`wire_api` 는 가장 먼저 확인할 키입니다. 0.170.0 에서 `"chat"` 을 주면 설정을 읽는 단계에서 멈춥니다.

```text
Error loading config.toml: `wire_api = "chat"` is no longer supported.
How to fix: set `wire_api = "responses"` in your provider config.
```

그래서 사내 추론 서버는 `/v1/responses` 를 제공해야 합니다. `/v1/chat/completions` 만 되는 서버에는 0.170.0 을 붙일 수 없습니다. 엔진이 두 경로를 다 연다면, 다른 클라이언트가 잘 붙는다는 사실만으로는 부족합니다. Codex 가 쓰는 Responses 경로를 따로 확인해야 합니다. 3편에서 본 별칭 모델 이름 404 도 이 경로에서만 일어났습니다.

사내 모델 이름은 Codex 의 모델 목록에 없습니다. 그래서 실행할 때마다 경고가 하나 나옵니다.

```text
warning: Model metadata for `서버가-서빙하는-모델-이름` not found. Defaulting to fallback metadata; this can degrade performance and cause issues.
```

`model_context_window` 를 적어도 이 경고는 남았습니다. 컨텍스트 창 크기와 자동 압축 시점은 직접 적어 두는 편이 안전합니다. 기본값을 추정에 맡기지 않기 위해서입니다. 모델 목록 자체를 JSON 파일로 넘기는 `model_catalog_json` 키도 있습니다. 이 키는 이번에 확인하지 못했습니다.

## 3. 권한 — 승인 정책과 샌드박스

| 키 | 0.170.0 이 받는 값 |
|---|---|
| `approval_policy` | `on-request`·`never`·`{ granular = { ... } }` |
| `sandbox_mode` | `read-only`·`workspace-write`·`danger-full-access` |

`approval_policy = "untrusted"` 는 0.170.0 에서 오류입니다.

```text
Error: approval_policy = "untrusted" is no longer supported; remove this setting
```

`on-failure` 는 스키마 목록에는 없지만 `--strict-config` 를 통과했습니다. 2편에서 본 대로 공식 문서는 이 값을 폐기 예정으로 적습니다. 새로 쓸 이유는 없습니다.

`codex exec` 의 시작 배너는 `approval_policy` 를 `on-request` 로 두어도 `approval: never` 로 찍혔습니다. 무인 실행에는 승인을 물어볼 사람이 없으니 이렇게 정한 것으로 보입니다(제 해석입니다). 같은 설정 파일이라도 대화형 화면과 무인 실행의 승인 동작이 다르다는 점을 기억해 둡니다.

`workspace-write` 를 고르면 배너에 쓸 수 있는 폴더가 나옵니다. 실측에서는 `[workdir, /tmp, $TMPDIR]` 였습니다. 범위는 `[sandbox_workspace_write]` 표로 조정합니다.

```toml
sandbox_mode = "workspace-write"

[sandbox_workspace_write]
writable_roots = ["/data/build"]   # 작업 폴더 밖에 더 열 폴더
network_access = false             # 샌드박스 안 명령의 네트워크
exclude_tmpdir_env_var = false
exclude_slash_tmp = false
```

이보다 세밀한 권한 묶음은 `default_permissions` 와 `[permissions.<이름>]` 으로 따로 정의할 수 있습니다. 이번 편에서는 다루지 않습니다.

## 4. 도구 — 웹 검색, MCP, 훅

### 웹 검색

`web_search` 는 값이 네 개입니다. 각 값을 주고 요청에 실린 도구 정의를 그대로 옮깁니다.

| 값 | 요청의 `tools` 에 실린 것 |
|---|---|
| (안 적음) | `{"type": "web_search", "external_web_access": false}` |
| `cached` | `{"type": "web_search", "external_web_access": false}` |
| `indexed` | `{"type": "web_search", "external_web_access": true, "indexed_web_access": true}` |
| `live` | `{"type": "web_search", "external_web_access": true}` |
| `disabled` | 실리지 않음 |

이 표에서 볼 점은 두 가지입니다. 첫째, 적지 않으면 `cached` 와 같습니다. 웹 검색은 기본으로 켜져 있습니다. 둘째, 이 도구는 Codex 가 직접 실행하지 않습니다. 요청의 도구 목록에 `web_search` 를 끼워 보내면 OpenAI 서버가 검색합니다. 공급자를 사내 서버로 바꿔도 이 도구는 그대로 실려 갑니다. 사내 서버에는 이를 실행할 주체가 없습니다. 서버가 이 도구를 무시할지, 오류를 낼지는 엔진에 따라 다릅니다. 폐쇄망에서는 `web_search = "disabled"` 로 끕니다.

검색이 꼭 필요하면 사내 검색을 MCP 서버나 훅으로 붙입니다. 아래 두 절이 그 자리입니다.

### MCP 서버

```toml
[mcp_servers.my-search]            # 로컬 프로세스(stdio)
command = "python3"
args = ["/opt/my-search/server.py"]
env = { SEARCH_URL = "http://<사내 검색 주소>/api" }
startup_timeout_sec = 20
tool_timeout_sec = 60
enabled_tools = ["search"]

[mcp_servers.docs]                 # HTTP 서버
url = "https://<MCP 서버 주소>/mcp"
bearer_token_env_var = "DOCS_MCP_TOKEN"
enabled = false                    # 지우지 않고 잠시 끈다
```

| 키 | 설명 |
|---|---|
| `command`·`args`·`env`·`cwd` | stdio 서버 실행 방법 |
| `url`·`bearer_token_env_var`·`http_headers` | HTTP 서버 주소와 인증 |
| `enabled` | `false` 면 정의는 두고 끈다 |
| `required` | `true` 면 이 서버가 안 뜰 때 세션을 시작하지 않는다 |
| `startup_timeout_sec`·`tool_timeout_sec` | 시작·도구 호출 제한 시간 |
| `enabled_tools`·`disabled_tools` | 도구 일부만 열거나 막는다 |
| `tools.<도구>.approval_mode` | 도구별 승인 방식 |

`required` 는 실행 파일이 없는 서버로 확인했습니다. 세션이 `required MCP servers failed to initialize` 오류로 시작하지 않았습니다. 꼭 있어야 하는 사내 도구에만 붙입니다. 등록은 `codex mcp add` 로도 합니다. 결과는 같은 표로 남습니다. 자세한 쓰임은 5편에 있습니다.

### 훅

훅은 `hooks.json` 대신 `config.toml` 안에 적어도 됩니다. 아래는 프롬프트를 낼 때마다 스크립트를 먼저 돌리는 `UserPromptSubmit` 훅입니다. 이 형식은 `--strict-config` 를 통과했습니다.

```toml
[[hooks.UserPromptSubmit]]
[[hooks.UserPromptSubmit.hooks]]
type = "command"
command = "python3 /opt/hooks/my_search.py"
timeout = 15
```

0.170.0 스키마가 받는 이벤트는 14개입니다. `SessionStart`·`SessionEnd`·`UserPromptSubmit`·`PreToolUse`·`PermissionRequest`·`PostToolUse`·`PreCompact`·`PostCompact`·`PreModelRequest`·`PostModelRequest`·`SubagentStart`·`SubagentStop`·`Stop`·`Interrupt` 입니다. 공식 문서 기준으로 `UserPromptSubmit` 훅이 표준 출력에 쓴 글은 개발자 컨텍스트로 모델에 들어갑니다. 모델이 판단하기 전에 사내 검색 결과를 끼워 넣는 데 쓸 수 있습니다. 훅은 `/hooks` 화면에서 신뢰해야 실행됩니다. 이 과정은 4편에 정리했습니다. 이번에는 형식 검사까지만 했고, 신뢰 뒤 실행은 확인하지 않았습니다.

## 5. 지시문과 컨텍스트

| 키 | 설명 |
|---|---|
| `developer_instructions` | 개발자 메시지로 매 요청에 들어가는 지시문 |
| `model_instructions_file` | 내장 기본 지시문을 이 파일로 바꾼다 |
| `project_doc_max_bytes` | AGENTS.md 등 프로젝트 지시문을 읽는 최대 바이트 |
| `project_doc_fallback_filenames` | AGENTS.md 가 없을 때 대신 찾을 파일 이름 |
| `model_auto_compact_token_limit` | 자동 압축 시작 토큰 수 |
| `skills.max_context_tokens` | 스킬 목록에 쓸 토큰 상한 |

`developer_instructions = "답은 한국어로 한다."` 를 적고 요청을 보니, 이 문장이 개발자 메시지에 그대로 들어 있었습니다. 저장소마다 다른 규칙은 AGENTS.md 에 두고, 어디서나 같은 규칙만 이 키에 둡니다.

`skills.max_context_tokens` 는 3편에서 잰 스킬 목록 크기와 이어집니다. 스키마 설명은 기본값을 컨텍스트 창의 2%로 적습니다. 값을 직접 정하면 1만 토큰이 상한입니다. 스킬을 많이 깔았다면 이 값으로 상한을 둘 수 있습니다.

`model_instructions_file` 은 Codex 의 기본 지시문을 통째로 바꿉니다. 3편에서 잰 기본 지시문 약 2만 1천 자가 그 대상입니다. 도구 사용법과 안전 규칙도 그 안에 있으니, 바꿀 이유가 분명할 때만 씁니다.

## 6. 프로필

0.170.0 에서 프로필은 별도 파일입니다. `-p <이름>` 은 `$CODEX_HOME/<이름>.config.toml` 을 읽습니다. 세 경우를 돌려 봤습니다.

| 설정 | 실행 | 결과 |
|---|---|---|
| `p.config.toml` 에 `model = "file-model"` | `codex -p p` | 배너 `model: file-model`. 적용됨 |
| `config.toml` 안에 `[profiles.p]` 표 | `codex -p p` | 오류로 멈춤 |
| `config.toml` 안에 `[profiles.p]` 표 | `-p` 없이 | 표는 무시됨. 경고 없음 |
| 프로필 파일이 없음 | `codex -p nope` | 오류 없이 기본 설정으로 실행 |

옛 형식 표를 남겨 둔 채 `-p` 를 쓰면 이런 오류가 납니다.

```text
Error loading config.toml: --profile `p` cannot be used while ~/.codex/config.toml contains
legacy `profile = "p"` or `[profiles.p]` config; move those settings into ~/.codex/p.config.toml
and remove the legacy profile selector/table.
```

3편에서는 이 표를 "아무 일도 하지 않는 설정"이라고 적었습니다. 0.170.0 에서도 `-p` 없이 두면 그렇습니다. 하지만 `-p` 를 함께 쓰면 멈춥니다. 반대로 마지막 줄은 조용한 실패입니다. 프로필 이름을 틀리게 적어도 오류가 나지 않고 기본 설정으로 돕니다. 프로필로 열었다면 배너의 `model` 줄부터 봅니다.

## 7. 기록·화면·알림·업데이트

```toml
check_for_update_on_startup = false   # 시작할 때 새 버전 확인을 끈다
notify = ["/usr/local/bin/my-notify"] # 알림을 바깥 명령으로 보낸다

[history]
persistence = "save-all"              # 또는 "none"
max_bytes = 10485760                  # 넘으면 오래된 항목부터 지운다

[analytics]
enabled = false

[feedback]
enabled = false

[tui]
notifications = true
```

`[history]` 는 `~/.codex/history.jsonl` 에 남길지를 정합니다. 공용 서버처럼 프롬프트를 디스크에 남기기 싫은 곳에서는 `"none"` 을 씁니다. `[analytics]`·`[feedback]` 은 OpenAI 로 보내는 사용 통계와 피드백 업로드를 끕니다. `[tui]` 에는 키가 36개 있습니다. 자주 손대는 것은 상태 줄(`status_line`), 테마(`theme`), 알림 방식(`notification_method`) 정도입니다. 테마 이름은 세션 안의 `/theme` 에서 고릅니다.

기능 스위치는 `[features]` 표에 둡니다. 0.170.0 의 `codex features list` 는 215개였습니다. stable 68개, under development 90개, removed 48개, experimental 5개, deprecated 4개입니다. 3편에서 센 0.169.0 과 개수가 같았습니다. `hooks` 는 stable 이고 켜져 있었고, `memories` 는 stable 이지만 꺼져 있었습니다. 웹 검색의 옛 스위치 `web_search_request`·`web_search_cached` 는 deprecated 입니다. 웹 검색은 4절의 `web_search` 키로 정합니다.

## 8. 전체 예시 — 폐쇄망의 사내 추론 서버

지금까지의 키를 폐쇄망 기준으로 모은 파일입니다. 이 파일 그대로 `--strict-config` 를 통과했습니다. 가짜 서버로 보낸 요청에는 `web_search` 가 없었고, 개발자 메시지에 한국어 지시문이 들어 있었습니다.

```toml
# ~/.codex/config.toml — 폐쇄망의 사내 추론 서버용 (codex-cli 0.170.0)

# 모델과 공급자
model_provider = "inhouse"
model = "서버가-서빙하는-모델-이름"
model_reasoning_effort = "medium"
model_context_window = 131072
model_auto_compact_token_limit = 100000

# 권한
approval_policy = "on-request"
sandbox_mode = "workspace-write"

# 바깥으로 나가는 기능
web_search = "disabled"
check_for_update_on_startup = false

# 지시문
developer_instructions = "답은 한국어로 한다."

[model_providers.inhouse]
name = "사내 추론 서버"
base_url = "https://<추론 서버 주소>/v1"
env_key = "INHOUSE_API_KEY"
wire_api = "responses"
stream_idle_timeout_ms = 300000

[sandbox_workspace_write]
network_access = false

[analytics]
enabled = false

[feedback]
enabled = false

[history]
persistence = "save-all"
max_bytes = 10485760

[mcp_servers.my-search]
command = "python3"
args = ["/opt/my-search/server.py"]
env = { SEARCH_URL = "http://<사내 검색 주소>/api" }
startup_timeout_sec = 20
tool_timeout_sec = 60

[tui]
notifications = true
```

최상위 키는 모두 첫 `[표]` 보다 위에 둡니다. TOML 은 표 머리 아래에 적은 키를 그 표의 키로 읽습니다. `[tui]` 아래에 `web_search = "disabled"` 를 적으면 `tui.web_search` 가 되어 무시됩니다. 이 실수도 `--strict-config` 가 잡습니다.

`model_context_window` 의 131072 와 자동 압축 기준 100000 은 예시 값입니다. 서버의 `/v1/models` 가 알려 주는 최대 길이에 맞춰 고칩니다. `INHOUSE_API_KEY` 는 셸에서 내보냅니다. 키 값을 이 파일에 적지 않습니다.

## 정리

`config.toml` 은 최상위 키 103개짜리 파일이지만, 실제로 손대는 키는 모델·공급자, 권한, 웹 검색·MCP·훅, 지시문, 프로필 정도입니다. 0.170.0 에서는 옛 설정 세 가지가 오류로 바뀌었습니다. `wire_api = "chat"`, `approval_policy = "untrusted"`, `-p` 와 함께 쓴 `[profiles]` 표입니다. 반대로 프로필 이름 오타와 기본으로 켜진 웹 검색은 오류 없이 지나갑니다.

다음에 확인할 것은 하나입니다. 지금 쓰는 설정 파일을 `codex --strict-config` 로 한 번 열어 보는 것입니다. 그리고 배너의 `model`·`approval`·`sandbox` 줄이 의도와 같은지 봅니다.
