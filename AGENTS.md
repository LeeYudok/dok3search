**기계 판독용 정보는 [`AGENTS.yaml`](AGENTS.yaml)** — 구성요소 그래프(`nodes`/`edges`), `commands`, `policies`, `known_issues` 등. 작업 전 이 파일을 먼저 읽고, 구성요소·경로·명령·환경변수가 바뀌면 이 문서와 **함께** 갱신한다. 두 파일이 어긋나면 실제 코드·파일시스템을 확인해 둘 다 고친다. 고친 뒤엔 `scripts/check-agents-yaml.sh` 로 검증한다. 날짜·버전 값은 항상 따옴표로 감싼다.

# dok3search 에이전트 지침

폐쇄망 환경에서 동작하는 문서·코드 검색 플랫폼(`dok3search`) 모노레포다. 다중 백엔드 확장을 대비해 언어별 백엔드는 `backend/` 아래에 두고, 현재 백엔드 워크스페이스는 `backend/rust/`에 위치한다.

## 짝 설계 저장소 (`dok3search-design`)

아키텍처·데이터 계약·운영 프로토콜의 원본 설계 SSOT는 별도 저장소 `dok3search-design`과 한 세트로 관리한다.

- 로컬 경로: `/Users/dok123/workspace/github.com/LeeYudok/dok3search-design`
- 원격 저장소: `https://github.com/LeeYudok/dok3search-design`
- 진입점: `index.md`, `index.yaml`, `map.md`, `map.yaml`, `RUST_WORKSPACE_AND_CONTRACTS.md`, `dok3search-implementation-roadmap.md`

화면·API 계약·검색 동작을 변경할 때는 `dok3search-design`의 계약 예시(`contracts/*.example.yaml`) 및 `frontend/src/samples/design/`과의 정합성을 함께 확인한다.

설계 변경 시 고정 동기화 절차:
1. `dok3search-design`에서 설계 문서·계약을 먼저 갱신하고 PR 머지한다.
2. `dok3search`의 `frontend/src/samples/design/` 사본과 `provenance.json`을 최신 커밋 기준으로 갱신한다.
3. `python3 scripts/check-design-copy.py --source ../dok3search-design` 통과를 확인한 뒤 `dok3search` PR을 올린다.

설계 사본의 원본 커밋과 파일별 SHA-256은 `frontend/src/samples/design/provenance.json`에 기록한다. 사본을 갱신한 뒤 `python3 scripts/check-design-copy.py --source ../dok3search-design`로 원본과 대조한다. 사본만 있는 환경에서는 `--source`를 빼고 무결성을 검사한다.

## 디렉터리 구조

- `frontend/` — React + TypeScript + Vite 웹 UI (로컬 폰트·샘플 데이터 포함, 외부 CDN 미사용, 개발 프록시 `/api`·`/health`, 문서 분류 편집 및 `/admin` 운영 관리 화면 포함). Markdown의 외부 URL은 클릭 가능한 링크로 렌더하지 않고 폐쇄망에서 열 수 없는 출처로 표시한다.
- `frontend/src/samples/{comprehensive-rust,rust-book,rust-reference,cargo-book,go-reference}/chapters/` — Rust와 Go 참고 자료 검색에 쓰는 장별 Markdown. 메인 `개발 가이드` 분류 안에서 Rust·Go 세부 분류로 나눈다. Comprehensive Rust와 `go-reference/chapters/practice/`의 Go 실무 가이드는 한국어이고 나머지는 영문이다. `go-reference/chapters/learn-go-with-tests/`에는 MIT 라이선스의 영문 실습 3편을 담는다.
- `frontend/src/samples/{rust-std,rust-crates,rustc-error-codes}/chapters/` — 로컬 `rust-docs`(`std`·`core`·`alloc`), 워크스페이스·직접 의존 크레이트(`cargo doc`), `rustc --explain` 오류 코드 인덱스를 변환한 `code`/`rust` Markdown 코퍼스다. `scripts/import-rustdoc-and-errors.py`로 재생성한다.
- `frontend/src/samples/doksam-blog/chapters/` — `blog.doksam.com`(`blog-doksam`)에서 선별한 Rust·Go 강의, AI 에이전트 가이드, 폐쇄망/개발 실무 Markdown 코퍼스다. `scripts/import-doksam-blog.py`로 재생성한다.
- `frontend/src/samples/kordoc/src/` — `LeeYudok/kordoc`(MIT) HWP·HWPX·PDF → Markdown 파서의 `src/**` TypeScript 소스 전용 코퍼스다. `scripts/import-kordoc-source.py`로 재생성한다.
- `backend/rust/` — Cargo workspace (`crates/` 아래 13개 크레이트: `dok3-core` ~ `dok3-server`)
- `config/` — 환경별 설정 예시 및 Google AX(`ax-workspace.example.yaml`) 매니페스트 배치 위치
- `deploy/windows/` — Windows Service 등록·제거·자동 복구·smoke PowerShell 스크립트 배치 위치
- `contracts/` — REST·MCP·도메인 계약 정의 배치 위치
- `docs/` — 아키텍처·ADR·운영 매뉴얼 배치 위치; `docs/manual/dok3search-manual.md`은 자체 운영 매뉴얼 Knowledge Pack을 자동 색인한다.
- `tests/` — 워크스페이스 단위 외 통합·장애 주입·성능 테스트 배치 위치
- `scripts/` — 저장소 검증·유틸리티 스크립트 (`check-agents-yaml.sh`)

## 의존성 방향 규칙

- `dok3-domain`은 외부 크레이트 구현(Tantivy, Axum, SQLite 드라이버 등)에 의존하지 않는다.
- `dok3-parser`는 `dok3-index`를 직접 의존하거나 수정하지 않는다.
- `dok3-index-manager`는 `dok3-storage`와 `dok3-index`를 조합해 세대 빌드·전환·복구를 담당한다.
- `dok3-api`와 `dok3-agent`는 `dok3-search` 응용 서비스를 호출하며 도메인 로직을 중복 구현하지 않는다.

## 실행 환경 변수 및 기동 모드 (`dok3-server`)

- `DOK3_BIND`: HTTP 바인드 주소 (기본 `127.0.0.1`)
- `DOK3_PORT`: HTTP 포트 (기본 `8080`)
- `DOK3_DATA_DIR`: 영속 스토어 루트 경로. 세대 전환은 `tantivy-persistent` 백엔드를 사용한다.
- `DOK3_STATIC_DIR`: 정적 React 빌드(`frontend/dist`) 경로 (미지정 시 자동 탐색)
- `DOK3_DESIGN_DIR`: 설계 문서 시드(`frontend/src/samples/design`) 경로 (미지정 시 자동 탐색)

- `DOK3_SOURCE_DIR`: 시작할 때 색인할 기본 문서 폴더(`local-markdown`, `managed_by_env: true`). `dok3-parser::ParserRegistry`로 Markdown(`md`·`markdown`·`mdown`·`mkd`), HTML(`html`·`htm`), 코드(`rs`·`java`·`go`·`sql`·`jsp`·`ts`·`tsx`·`mts`·`cts`·`js`·`jsx`·`mjs`·`cjs`·`py`), 텍스트(`txt`·`log`)를 읽는다(`DEFAULT_DOCUMENT_EXTENSIONS`; `json`·`yaml` 등은 코퍼스 매니페스트 오염 방지를 위해 제외). 코드·텍스트는 열람 화면에서 펜스 코드 블록으로 보인다. 설정하면 IPv4 루프백에서만 열리며, `DOK3_DATA_DIR` 없이 기동할 때는 메모리 검색 스냅샷을 구성한다. `DOK3_DATA_DIR` 와 함께 설정하면 `sources.json` 다중 Source 레지스트리와 영속 저장소(`dok3-storage`)·Blue-Green 세대(`dok3-index-manager`)를 사용한다. 읽을 수 없거나 비어 있거나 UTF-8이 아닌 파일은 경로와 사유를 기록하고 나머지 문서 색인은 계속한다.
- `DOK3_REFRESH_INTERVAL_SECS`: 영속 모드(`DOK3_SOURCE_DIR` + `DOK3_DATA_DIR`)에서 주기적 증분 색인 워커 실행 간격(초, 선택). 미설정 시 백그라운드 워커는 비활성.
- `DOK3_ENABLE_ADMIN`: `true` 설정 시 IPv4 루프백 바인드에서 관리자 API(`/api/v1/index/jobs*`, `/api/v1/sources*`, `PATCH /api/v1/documents/{id}/classification`) 및 `/admin` 관리 UI를 활성화한다(기본 `false`, 루프백 `Origin`·`Sec-Fetch-Site`·`Content-Type: application/json` 가드 강제).
- `DOK3_ALLOW_NON_LOOPBACK_BIND`: `true`/`1` 설정 시 `DOK3_SOURCE_DIR` 모드에서 비-loopback `DOK3_BIND`(예: 컨테이너의 `0.0.0.0`)를 명시적으로 허용한다(기본 `false`, true/false/1/0만 허용). 관리자 API 바인드는 이 플래그와 무관하게 항상 loopback만 허용한다. 인증 프록시 뒤 컨테이너 배포에서만 켠다.
- `DOK3_ANALYZER_PROFILE`: 한국어 형태소 분석 프로파일(`builtin`, `lindera`, `nori-mixed`, `nori-discard`). `DOK3_DICT_PATH`가 있으면 기본 `nori-mixed`, 없으면 `builtin`. 프로파일·사전 변경 시 영속 세대를 자동 재구축한다.
- `DOK3_DICT_PATH`: 컴파일된 `mecab-ko-dic` 바이너리(`ko-dic.bin`) 경로(`lindera`/`nori-*` 프로파일 필수). `dok3-server --compile-kodic <src_dir> <out_bin>`으로 생성한다.
- `DOK3_USER_DICT_PATH`: 선택적 `mecab-ko-dic` 사용자 사전 CSV 경로.
- `DOK3_CLASSIFIER_ENDPOINT`: 오프라인 자동 분류기 OpenAI 호환 로컬/LAN 엔드포인트(`http://` 루프백·RFC1918 IP만 허용).
- `DOK3_CLASSIFIER_MODEL`: 오프라인 자동 분류기 모델명(기본 `qwen2.5-3b-instruct`).

- `--mcp-stdio`: MCP 서버를 표준입출력 모드로 시작한다.
- `--acp-stdio`: ACP 어댑터를 표준입출력 모드로 시작한다(`POST /acp`는 HTTP 모드 엔드포인트, `Principal::anonymous_public()` 고정).
- `--windows-service`: Windows 네이티브 SCM(Service Control Manager) 서비스로 실행한다(`deploy/windows/install-service.ps1` 의 `sc.exe create` 에서 사용). `POST /mcp`는 HTTP 모드의 MCP 엔드포인트다. 두 경로 모두 `Principal::anonymous_public()` 고정을 사용한다.

## doksam 배포 (`deploy/doksam/`)

dok3search를 단일 노드 doksam Kubernetes에 재현 가능하게 배포하고 `https://www.doksam.com` Host로 서비스한다. 자세한 절차·경로·롤백 리허설은 [deploy/doksam/README.md](deploy/doksam/README.md)에 둔다.

- 공개 진입점은 인증 없이 접근 가능한 공개 서비스(`www.doksam.com`)로 운영한다.
- 컨테이너는 `0.0.0.0:8080`에 바인드하고 ClusterIP Service로만 노출한다. jex3 등이 쓰는 host port(`192.168.113.220:8081` 계열)는 재사용하지 않는다.
- TLS Host는 `www.doksam.com`, 기존 cert-manager ClusterIssuer `letsencrypt-dns01`(DNS-01)을 사용한다.
- 이미지는 커밋 SHA 기반 태그(`dok3search:sha-<git-sha>`)로 빌드하고 레지스트리 push 없이 노드에 import한다(`imagePullPolicy: IfNotPresent`).
- `SUDO_PASS`는 doksam 런타임 환경 변수나 stdin에서 읽어 `sudo -S`에만 파이프하고, `sudo`/`curl` argv에는 비밀번호를 넣지 않는다.
- 롤백은 Deployment 이미지만 직전 값으로 되돌리며 Namespace/PVC 등 운영 데이터를 파괴하지 않는다. 배포·롤백은 모두 idempotent해야 한다.
- `DOK3_SOURCE_DIR` 모드의 loopback 제한은 컨테이너에서 `DOK3_ALLOW_NON_LOOPBACK_BIND=true`로 명시 해제한다.

## 관리자 및 분류 API 엔드포인트

- `GET/POST /api/v1/index/jobs`, `GET /api/v1/index/jobs/{job_id}`: 영속 색인 작업 이력 조회 및 증분/전체(또는 특정 `source_id`) 색인 트리거 (`DOK3_ENABLE_ADMIN=true` 필요).
- `GET/PATCH /api/v1/documents/{id}/classification`: 문서 분류 조회(ACL 준수) 및 수동 수정/`reset_to_rule` 초기화 (원본 바이트·해시 불변, `PATCH`는 관리자 가드 적용).
- `GET /api/v1/documents/{id}/original`: ACL 및 SHA-256 무결성 검증 후 `application/octet-stream` 첨부 내려받기.
- `POST /api/v1/search/analyze`: 활성 분석기 프로파일 토큰·n-gram·정규화 질의어 반환(`/admin/analyzers`).
- `GET/POST /api/v1/sources`, `DELETE /api/v1/sources/{id}`: 로컬 디렉터리(`local_dir`) 및 SQLite(`sqlite`, `source_cursors.json` 증분 커서 체크포인트) Source 등록·목록 조회·삭제 (`local-markdown` 삭제 금지, Source 삭제 시 디스크 원본·DB 파일 불변, 관리자 가드 적용).
- `GET /api/v1/corpora`: 현재 접근 권한(ACL)으로 열람 가능한 코퍼스별 문서 수·툴체인·패키지 버전·라이선스·출처 카탈로그 공개 조회(관리자 권한 불필요).
- `GET /api/v1/search/recommendations`: 현재 접근 권한(ACL)으로 열람 가능한 코퍼스·카테고리·접두어 기반 추천 검색어 목록 공개 조회(`q`, `category`, `limit`).

## 검색 섹션 매칭 (`matched_section_id`)

- `dok3_analyzer::select_matched_section_id`가 빈 H1 서문(`body` 공백)을 건너뛰고 질의어와 제목·본문 점수가 가장 높은 비어 있지 않은 하위 섹션을 `matched_section_id`로 반환한다(`dok3-index`와 `dok3-search` 공통).

## 협업 및 머지 규칙

- 이슈에는 `owner:<agent_id>` 라벨과 담당 코멘트를 먼저 남기고, `batch/<agent>-<YYYYMMDD>-<8hex>` 브랜치에서 작업한다.
- PR 본문에 `Closes #N`으로 이슈 연결을 명시한다.
- 자동 머지는 타 에이전트 교차 리뷰와 필수 CI 통과를 모두 만족할 때만 현재 지휘자가 `--merge`로 즉시 수행하며, squash 금지를 유지한다.

## 검증 및 실행 명령

- 프론트엔드 빌드: `npm --prefix frontend run build`
- 프론트엔드 테스트·커버리지: `npm --prefix frontend run test:coverage`
- Comprehensive Rust 변환: `python3 scripts/import-comprehensive-rust.py <snapshot-dir> <output-dir>` ([준비할 입력과 라이선스](frontend/src/samples/comprehensive-rust/README.md))
- Rust Reference·Cargo Book 변환: `python3 scripts/import-rust-official-components.py --rust-docs-root <toolchain>/share/doc/rust/html --license-root <toolchain>/share/doc/rust/licenses --output-root frontend/src/samples --version '<rustc version>'` ([출처와 라이선스](frontend/src/samples/rust-reference/README.md))
- `rustdoc`(`std` + 사용 크레이트)·`rustc --explain` 변환: `python3 scripts/import-rustdoc-and-errors.py --output-root frontend/src/samples`
- `kordoc` 소스 코퍼스 반입: `python3 scripts/import-kordoc-source.py --source <kordoc-dir> --output frontend/src/samples/kordoc`
- Rust 서버 실행: `cargo run --manifest-path backend/rust/Cargo.toml -p dok3-server`
- CI: `.github/workflows/ci.yml` 가 PR 의 `rust-fmt`, `rust-clippy`, `rust-test`, `frontend-build` 를 필수 검사로 실행한다.
- Rust 정적/테스트 검증:
  - `cargo fmt --manifest-path backend/rust/Cargo.toml --all -- --check`
  - `cargo clippy --manifest-path backend/rust/Cargo.toml --workspace --all-targets -- -D warnings`
  - `cargo test --manifest-path backend/rust/Cargo.toml --workspace`
- `AGENTS.yaml` 검증: `scripts/check-agents-yaml.sh`
- doksam 배포·검증:
  - 이미지 빌드: `deploy/doksam/scripts/build-image.sh`
  - 컨테이너 smoke: `deploy/doksam/scripts/smoke-container.sh`
  - 배포: `deploy/doksam/scripts/deploy.sh`
  - 롤백: `deploy/doksam/scripts/rollback.sh`
  - 검증(status code): `deploy/doksam/scripts/verify.sh`
