# dok3search

폐쇄망 문서·코드 검색 플랫폼의 모노레포입니다. React 프론트엔드와 Rust 백엔드(`dok3-core`·`dok3-domain`·`dok3-search`·`dok3-api`·`dok3-server`)가 인메모리 수직 슬라이스(`P0-2`~`P0-3`) 기준으로 연결되어 있습니다.

## 디렉터리 구조

```text
dok3search/
├── frontend/                  # React + TypeScript + Vite 문서 검색 웹 UI
├── backend/
│   └── rust/                  # Rust Cargo workspace (13개 크레이트)
├── config/                    # 실행 설정 파일 배치 디렉터리
├── contracts/                 # API·도메인 계약 스키마 배치 디렉터리
├── docs/                      # 운영·아키텍처 문서 배치 디렉터리
├── tests/                     # 통합·장애 주입·성능 테스트 배치 디렉터리
├── scripts/                   # 저장소 검증·빌드 보조 스크립트
├── CHANGELOG.md               # 주요 변경 이력
├── DESIGN.md                  # UI 설계 개요
└── LICENSE                    # MIT 라이선스
```

## 로컬 실행 (단일 바이너리 또는 개발 프록시)

1. 프론트엔드 정적 자산을 빌드합니다.

```sh
npm --prefix frontend ci
npm --prefix frontend run build
```

2. Rust 백엔드 서버를 실행합니다 (`http://127.0.0.1:8080`, `frontend/dist` 자동 서빙).

```sh
cargo run --manifest-path backend/rust/Cargo.toml -p dok3-server
```

프론트엔드 개발 서버(`npm --prefix frontend run dev`)를 별도로 띄우면 `/api`·`/health` 요청이 백엔드(`DOK3_API_URL`, 기본 `http://127.0.0.1:8080`)로 프록시되며, 백엔드가 꺼져 있을 때는 내장 샘플 데이터로 폴백합니다.

### 제공 중인 REST 엔드포인트

- `GET /health/ready` — 서비스 준비 상태, 색인 문서 수, 스냅샷 메타데이터
- `POST /api/v1/search` — ACL 기반 문서 검색, 카테고리별 건수, 관련 검색어, 페이징
- `GET /api/v1/documents/{id}` — 문서 상세, 섹션 목차, Markdown/YAML 형식 연결 정보
- `GET /api/v1/documents/{id}/sections/{sid}` — 문자 커서 기반 섹션 본문 페이징

### 서버 환경 변수

- `DOK3_BIND` (기본 `127.0.0.1`), `DOK3_PORT` (기본 `8080`)
- `DOK3_STATIC_DIR` (정적 `frontend/dist` 경로), `DOK3_DESIGN_DIR` (`samples/design` 경로)

## 폐쇄망 개발 참고 자료

`/search?category=rust`는 Google Comprehensive Rust 한국어판 337개 장과 Rust 공식 책 영문판 111개 장을 검색합니다. `/search?category=go`는 Go 언어 명세, Go Modules Reference, Effective Go의 영문 항목 51개를 검색합니다. 제목·본문·코드를 검색할 수 있고, 참고 자료 본문과 그림은 정적 빌드에 포함됩니다.

개발 에이전트는 `frontend/src/samples/{comprehensive-rust,rust-book,go-reference}/chapters/`의 Markdown을 직접 읽거나 파일 색인에 넣을 수 있습니다. [Comprehensive Rust 출처와 라이선스](frontend/src/samples/comprehensive-rust/README.md) · [Rust 공식 책 출처와 라이선스](frontend/src/samples/rust-book/README.md) · [Go 문서 출처와 라이선스](frontend/src/samples/go-reference/README.md)

## Rust 백엔드 검증

```sh
cd backend/rust
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

설계 원본은 [설계 저장소 (`dok3search-design`)](https://github.com/LeeYudok/dok3search-design)에 있으며, 샘플 문서 라이선스는 [샘플 라이선스](frontend/src/samples/design/LICENSE)를 함께 둡니다.

## 라이선스 및 변경 이력

- 라이선스: [MIT License](LICENSE)
- 변경 이력: [CHANGELOG.md](CHANGELOG.md)
