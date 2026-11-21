# dok3search

폐쇄망 문서·코드 검색 플랫폼의 모노레포입니다. 현재는 로컬 샘플 기반 React 프론트엔드와 Rust 백엔드 워크스페이스 뼈대로 구성되어 있습니다.

## 디렉터리 구조

```text
dok3search/
├── frontend/                  # React + TypeScript + Vite 문서 검색 웹 UI
├── backend/
│   └── rust/                  # Rust Cargo workspace (13개 크레이트 골격)
├── config/                    # 실행 설정 파일 배치 디렉터리
├── contracts/                 # API·도메인 계약 스키마 배치 디렉터리
├── docs/                      # 운영·아키텍처 문서 배치 디렉터리
├── tests/                     # 통합·장애 주입·성능 테스트 배치 디렉터리
├── scripts/                   # 저장소 검증·빌드 보조 스크립트
├── CHANGELOG.md               # 주요 변경 이력
├── DESIGN.md                  # UI 설계 개요
└── LICENSE                    # MIT 라이선스
```

## 프론트엔드 실행

```sh
npm --prefix frontend ci
npm --prefix frontend run dev
```

Vite가 출력하는 로컬 주소를 엽니다. `npm --prefix frontend run build`로 TypeScript 검사와 정적 빌드를 실행합니다.

## 폐쇄망 Rust 참고 자료

`/search?category=rust`에서 Google의 Comprehensive Rust 한국어판 337개 장과 Rust 공식 책 영문판 111개 장을 검색합니다. 제목·본문·코드를 검색할 수 있고, 열람 화면의 코드 예제와 강의 자료에 포함된 그림은 로컬에서 엽니다.

개발 에이전트는 `frontend/src/samples/comprehensive-rust/chapters/`와 `frontend/src/samples/rust-book/chapters/`의 Markdown을 직접 읽거나 파일 색인에 넣을 수 있습니다. [Comprehensive Rust 출처와 라이선스](frontend/src/samples/comprehensive-rust/README.md) · [Rust 공식 책 출처와 라이선스](frontend/src/samples/rust-book/README.md)

## Rust 백엔드 점검

```sh
cd backend/rust
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

설계 문서 샘플은 [설계 저장소](https://github.com/LeeYudok/dok3search-design)에서 가져왔습니다. [샘플 라이선스](frontend/src/samples/design/LICENSE)를 함께 둡니다.

## 라이선스 및 변경 이력

- 라이선스: [MIT License](LICENSE)
- 변경 이력: [CHANGELOG.md](CHANGELOG.md)
