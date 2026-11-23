# dok3search 제품·운영 매뉴얼

## 개요

dok3search는 폐쇄망에서 동작하는 문서·코드 검색 서버다. 하나의 실행 파일(`dok3-server`)이 웹 화면, REST API, MCP(Model Context Protocol) 엔드포인트를 같은 포트에서 제공한다. 외부 네트워크나 CDN에 접속하지 않는다.

이 매뉴얼은 서버에 기본 문서로 내장되어 있다. 문서 ID는 `dok3search-manual`, 분류는 `manual`(운영 매뉴얼)이다. 운영자와 AI 에이전트는 이 문서를 dok3search 자체 검색으로 찾고 절 단위로 읽을 수 있다. 자세한 방법은 "운영 매뉴얼 자체 검색" 절에 있다.

이 매뉴얼은 저장소 `main` 브랜치에 구현된 동작만 설명한다. 계획 중이거나 설계 문서에만 있는 기능은 다루지 않는다.

## 빠른 시작

저장소 루트에서 다음 순서로 실행한다.

```sh
npm --prefix frontend run build
cargo run --manifest-path backend/rust/Cargo.toml -p dok3-server
```

환경 변수를 지정하지 않으면 `http://127.0.0.1:8080`에서 `in-memory-fake` 모드로 기동한다. 기동에 성공하면 표준 출력에 다음 형식의 한 줄이 찍힌다.

```text
dok3search listening on http://127.0.0.1:8080 (docs=<문서 수>, static=<정적 폴더 또는 disabled>)
```

준비 상태는 `GET /health/ready`로 확인한다.

## 환경 변수

| 변수 | 기본값 | 설명 |
| --- | --- | --- |
| `DOK3_BIND` | `127.0.0.1` | HTTP 바인드 주소. 빈 값이면 기동하지 않는다. |
| `DOK3_PORT` | `8080` | HTTP 포트. 1~65535 범위의 정수만 허용한다. |
| `DOK3_STATIC_DIR` | 자동 탐색 | React 빌드 결과(`frontend/dist`) 경로. |
| `DOK3_DESIGN_DIR` | 자동 탐색 | 설계 문서 사본(`frontend/src/samples/design`) 경로. `in-memory-fake` 모드에서만 쓴다. |
| `DOK3_SOURCE_DIR` | 없음 | 시작할 때 색인할 Markdown 폴더. |
| `DOK3_DATA_DIR` | 없음 | 영속 저장소와 색인 세대를 둘 폴더. `DOK3_SOURCE_DIR`과 함께 써야 한다. |
| `DOK3_ALLOW_NON_LOOPBACK_BIND` | `false` | `true`/`1`이면 `DOK3_SOURCE_DIR` 모드에서 비-loopback `DOK3_BIND`를 허용한다. 인증 프록시 뒤 컨테이너 배포에서만 켠다. |

값 앞뒤의 공백은 무시한다. 경로 변수에 빈 문자열을 주면 지정하지 않은 것으로 본다.

`DOK3_STATIC_DIR`을 지정하지 않으면 현재 작업 폴더 기준으로 `frontend/dist`, `../../frontend/dist`를 차례로 살펴 `index.html`이 있는 첫 폴더를 쓴다. 찾지 못하면 웹 화면 없이 API와 MCP만 제공한다. `DOK3_DESIGN_DIR`도 같은 방식으로 `frontend/src/samples/design`, `../../frontend/src/samples/design`을 찾는다.

## 기동 모드

서버는 `DOK3_SOURCE_DIR`과 `DOK3_DATA_DIR` 조합으로 기동 모드를 정한다. 현재 모드는 `/health/ready` 응답의 `mode` 필드에 나온다.

| `DOK3_SOURCE_DIR` | `DOK3_DATA_DIR` | 모드 | 동작 |
| --- | --- | --- | --- |
| 지정 | 지정 | `tantivy-persistent` | 원본 폴더를 영속 저장소에 동기화하고, 디스크의 Tantivy 색인 세대로 검색한다. |
| 지정 | 없음 | `tantivy-local` | 원본 폴더를 매번 읽어 메모리 Tantivy 색인을 새로 만든다. |
| 없음 | 지정 | 기동 실패 | `DOK3_DATA_DIR requires DOK3_SOURCE_DIR` 오류로 종료한다. |
| 없음 | 없음 | `in-memory-fake` | 내장 샘플 문서, 설계 문서 사본, 이 매뉴얼을 메모리에 올려 검색한다. |

`DOK3_SOURCE_DIR`을 지정하면 `DOK3_BIND`는 기본적으로 IPv4 루프백 주소(`127.0.0.1`, `127.0.0.2` 등)여야 한다. `0.0.0.0`, `localhost`, `::1`이면 `DOK3_SOURCE_DIR requires an IPv4 loopback DOK3_BIND` 오류로 종료한다. 인증 프록시 뒤 컨테이너에서 `0.0.0.0` 바인드가 필요하면 `DOK3_ALLOW_NON_LOOPBACK_BIND=true`를 명시한다. 관리자 API 바인드는 이 플래그와 무관하게 항상 loopback만 허용한다.

이 매뉴얼(`dok3search-manual`)은 `in-memory-fake` 모드의 기본 문서다. `tantivy-local`과 `tantivy-persistent` 모드는 원본 폴더의 문서만 색인한다. 두 모드에서도 매뉴얼을 검색하려면 `docs/manual/dok3search-manual.md`를 원본 폴더에 복사한다.

## 원본 폴더 색인 규칙

`tantivy-local`과 `tantivy-persistent` 모드는 `DOK3_SOURCE_DIR` 아래 파일을 다음 규칙으로 색인한다.

- 확장자가 `md`, `markdown`, `mdown`, `mkd`인 파일만 읽는다.
- 파일 하나의 최대 크기는 4 MiB다.
- 모든 문서의 분류는 `guide`, 출처 ID는 `local-markdown`, 접근 범위는 공개(`public`)다.
- 언어는 본문을 보고 판정한다.
- 읽을 수 없거나, 비어 있거나, UTF-8이 아니거나, 파싱에 실패한 파일은 건너뛴다. 나머지 문서 색인은 계속한다.

색인 결과는 표준 오류에 기록한다. 표준 출력은 `--mcp-stdio` 모드의 JSON-RPC 전용이기 때문이다.

```text
local Markdown snapshot: indexed=<색인 수> skipped=<건너뛴 수>
local Markdown skipped: <상대 경로>: <사유>
```

`tantivy-persistent` 모드는 첫 줄의 접두어가 `persistent Markdown snapshot`이다.

## HTTP API

모든 응답에는 보안 헤더 `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, `Referrer-Policy: no-referrer`, `Content-Security-Policy`(`frame-ancestors 'none'` 포함)가 붙는다.

### 준비 상태와 코퍼스 카탈로그

`GET /health/ready`는 다음 필드를 돌려준다.

- `status`: 기동이 끝났으면 `ready`
- `mode`: `tantivy-persistent`, `tantivy-local`, `in-memory-fake` 중 하나
- `document_count`: 저장소의 문서 수
- `snapshot`: 검색 세대 정보(`generation_id`, `epoch`, `schema_version`, `acl_policy_version`, `applied_change_sequence`, `reader_state`)

`tantivy-persistent` 모드에서 활성 세대가 없으면 `snapshot.generation_id`는 `none`, `reader_state`는 `degraded`다.

`GET /api/v1/corpora`는 현재 접근 권한(ACL)으로 열람 가능한 코퍼스 목록(`corpus_id`, `label`, `category`, `topic`, `version`, `toolchain`, `license`, `document_count`, `packages`)을 돌려준다. 관리자 권한 없이 조회할 수 있다.

### 검색

`POST /api/v1/search`에 JSON 본문을 보낸다.

```json
{
  "query": "운영 매뉴얼",
  "filters": { "category": "manual", "language": null, "source_id": null, "tags": [] },
  "page": 1,
  "limit": 15,
  "include_snippets": true
}
```

- 모든 필드는 생략할 수 있다. `query`가 비면 필터에 맞는 전체 문서를 돌려준다.
- `query`는 최대 512자다. 넘으면 `400 INVALID_INPUT`이다.
- `page`는 1 이상으로, `limit`은 1~100으로 맞춘다. `limit`이 0이면 15를 쓴다.
- `filters.tags`에 여러 태그를 주면 모든 태그가 맞는 문서만 남긴다.
- 응답의 `category_counts`와 `total_unfiltered_hits`는 분류 필터를 적용하기 전 값이다.
- `requested_scope`는 범위를 좁히는 용도로만 쓴다. 호출자 권한을 넓히지 못한다.

### 문서와 절 조회

- `GET /api/v1/documents/{id}`: 문서 메타데이터, 본문(`body` 또는 `markdown`), 절 목록(`sections`)을 돌려준다.
- `GET /api/v1/documents/{id}/sections/{sid}`: 절 본문을 잘라서 돌려준다. 쿼리 파라미터 `cursor`(문자 단위 시작 위치)와 `max_chars`(기본 12000, 최대 100000)를 받는다. 남은 본문이 있으면 응답의 `truncated`가 `true`이고 `next_cursor`에 다음 시작 위치가 있다.

절 ID는 `##` 제목에서 만든다. 제목을 소문자로 바꾸고, 글자와 숫자가 아닌 문자열을 `-` 하나로 바꾸고, 끝의 `-`를 지운 뒤 `section-`을 앞에 붙인다. 예를 들어 이 매뉴얼의 "백업과 복구" 절 ID는 `section-백업과-복구`다.

### 오류 응답

REST 오류는 `{"error": {"code": "...", "message": "..."}}` 형식이다.

| 코드 | HTTP 상태 | 뜻 |
| --- | --- | --- |
| `INVALID_INPUT` | 400 | 잘못된 JSON, 쿼리 파라미터, 입력 길이 |
| `FORBIDDEN` | 403 | 호출자 권한으로 열람할 수 없는 문서 |
| `NOT_FOUND` | 404 | 없는 문서·절, 정의되지 않은 `/api` 경로 |
| `PARSE_ERROR`, `INDEX_ERROR`, `STORAGE_ERROR`, `INTERNAL_ERROR` | 500 | 서버 내부 오류 |

## MCP 연동

dok3search는 MCP 서버를 두 가지 전송 방식으로 제공한다. 두 방식 모두 HTTP API와 같은 검색 서비스를 쓴다.

### Streamable HTTP

`POST /mcp`에 JSON-RPC 2.0 메시지를 보낸다. HTTP 서버와 같은 주소·포트를 쓴다.

- 요청 헤더에 `Content-Type: application/json`, `Accept: application/json, text/event-stream`을 넣는다.
- `GET /mcp`는 `405 Method Not Allowed`(`Allow: POST`)를 돌려준다.
- `Origin` 헤더가 있으면 호스트가 `localhost` 또는 `127.0.0.1`이어야 한다. 다른 출처는 `403`으로 거부한다.
- 응답에 협상된 프로토콜 버전이 `MCP-Protocol-Version` 헤더로 붙는다. 지원 버전은 `2026-07-28`(기본), `2025-11-25`, `2024-11-05`다.
- 알림(id 없는 메시지)에는 본문 없이 `202 Accepted`로 응답한다.

```sh
curl -s http://127.0.0.1:8080/mcp \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"kb_search","arguments":{"query":"운영 매뉴얼","category":"manual"}}}'
```

### stdio

`--mcp-stdio` 인자로 실행하면 HTTP 포트를 열지 않고 표준 입출력으로 MCP를 제공한다.

```sh
cargo run --manifest-path backend/rust/Cargo.toml -p dok3-server -- --mcp-stdio
```

- 표준 입력에서 한 줄에 JSON-RPC 메시지 하나를 읽고, 응답도 한 줄씩 표준 출력에 쓴다.
- 알림에는 응답을 쓰지 않는다.
- 진단 메시지는 표준 오류에만 쓴다. 준비가 끝나면 `dok3search MCP stdio ready (docs=<문서 수>, mode=<모드>)`가 찍힌다.
- 표준 입력이 닫히면 종료한다.
- `--mcp-stdio` 외의 인자를 주면 `unknown argument` 오류로 종료한다. 오타로 HTTP 모드가 뜨는 일을 막기 위해서다.

## MCP 도구

`tools/list`는 다음 7개 도구를 돌려준다.

| 도구 | 용도 | 필수 인자 |
| --- | --- | --- |
| `kb_search` | 권한 안의 문서를 검색한다. | 없음 |
| `kb_get_document` | 문서 메타데이터, 본문, 절 목록을 읽는다. | `document_id` |
| `kb_get_section` | 절 본문을 커서 단위로 읽고 출처를 돌려준다. | `document_id`, `section_id` |
| `kb_search_api` | REST·API 계약과 엔드포인트를 검색한다. | 없음 |
| `kb_search_error` | 오류 코드와 장애 조치 문서를 검색한다. | 없음 |
| `kb_search_code` | 코드 심볼과 예제를 검색한다. | 없음 |
| `kb_list_categories` | 권한 안의 분류별 문서 수를 돌려준다. | 없음 |

검색 계열 도구(`kb_search`, `kb_search_api`, `kb_search_error`, `kb_search_code`)는 공통 인자 `category`, `language`, `source_id`, `tags`, `page`, `limit`(기본 15, 최대 100), `include_snippets`, `requested_scope`를 받는다. 도구별 추가 인자는 `kb_search`의 `query`, `kb_search_api`의 `query`·`endpoint`, `kb_search_error`의 `error_code`·`query`, `kb_search_code`의 `symbol`·`query`다.

`kb_get_section`은 `cursor`(0 이상)와 `max_chars`(1~100000, 기본 12000)를 받는다. 응답에는 `heading_path`(문서 제목, 절 제목), `citation`(`source_uri`, `section`), `source_revision`이 들어 있다. 에이전트가 답변에 출처를 붙일 때 이 값을 쓴다.

## 접근 제어

현재 인증 계층이 없다. 그래서 HTTP MCP와 stdio MCP의 모든 호출자를 익명 공개 사용자로 취급한다. 익명 공개 사용자는 접근 범위가 `public`인 문서만 검색하고 열람한다.

- 검색 결과와 분류별 문서 수에서 권한 밖 문서를 뺀다.
- 권한 밖 문서를 ID로 직접 조회하면 `403 FORBIDDEN`이다.
- `requested_scope`로 `internal:<이름>` 범위를 요청해도 권한이 넓어지지 않는다. 결과가 0건이 된다.

내장 샘플 문서, 설계 문서 사본, 이 매뉴얼, 원본 폴더 문서는 모두 `public` 범위로 적재된다.

## 에어갭 열람 규칙

dok3search는 외부 네트워크가 없는 환경을 전제로 한다.

- 웹 화면은 글꼴, 아이콘, 샘플 문서를 모두 빌드 결과에 포함한다. 외부 CDN을 부르지 않는다.
- Markdown 본문의 외부 URL은 클릭할 수 있는 링크로 그리지 않는다. 폐쇄망에서 열 수 없는 출처로 표시한다.
- 개발 서버(`npm --prefix frontend run dev`)는 `/api`와 `/health` 요청을 `DOK3_API_URL`(기본 `http://127.0.0.1:8080`)로 넘긴다.
- MCP HTTP 엔드포인트는 루프백 `Origin`만 받는다. 브라우저에서 다른 사이트가 `/mcp`를 호출하지 못한다.
- 원본 폴더 색인은 IPv4 루프백 바인드에서만 허용한다. 색인한 내부 문서를 외부 인터페이스로 노출하지 않기 위해서다.

## 영속 데이터 디렉터리

`tantivy-persistent` 모드는 `DOK3_DATA_DIR` 아래에 다음 구조를 만든다.

```text
<DOK3_DATA_DIR>/
  storage/
    documents.json            문서 메타데이터
    journal.jsonl             변경 일지(추가·수정·삭제 순번)
    objects/                  원본 파일 사본
  generations/
    generation_state.json     세대 상태와 활성 세대
    gen-000001-seq-000003/    색인 세대(이름은 세대 번호와 일지 순번)
      manifest.json
      tantivy/
```

기동할 때마다 다음 순서로 처리한다.

1. 원본 폴더를 읽어 `storage`에 반영한다. 바뀐 문서만 저장하고 일지에 추가·수정 사건을 남긴다. 원본 폴더에서 사라진 문서는 저장소에서 지우고 삭제 사건을 남긴다.
2. 중단된 세대 전환이 있으면 복구한다.
3. 활성 세대의 일지 순번과 문서 수가 저장소와 같으면 그 세대를 그대로 쓴다.
4. 다르면 새 세대를 만들고 검증한 뒤 활성화한다. 이전 세대는 퇴역시키고 파일을 지운다.

새 세대는 활성화 전에 매니페스트 무결성, 스키마·분석기 버전, 일지 순번, 문서 수, 공개 사용자 권한 누출 여부를 검사한다. 검사를 통과하지 못한 세대는 활성화하지 않는다.

## 스키마 버전과 재색인

현재 Tantivy 색인 스키마 버전(`SCHEMA_VERSION`)은 2이고 분석기 버전은 1이다. 스키마 버전은 색인 필드나 토크나이저가 호환되지 않게 바뀔 때 올린다. 현재 토크나이저 이름은 `dok3_text_v2`, `dok3_ngram_v2`다.

- `tantivy-local`과 `in-memory-fake` 모드는 기동할 때마다 색인을 새로 만든다. 따로 재색인할 필요가 없다.
- `tantivy-persistent` 모드는 디스크의 세대를 다시 쓴다. 스키마 버전이 다른 바이너리로 교체하면 기존 세대를 열 수 없어 `failed to open/create Tantivy index (expected schema v2)` 오류로 기동이 실패할 수 있다.

스키마 버전이 바뀐 바이너리로 교체할 때는 다음 순서로 재색인한다.

1. 서버를 멈춘다.
2. `<DOK3_DATA_DIR>/generations` 폴더를 다른 이름으로 옮긴다. `storage`는 그대로 둔다.
3. 새 바이너리로 기동한다. 활성 세대가 없으므로 저장소 기준으로 새 세대를 만든다.
4. `/health/ready`에서 `snapshot.schema_version`이 2이고 `document_count`가 예상과 같은지 확인한다.
5. 검색이 정상이면 옮겨 둔 이전 `generations` 폴더를 지운다.

## 백업과 복구

문서의 원본은 `DOK3_SOURCE_DIR`이다. 서버는 기동할 때마다 원본 폴더를 기준으로 저장소를 맞춘다. 따라서 원본 폴더가 1차 백업 대상이다. `DOK3_DATA_DIR`은 원본에서 다시 만들 수 있는 파생 데이터지만, 백업해 두면 재기동이 빠르고 세대 이력을 보존할 수 있다.

### 백업

1. 원본 폴더를 정기적으로 백업한다.
2. `DOK3_DATA_DIR`을 백업하려면 서버를 멈춘 뒤 폴더 전체(`storage`와 `generations`)를 한꺼번에 복사한다. 실행 중에 복사하면 일지와 세대 상태가 서로 다른 시점을 가리킬 수 있다.

### 복구

1. 서버를 멈춘다.
2. 원본 폴더를 복원한다.
3. 다음 중 하나를 고른다.
   - 백업한 `DOK3_DATA_DIR`을 같은 경로에 복원한다.
   - `DOK3_DATA_DIR`을 빈 폴더로 둔다. 서버가 원본 폴더에서 저장소와 첫 세대를 새로 만든다.
4. 기동 후 표준 오류의 `persistent Markdown snapshot: indexed=… skipped=…` 줄과 `/health/ready`의 `document_count`, `snapshot.generation_id`를 확인한다.
5. 대표 검색어 몇 개로 결과를 확인한다.

복원한 저장소에 있지만 원본 폴더에는 없는 문서는 기동할 때 저장소에서 지워진다. 원본 폴더를 먼저 복원해야 하는 이유다.

## 장애 진단

| 증상 | 원인과 조치 |
| --- | --- |
| `DOK3_DATA_DIR requires DOK3_SOURCE_DIR` | `DOK3_SOURCE_DIR`을 함께 지정한다. |
| `DOK3_SOURCE_DIR requires an IPv4 loopback DOK3_BIND` | `DOK3_BIND`를 `127.0.0.1`로 바꾸거나, 인증 프록시 뒤 컨테이너라면 `DOK3_ALLOW_NON_LOOPBACK_BIND=true`를 명시한다. |
| `invalid DOK3_PORT value` 또는 `DOK3_PORT must be between 1 and 65535` | 1~65535 범위의 정수로 지정한다. |
| `failed to bind <주소>` | 포트를 다른 프로세스가 쓰고 있다. 포트를 바꾸거나 기존 프로세스를 멈춘다. |
| `unknown argument` | 지원하는 인자는 `--mcp-stdio` 하나다. |
| 문서 수가 예상보다 적다 | 표준 오류의 `local Markdown skipped:` 줄에서 건너뛴 파일과 사유를 확인한다. |
| `failed to open/create Tantivy index` | 스키마가 다른 세대가 남아 있다. "스키마 버전과 재색인" 절의 순서로 재색인한다. |
| `/mcp` 호출이 403 | 요청의 `Origin`이 루프백 주소가 아니다. |
| 웹 화면이 뜨지 않는다 | 기동 로그의 `static=disabled`를 확인한다. 프런트엔드를 빌드하거나 `DOK3_STATIC_DIR`을 지정한다. |

## 운영 매뉴얼 자체 검색

`in-memory-fake` 모드에서는 이 매뉴얼이 문서 ID `dok3search-manual`로 적재된다. 다른 운영 문서와 같은 `manual` 분류에 속한다.

웹 화면에서는 "운영 매뉴얼" 분류를 열거나 "운영 매뉴얼"로 검색한다.

REST API로는 다음과 같이 찾고 읽는다.

```sh
curl -s http://127.0.0.1:8080/api/v1/search \
  -H 'Content-Type: application/json' \
  -d '{"query":"백업","filters":{"category":"manual"}}'
curl -s http://127.0.0.1:8080/api/v1/documents/dok3search-manual
curl -s 'http://127.0.0.1:8080/api/v1/documents/dok3search-manual/sections/section-%EB%B0%B1%EC%97%85%EA%B3%BC-%EB%B3%B5%EA%B5%AC'
```

URL 경로의 한글 절 ID는 퍼센트 인코딩해서 보낸다. 위 예시는 `section-백업과-복구`를 인코딩한 값이다.

MCP 에이전트는 `kb_search`로 `category: "manual"`을 주고 찾은 뒤, `kb_get_document`로 절 목록을 받아 `kb_get_section`으로 필요한 절만 읽는다. 절 본문이 길면 `next_cursor`를 다음 호출의 `cursor`로 넘긴다.
