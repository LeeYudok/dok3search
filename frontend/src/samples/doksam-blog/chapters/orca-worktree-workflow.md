# Orca 사용법 — worktree 워크플로 중심으로

AI 코딩 에이전트를 쓰다 보면 금방 부딪히는 문제가 있다. **세션 여러 개가 같은 레포를 동시에 만지면 서로 발밑을 바꾼다**는 것. 한 세션이 브랜치를 갈아타면 다른 세션의 dev 서버가 깨지고, `git add -A` 한 방에 남의 작업이 커밋에 섞인다.

[Orca](https://orca.dev)는 이 문제를 **git worktree 격리**로 푸는 에이전트 오케스트레이션 도구다. 이 글은 그중 핵심인 worktree 워크플로만 실제 사용 흐름대로 정리한다.

> [그림] Orca 메인 화면 — 왼쪽은 worktree 별 에이전트 세션 목록, 오른쪽은 선택한 세션의 터미널

위가 실제 작업 중인 Orca 화면이다. 왼쪽 사이드바에는 worktree 단위 에이전트 세션이 WORKING / DONE 으로 나뉘어 쌓인다. 오른쪽에는 선택한 세션의 터미널(에이전트 대화·diff)이 뜬다. 세션 여러 개가 서로 다른 레포·브랜치에서 동시에 돌아도 충돌이 없다. 각자 자기 worktree 폴더 안에서 일하기 때문이다. 목록 두 번째 "Check blog status" 세션이 바로 이 글을 쓰고 있는 세션이다.

## 핵심 원칙: 1 세션 = 1 worktree = 1 브랜치

git worktree 는 한 레포의 여러 브랜치를 **각각 독립된 폴더에 동시에 체크아웃**하는 git 내장 기능이다. Orca 는 이걸 관리 레이어로 감싸서, 에이전트 세션마다 자기만의 작업 폴더를 준다.

- 정식 클론은 default 브랜치(main) 미러로만 둔다. **checkout / switch 를 하지 않는다** — 읽기와 pull 전용.
- 작업은 전부 worktree 안에서. 세션마다 폴더가 다르니 브랜치 전환·파일 덮어쓰기 경쟁이 원천적으로 없다.
- git 은 같은 브랜치의 이중 체크아웃을 거부하므로, "자동 프로세스가 몰래 main 으로 checkout 해서 main 위에 커밋되는" 사고도 구조상 막힌다.

> [그림] Orca 사이드바 — worktree 단위 에이전트 세션이 WORKING / DONE 으로 정리된다

사이드바를 보면 원칙이 그대로 UI 에 드러난다. 카드 하나가 곧 worktree 하나이고, 각 카드에 레포 이름과 브랜치·마지막 작업 요약이 붙는다. 어떤 세션이 일하는 중(WORKING)이고 어떤 세션이 끝났는지(DONE)가 목록만 봐도 구분된다.

## 기본 흐름

### 1. 레포 등록

```bash
orca repo add ~/workspace/my-repo   # 로컬 클론을 Orca 에 등록
orca repo list                      # 등록된 레포 확인
```

### 2. worktree 생성

작업(이슈) 하나 시작할 때마다 worktree 를 하나 만든다.

```bash
orca worktree create   # 레포·베이스 브랜치 지정해 생성
orca worktree list     # 관리 중인 worktree 목록
orca worktree current  # 현재 디렉터리가 속한 worktree 확인
```

생성된 worktree 는 독립 폴더라서 dev 서버 포트도 세션별로 따로 잡으면 충돌이 없다.

### 3. 에이전트 세션 붙이기

worktree 안에서 터미널을 열고 에이전트(Claude Code 등)를 띄운다.

```bash
orca terminal create   # worktree 에 터미널 세션 생성
orca terminal list     # 살아 있는 터미널 목록
orca terminal read     # 터미널 출력 읽기 (다른 세션에서 진행 상황 확인)
orca terminal send     # 터미널에 입력 보내기
```

`terminal read` / `send` 가 은근 강력하다. 부모 세션이 자식 세션의 화면을 읽고 지시를 내릴 수 있어서, **에이전트가 에이전트를 감독하는 구조**가 된다.

> [그림] 에이전트 세션 터미널 — 에이전트가 이 글에 이미지를 넣는 diff 를 만들고 있다

세션을 선택하면 이렇게 에이전트의 터미널이 통째로 보인다. 위 화면은 실제로 에이전트가 이 글의 마크다운에 스크린샷을 삽입하는 diff 를 만들고, 커밋 → MR → 머지 → 배포 폴링까지 진행하는 장면이다. 사람은 이 화면을 읽으면서 방향만 잡아 주면 된다.

### 4. 작업 → 머지

worktree 안에서 하는 작업은 평소와 같다. 브랜치에 커밋하고, 푸시하고, MR/PR 을 올린다. 중요한 건 **머지는 부모(사람 또는 감독 세션)가 게이트**한다는 것. 병렬로 도는 세션이 각자 마음대로 main 에 머지하면 격리한 의미가 없다.

### 5. 정리

머지가 끝난 worktree 는 그 자리에서 바로 지운다.

```bash
orca worktree rm       # Orca 와 git 양쪽에서 worktree 제거
```

쌓아 두면 다음 세션이 stale worktree 를 열거나, 정리 대상 감사부터 해야 한다. "머지 완료 = worktree 삭제"를 습관으로.

## 병렬 오케스트레이션 한 눈에 보기

여러 worktree 를 동시에 굴릴 때는 요약 뷰가 있다.

```bash
orca worktree ps       # worktree 별 상태 압축 요약
orca orchestration send   # 세션 간 메시지 전달
orca orchestration inbox  # 수신 메시지 확인
```

> [그림] orca worktree ps 실행 화면 — live 세션만 필터링하면 레포·브랜치·터미널 상태가 한 줄씩 잡힌다

`orca worktree ps | grep -A2 'live:[1-9]'` 로 살아 있는 세션만 걸러 본 화면이다. worktree 마다 레포·브랜치·pty 상태·마지막 터미널 미리보기가 잡힌다. 그래서 지금 이 머신에서 에이전트 몇 개가 어느 브랜치를 만지고 있는지 한 번에 파악할 수 있다. 왼쪽 프로젝트 트리에도 레포별 worktree 가 계층으로 정리되어 있다.

작업을 task 로 등록해 특정 터미널에 배분하는 `orchestration task-create` / `dispatch` 도 있어서, 규모가 커지면 코디네이터 세션 하나가 나머지를 지휘하는 형태로 확장된다.

## 정리

| 규칙 | 이유 |
|---|---|
| 정식 클론에서 checkout 금지 | 다른 세션·dev 서버의 발밑을 바꾸지 않기 위해 |
| 1 세션 = 1 worktree = 1 브랜치 | 덮어쓰기·checkout 레이스·커밋 오염 차단 |
| 머지는 부모가 게이트 | 병렬 세션의 무질서한 main 머지 방지 |
| 머지 후 즉시 worktree 삭제 | stale worktree 가 다음 세션의 부채가 됨 |

도구는 Orca 지만 원칙은 범용이다. 에이전트를 하나만 써도 "정식 클론은 건드리지 않고 worktree 에서 작업"하는 습관은 그대로 유효하다. 실제로 이 글도 Orca worktree 안의 Claude Code 세션이 작성해서 MR 로 올린 것이다.
