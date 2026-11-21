**기계 판독용 정보는 [`AGENTS.yaml`](AGENTS.yaml)** — 구성요소 그래프(`nodes`/`edges`), `commands`, `policies`, `known_issues` 등. 작업 전 이 파일을 먼저 읽고, 구성요소·경로·명령·환경변수가 바뀌면 이 문서와 **함께** 갱신한다. 두 파일이 어긋나면 실제 코드·파일시스템을 확인해 둘 다 고친다. 고친 뒤엔 `scripts/check-agents-yaml.sh` 로 검증한다. 날짜·버전 값은 항상 따옴표로 감싼다.

# dok3search 에이전트 지침

폐쇄망 환경에서 동작하는 문서·코드 검색 플랫폼(`dok3search`) 모노레포다. 다중 백엔드 확장을 대비해 언어별 백엔드는 `backend/` 아래에 두고, 현재 백엔드 워크스페이스는 `backend/rust/`에 위치한다.

## 짝 설계 저장소 (`dok3search-design`)

아키텍처·데이터 계약·운영 프로토콜의 원본 설계 SSOT는 별도 저장소 `dok3search-design`과 한 세트로 관리한다.

- 로컬 경로: `/Users/dok123/workspace/github.com/LeeYudok/dok3search-design`
- 원격 저장소: `https://github.com/LeeYudok/dok3search-design`
- 진입점: `index.md`, `index.yaml`, `map.md`, `map.yaml`, `RUST_WORKSPACE_AND_CONTRACTS.md`, `dok3search-implementation-roadmap.md`

화면·API 계약·검색 동작을 변경할 때는 `dok3search-design`의 계약 예시(`contracts/*.example.yaml`) 및 `frontend/src/samples/design/`과의 정합성을 함께 확인한다.

설계 사본의 원본 커밋과 파일별 SHA-256은 `frontend/src/samples/design/provenance.json`에 기록한다. 사본을 갱신한 뒤 `python3 scripts/check-design-copy.py --source ../dok3search-design`로 원본과 대조한다. 사본만 있는 환경에서는 `--source`를 빼고 무결성을 검사한다.

## 디렉터리 구조

- `frontend/` — React + TypeScript + Vite 웹 UI (로컬 폰트·샘플 데이터 포함, 외부 CDN 미사용, 개발 프록시 `/api`·`/health`). Markdown의 외부 URL은 클릭 가능한 링크로 렌더하지 않고 폐쇄망에서 열 수 없는 출처로 표시한다.
- `frontend/src/samples/{comprehensive-rust,rust-book,rust-reference,cargo-book,go-reference}/chapters/` — Rust와 Go 참고 자료 검색에 쓰는 장별 Markdown. 메인 `개발 가이드` 분류 안에서 Rust·Go 세부 분류로 나눈다. Comprehensive Rust와 `go-reference/chapters/practice/`의 Go 실무 가이드는 한국어이고 나머지는 영문이다. `go-reference/chapters/learn-go-with-tests/`에는 MIT 라이선스의 영문 실습 3편을 담는다.
- `backend/rust/` — Cargo workspace (`crates/` 아래 13개 크레이트: `dok3-core` ~ `dok3-server`)
- `config/` — 환경별 설정 예시 및 스키마 배치 위치
- `contracts/` — REST·MCP·도메인 계약 정의 배치 위치
- `docs/` — 아키텍처·ADR·운영 매뉴얼 배치 위치
- `tests/` — 워크스페이스 단위 외 통합·장애 주입·성능 테스트 배치 위치
- `scripts/` — 저장소 검증·유틸리티 스크립트 (`check-agents-yaml.sh`)

## 의존성 방향 규칙

- `dok3-domain`은 외부 크레이트 구현(Tantivy, Axum, SQLite 드라이버 등)에 의존하지 않는다.
- `dok3-parser`는 `dok3-index`를 직접 의존하거나 수정하지 않는다.
- `dok3-index-manager`는 `dok3-storage`와 `dok3-index`를 조합해 세대 빌드·전환·복구를 담당한다.
- `dok3-api`와 `dok3-agent`는 `dok3-search` 응용 서비스를 호출하며 도메인 로직을 중복 구현하지 않는다.

## 실행 환경 변수 (`dok3-server`)

- `DOK3_BIND`: HTTP 바인드 주소 (기본 `127.0.0.1`)
- `DOK3_PORT`: HTTP 포트 (기본 `8080`)
- `DOK3_STATIC_DIR`: 정적 React 빌드(`frontend/dist`) 경로 (미지정 시 자동 탐색)
- `DOK3_DESIGN_DIR`: 설계 문서 시드(`frontend/src/samples/design`) 경로 (미지정 시 자동 탐색)

- `DOK3_SOURCE_DIR`: 시작할 때 색인할 Markdown 폴더. 설정하면 IPv4 루프백에서만 열리고, 매번 현재 파일로 메모리 검색 스냅샷을 새로 구성한다. 읽을 수 없거나 비어 있거나 UTF-8이 아닌 파일은 경로와 사유를 기록하고 나머지 문서 색인은 계속한다.

## 검증 및 실행 명령

- 프론트엔드 빌드: `npm --prefix frontend run build`
- Comprehensive Rust 변환: `python3 scripts/import-comprehensive-rust.py <snapshot-dir> <output-dir>` ([준비할 입력과 라이선스](frontend/src/samples/comprehensive-rust/README.md))
- Rust Reference·Cargo Book 변환: `python3 scripts/import-rust-official-components.py --rust-docs-root <toolchain>/share/doc/rust/html --license-root <toolchain>/share/doc/rust/licenses --output-root frontend/src/samples --version '<rustc version>'` ([출처와 라이선스](frontend/src/samples/rust-reference/README.md))
- Rust 서버 실행: `cargo run --manifest-path backend/rust/Cargo.toml -p dok3-server`
- Rust 정적/테스트 검증:
  - `cargo fmt --manifest-path backend/rust/Cargo.toml --all -- --check`
  - `cargo clippy --manifest-path backend/rust/Cargo.toml --workspace --all-targets -- -D warnings`
  - `cargo test --manifest-path backend/rust/Cargo.toml --workspace`
- `AGENTS.yaml` 검증: `scripts/check-agents-yaml.sh`
