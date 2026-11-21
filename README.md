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
└── DESIGN.md                  # UI 설계 개요
```

## 프론트엔드 실행

```sh
npm --prefix frontend ci
npm --prefix frontend run dev
```

Vite가 출력하는 로컬 주소를 엽니다. `npm --prefix frontend run build`로 TypeScript 검사와 정적 빌드를 실행합니다.

## Rust 백엔드 점검

```sh
cd backend/rust
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

설계 문서 샘플은 [설계 저장소](https://github.com/LeeYudok/dok3search-design)에서 가져왔습니다. [샘플 라이선스](frontend/src/samples/design/LICENSE)를 함께 둡니다.
