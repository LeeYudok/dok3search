# 폐쇄망 오프라인 의존성 번들

이 문서는 인터넷이 연결된 빌드 호스트에서 dok3search의 Rust·프론트엔드 의존성을 한 번 모아 폐쇄망으로 옮기는 절차를 설명한다. 번들에는 소스 코드가 아니라 의존성 캐시만 들어간다. 수신 측에는 같은 커밋의 dok3search 소스 체크아웃과 Rust·Node.js 도구 체인이 미리 있어야 한다.

## 번들 만들기

빌드 호스트에서 저장소 루트 기준으로 실행한다.

```sh
scripts/offline-bundle.sh
```

기본 출력 위치는 `offline-bundles/`다. 이 폴더는 `.gitignore`에 포함되어 있으므로 산출물을 커밋하지 않는다. 다른 위치에 만들려면 다음처럼 지정한다.

```sh
scripts/offline-bundle.sh --output-dir /secure/transfer/dok3search
```

스크립트는 `backend/rust/Cargo.lock`을 기준으로 `cargo vendor`를 실행하고, `frontend/package-lock.json`을 기준으로 npm 캐시를 채운다. 결과는 다음 두 파일이다.

```text
dok3search-offline-<UTC timestamp>.tar.gz
dok3search-offline-<UTC timestamp>.tar.gz.sha256
```

번들 안의 구조는 다음과 같다.

```text
dok3search-offline-<UTC timestamp>/
  bundle-info.txt
  cargo/
    config.toml
    vendor/
  npm/
    cache/
```

`bundle-info.txt`에는 생성 시각, Git 커밋, Rust·Cargo·Node.js·npm 버전, OS와 CPU 아키텍처가 기록된다. 네이티브 npm 선택 패키지와 Rust 타깃은 플랫폼에 영향을 받으므로, 번들은 수신 환경과 같은 OS/아키텍처 계열에서 만드는 것이 원칙이다.

## 무결성 확인과 해제

전송 후 SHA-256 매니페스트를 먼저 검증한다.

```sh
cd /path/to/bundle
shasum -a 256 -c dok3search-offline-*.tar.gz.sha256
```

GNU 환경에서는 다음 명령을 쓸 수 있다.

```sh
sha256sum -c dok3search-offline-*.tar.gz.sha256
```

그다음 번들을 작업 폴더에 푼다.

```sh
mkdir -p /tmp/dok3search-offline
tar -xzf dok3search-offline-*.tar.gz -C /tmp/dok3search-offline
```

이후 아래 명령에서 `BUNDLE_DIR`는 실제로 풀린 `dok3search-offline-<UTC timestamp>` 폴더를 뜻한다.

```sh
BUNDLE_DIR=/tmp/dok3search-offline/dok3search-offline-<UTC timestamp>
```

## Rust 의존성 설치

저장소 소스 체크아웃의 `backend/rust` 아래에 vendored 소스를 둔다.

```sh
rm -rf backend/rust/vendor
mkdir -p backend/rust/vendor .cargo
cp -R "$BUNDLE_DIR/cargo/vendor/." backend/rust/vendor/
cp "$BUNDLE_DIR/cargo/config.toml" .cargo/config.toml
```

저장소 루트의 `.cargo/config.toml`은 다음 내용이어야 한다.

```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "backend/rust/vendor"
```

`directory`는 저장소 루트의 `.cargo/config.toml`을 기준으로 해석된다. 이 위치를 쓰면 `--manifest-path backend/rust/Cargo.toml`처럼 저장소 루트에서 실행하는 Cargo 명령도 vendored 소스를 찾는다. `backend/rust`로 이동해 Cargo를 실행하는 습관이 있다면 같은 내용을 `backend/rust/.cargo/config.toml`에 두고 `directory = "vendor"`로 바꿔도 된다. 한 저장소에는 둘 중 한 위치만 사용한다.

네트워크 없이 잠금 파일과 vendored 소스만 사용해 빌드한다.

```sh
CARGO_TARGET_DIR=/tmp/dok3-offline-target \
  cargo build \
  --manifest-path backend/rust/Cargo.toml \
  --workspace \
  --offline \
  --frozen
```

`cargo vendor`가 출력하는 설정의 절대 경로는 사용하지 않는다. 번들에 포함된 `cargo/config.toml`은 위처럼 저장소 루트 기준 상대 경로를 사용한다. 설치 후 생기는 `.cargo/config.toml`, `backend/rust/vendor/`, `.offline-cache/`는 `.gitignore`에 포함되어 있으므로 커밋하지 않는다.

## 프론트엔드 의존성 설치

npm 캐시를 저장소 밖의 전용 폴더로 복사하고 오프라인 설치를 실행한다.

```sh
rm -rf .offline-cache/npm
mkdir -p .offline-cache
cp -R "$BUNDLE_DIR/npm/cache" .offline-cache/npm
(
  cd frontend
  npm ci \
    --cache "$PWD/../.offline-cache/npm" \
    --offline \
    --ignore-scripts \
    --no-audit \
    --no-fund
)
npm --prefix frontend run build
```

`npm ci`는 `node_modules`를 잠금 파일 기준으로 다시 만든다. `--offline`은 네트워크 접근 대신 `.offline-cache/npm`만 사용하게 한다. `--ignore-scripts`는 설치 시 임의 스크립트와 외부 다운로드를 막는다. 현재 프론트엔드 빌드는 설치 스크립트 없이 동작한다.

## 오프라인 검증 체크리스트

폐쇄망 반입 전에 네트워크가 없는 임시 환경에서 다음 두 명령을 확인한다.

```sh
cargo build \
  --manifest-path backend/rust/Cargo.toml \
  --workspace \
  --offline \
  --frozen
npm --prefix frontend run build
```

두 번째 명령 전에는 반드시 위의 오프라인 `npm ci`를 먼저 실행한다. Rust와 npm 명령 모두 성공해야 번들이 완전하다. 실패하면 `bundle-info.txt`의 플랫폼·도구 버전과 수신 환경을 비교하고, 같은 플랫폼에서 번들을 다시 만든다.

## 제약

- Rust 도구 체인(`rustc`, `cargo`, `rustfmt`, `clippy`)과 Node.js·npm 자체는 번들에 포함하지 않는다. 폐쇄망에 별도로 준비해야 한다.
- npm 선택 의존성과 Rust 타깃별 패키지는 생성 플랫폼에 맞춰진다. 다른 OS/CPU에서 쓸 번들은 해당 환경에서 생성한다.
- Git 저장소를 직접 가리키는 Cargo·npm 의존성은 이 절차의 검증 범위가 아니다. 잠금 파일이 레지스트리 아티팩트만 참조하는 현재 구조를 전제로 한다.
