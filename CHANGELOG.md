# 변경 이력

이 파일은 `dok3search` 저장소의 주요 변경 사항을 기록합니다.

## [Unreleased]

### 추가
- `P0-3` 인메모리 검색 서비스(`dok3-search`), Axum REST API 및 보안 헤더·정적 SPA 호스팅(`dok3-api`·`dok3-server`), React REST API 클라이언트 연동 및 로컬 폴백 구성 (#11)
- `P0-2` `dok3-core` 오류·설정(`Dok3Error`, `ServerConfig`) 및 `dok3-domain` 도메인 계약 DTO·기본 거부 ACL·Port 트레이트·계약 테스트 추가 (#10)
- 설계 문서 목록에서 인덱스와 맵의 Markdown·YAML을 각 1행으로 묶고 상세 화면에서 형식을 전환하며 두 형식 본문을 검색하도록 개선 (#5, PR #9)
- MIT 라이선스(`LICENSE`) 및 변경 이력(`CHANGELOG.md`) 추가 (#3)
- `frontend/` + `backend/rust/` 모노레포 스캐폴딩 구성 — 설계서 v1.3 기준 Rust 13개 크레이트 최소 스텁(`dok3-core` ~ `dok3-server`), `config/`·`contracts/`·`docs/`·`tests/` 디렉터리, `AGENTS.md`·`AGENTS.yaml`·`scripts/check-agents-yaml.sh` (#1, PR #2)
- React + TypeScript + Vite 기반 문서 검색 로컬 미리보기 초기 스캐폴딩 (`5a48d6c`)
