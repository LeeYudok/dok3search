# dok3search → doksam k3s 배포

dok3search를 단일 노드 doksam Kubernetes에 배포하고 `https://www.doksam.com`
Host로 서비스한다. 공개 진입점은 Traefik basicAuth로 전면 보호하므로 인증 없는
direct request는 401이다.

## 토폴로지

```
Cloudflare / DNS
  → pig nginx
  → doksam:443 (Traefik, hostPort 80/443, entrypoint websecure)
  → Ingress dok3search (host www.doksam.com, basicAuth middleware)
  → Service dok3search (ClusterIP :80)
  → Pod dok3search (:8080, DOK3_BIND=0.0.0.0)
```

- 컨테이너는 `0.0.0.0:8080`에 바인드하고 Service는 ClusterIP만 노출한다.
  jex3 등이 쓰는 host port(`192.168.113.220:8081` 계열)는 재사용하지 않는다.
- TLS 인증서는 기존 cert-manager ClusterIssuer `letsencrypt-dns01`(DNS-01)로
  발급한다. ACME HTTP 경로가 공개될 필요가 없어 모든 경로를 인증 뒤에 둘 수 있다.
- `www.doksam.com`은 현재 pig nginx가 서비스하는 Host다. doksam으로 라우팅을
  옮길 때는 pig/Cloudflare 설정 변경이 함께 필요하다(이 문서는 클러스터 측만
  다룬다).

## 파일

- `k3s/dok3search.yaml` — Namespace, PVC, Deployment, Service, Traefik
  Middleware(basicAuth·https-redirect), Ingress(websecure/web). Secret은
  정의하지 않는다.
- `scripts/build-image.sh` — linux/amd64 이미지 빌드.
- `scripts/create-basic-auth-secret.sh` — basicAuth Secret·0600 파일 생성.
- `scripts/smoke-container.sh` — 이미지 안에서 SPA·health endpoint 확인.
- `scripts/deploy.sh` — 빌드·import·apply·rollout·verify (idempotent).
- `scripts/rollback.sh` — 직전 이미지로 복원 (idempotent, 데이터 불변).
- `scripts/verify.sh` — status code만 확인.

## 기본 인증 (basicAuth)

Ingress 전체가 Traefik `Middleware/basicAuth`로 보호된다. 자격증명 값은 git에
넣지 않는다. 경로와 키 이름만 기록한다.

| 항목 | 위치 |
| --- | --- |
| 평문 비밀번호 (0600) | doksam 호스트 `/etc/dok3search/basic-auth.pass` |
| htpasswd (0600) | doksam 호스트 `/etc/dok3search/basic-auth.users` |
| Kubernetes Secret | `dok3search` namespace, 이름 `dok3search-basic-auth`, 키 `users` |

대화형 생성/갱신:

```sh
BASIC_AUTH_USER=dok3search deploy/doksam/scripts/create-basic-auth-secret.sh
```

- 프롬프트에서 basicAuth password, 확인 값, doksam sudo password를 입력한다.
- 비대화형 실행은 mode-0600 stdin 파일을 사용한다. 첫 줄은 basicAuth password,
  둘째 줄은 doksam sudo password다.

```sh
install -m 600 /dev/null /tmp/dok3search-basic-auth.stdin
"${EDITOR:-vi}" /tmp/dok3search-basic-auth.stdin
deploy/doksam/scripts/create-basic-auth-secret.sh < /tmp/dok3search-basic-auth.stdin
rm -f /tmp/dok3search-basic-auth.stdin
```

- 비밀번호 값은 명령행 인자나 환경 변수 예시로 직접 쓰지 않는다. htpasswd 해시는
  doksam에서 계산하며, `htpasswd`가 없는 환경을 기준으로
  `openssl passwd -apr1 -stdin`만 사용한다.
- 비대화형 stdin은 첫 줄을 basicAuth 비밀번호, 둘째 줄을 `SUDO_PASS`로 읽는다.
  둘째 줄이 없으면 doksam 런타임 환경의 `SUDO_PASS`를 사용한다.
- `SUDO_PASS`는 doksam 런타임의 환경 변수나 stdin으로 읽고 `sudo -S`에만
  파이프한다. `sudo` 명령 argv에는 비밀번호를 넣지 않는다.
- 스크립트는 평문·해시를 화면이나 로그로 출력하지 않는다.
- `HOST=local`이면 doksam에 이미 붙어 있는 셸에서 직접 생성한다(기본은 ssh 별칭
  `doksam` 경유).

## 이미지 빌드

```sh
deploy/doksam/scripts/build-image.sh
# 또는
CONTAINER_ENGINE=docker deploy/doksam/scripts/build-image.sh
```

- 태그는 커밋 SHA 기반으로 고정한다: `dok3search:sha-<git-sha>`.
- `podman build --platform linux/amd64`(또는 docker)로 빌드하며, 멀티스테이지
  `Dockerfile`이 프론트엔드 SPA·Rust 서버·knowledge corpus를 한 이미지에 담는다.
- 원격 podman 서비스를 쓰면 엔진 인자를 한 변수에 넣는다:
  `CONTAINER_ENGINE='podman --connection podman-machine-default'`.
- `npm ci` + `cargo build --locked`로 의존성을 고정한다. 더 엄격한 재현성이
  필요하면 base 이미지 digest(`image@sha256:...`)를 release 자동화에서 핀한다.
- 레지스트리로 push하지 않는다. 이미지는 노드에 import한다.

## 컨테이너 smoke

```sh
deploy/doksam/scripts/smoke-container.sh
SKIP_BUILD=1 deploy/doksam/scripts/smoke-container.sh
```

- 이미지를 `linux/amd64`로 빌드한 뒤 컨테이너를 띄우고 컨테이너 안에서
  `/health/ready`, `/health`, `/`를 확인한다.
- host port를 열지 않고 `podman exec`로 확인하므로 원격 podman connection에서도
  같은 경로로 검증할 수 있다.

## 배포

```sh
deploy/doksam/scripts/deploy.sh
```

1. namespace를 보장하고 basicAuth Secret과 키 존재를 확인한다(없으면 실패).
2. `build-image.sh`로 `dok3search:sha-<git-sha>`를 빌드한다.
3. `podman save` → doksam `/tmp`로 복사 → `SUDO_PASS`를 stdin으로만 넘긴
   `sudo -S ctr -n k8s.io images import`.
4. 매니페스트의 `image: dok3search:doksam` 자리표시자를 실제 태그로 치환해
   `kubectl apply`로 적용한다.
5. 직전 실행 이미지를 `dok3search.doksam/previous-image` annotation에 기록한다
   (롤백 앵커).
6. `kubectl rollout status` 후 `verify.sh`를 실행한다.

`imagePullPolicy: IfNotPresent`를 쓴다. 태그가 커밋 SHA로 불변이므로 import된
레이어를 재사용하고 레지스트리에 접근하지 않는다.

### 영속 데이터

- `DOK3_DATA_DIR=/data`는 PVC `dok3search-data`(StorageClass `local-path`)에
  마운트한다. SQLite journal과 Tantivy blue-green 세대가 여기에 산다.
- `DOK3_SOURCE_DIR=/app/knowledge`는 이미지에 baked된 knowledge corpus다.
- 컨테이너는 `0.0.0.0`으로 바인드하되, `DOK3_ALLOW_NON_LOOPBACK_BIND=true`로
  loopback 제한을 명시적으로 해제한다. 이 플래그가 없으면 `DOK3_SOURCE_DIR`
  모드의 비-loopback 바인드는 거부된다. 관리자 API bind는 이 플래그와 무관하게
  항상 loopback만 허용한다.

## 롤백

```sh
deploy/doksam/scripts/rollback.sh
```

- 범위: 직전 deploy가 기록한 `previous-image` annotation 값으로 Deployment
  컨테이너 이미지만 되돌린다.
- Namespace·Service·Ingress·Middleware·Secret·PVC는 건드리지 않으므로 운영
  데이터는 파괴되지 않는다.
- `previous-image`를 변경하지 않으므로 두 번 실행해도 같은 결과다(idempotent).
- 롤백 리허설: 새 커밋을 배포한 뒤 `rollback.sh`로 직전 이미지 복원 →
  `rollout status`·`verify.sh` 통과 확인 → 다시 `deploy.sh`로 전진 배포.

### 리허설 절차

```sh
deploy/doksam/scripts/deploy.sh              # 새 이미지 배포 (이전 이미지 기록됨)
deploy/doksam/scripts/rollback.sh            # 직전 이미지로 복원
deploy/doksam/scripts/deploy.sh              # 다시 전진 배포 (idempotent)
```

## 검증

```sh
deploy/doksam/scripts/verify.sh
```

- `Host: www.doksam.com`으로 doksam:443에 인증 없이 요청 → 401.
- 인증 없이 `/`를 요청해도 Ingress 전체가 basicAuth로 막히는지 401로 확인한다.
- 인증 후 `/health/ready`(readiness), `/health`(liveness), `/`(정적 React), `POST
  /api/v1/search`(검색 API) → 200.
- 검증에는 자격증명이 있어야 한다. 없으면 실패하며, 인증 요청은 mode-0600
  임시 `curl --config` 파일을 써서 비밀번호를 curl argv에 넣지 않는다.
- status code만 기록하고 자격증명 값은 출력하지 않는다.
- 기본 검증은 시스템 trust store로 `www.doksam.com` TLS 인증서를 검증한다.
  인증서 발급 전 부트스트랩 디버깅에만 `TLS_INSECURE=1`로 일시 우회한다.

## 리소스 확인

```sh
ssh doksam 'kubectl -n dok3search get deploy,svc,ingress,pod -o wide'
ssh doksam 'kubectl -n dok3search get certificate'
```
