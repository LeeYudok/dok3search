# SonarQube 는 이슈를 고쳐도 리팩토링은 하지 않는다 — 근거와 추측 구분하기

> 정적분석 도구를 들이면 레거시 리팩토링이 될까?

답부터 적는다. 되지 않는다. SonarQube 는 문제를 찾아 점수를 매기고 머지를 막는다. 지금은 유료 판에 수정을 제안하는 AI CodeFix(2025-07 정식 출시)와 PR 을 여는 Remediation Agent(2026.5)가 붙었다. 그래도 고치는 단위는 규칙 위반 한 건이다. 무엇을 고치지 않을지, 고친 뒤에도 동작이 같은지, 구조를 어떻게 나눌지는 여전히 사람이 정한다.
2026-08-25 에 내 서비스 저장소의 Sonar 이슈 322건을 에이전트로 정리하면서 그 경계를 직접 봤다. 공식 문서는 2026-11-18 에 다시 확인했다.

> [그림] 검사 로봇이 블록으로 쌓은 탑에 빨간 깃발을 잔뜩 꽂고 클립보드에 322 라고 적는다. 수리 로봇은 깃발 하나가 꽂힌 블록 하나를 렌치로 고친다. 탑 옆에서는 설계도를 든 로봇이 어느 층부터 손볼지 손가락으로 가리킨다

그림으로 줄이면 이렇다. 검사 로봇은 깃발을 꽂고, 수리 로봇은 깃발 하나씩 고친다. 어느 층부터 손볼지는 설계도를 든 쪽이 정한다.

## SonarQube 는 유료 판에 고치는 손을 달았다

옛 글은 "SonarQube 는 실제 코드를 고치지 않는다"고 적었다. 무료 판에서는 지금도 맞다. Community Build 문서는 "Starting with the Enterprise edition, you can get AI-suggested fixes for your code issues." 라고 적는다. 유료 판에서는 사정이 다르다.

| 기능 | 쓸 수 있는 판 | 하는 일 | 하지 않는 일 |
|---|---|---|---|
| 정적분석과 Quality Gate | 모든 판 | 스멜·중복·복잡도·보안 문제를 찾고 새 코드 기준으로 머지를 막는다 | 코드 수정 |
| AI CodeFix | Server Enterprise·Data Center, Cloud Team·Enterprise | 정해진 규칙의 위반 한 건에 LLM 수정안을 보여 준다 | 자동 적용. IDE 에서 사람이 적용하거나 거절한다 |
| Remediation Agent | Server 2026.5 이상 Enterprise·Data Center, 별도 구독 | 백로그 이슈를 고쳐 PR 을 연다 | 머지. 문서는 "the agent never merges anything" 이라고 적는다 |
| IDE 퀵픽스 | SonarQube for IDE | 일부 규칙을 규칙 기반으로 한 번에 고친다 | 인지 복잡도 규칙(S3776)은 퀵픽스가 없다 |

AI CodeFix 의 수정안은 Java·JavaScript·TypeScript·Python·HTML·CSS·C#·C++ 의 일부 규칙에만 나온다. LLM 은 Sonar 가 고른 OpenAI 모델, 자체 Azure OpenAI·AWS Bedrock, 자체 호스팅 게이트웨이 가운데 고른다. Remediation Agent 가 다루는 정적분석 이슈는 C#·Java·JavaScript/TypeScript·Python 뿐이다.

옛 글이 SonarQube 의 할 일로 적은 "보안 핫스팟 탐지"도 낡았다. 2026.5 문서는 "Security Hotspots are being phased out." 이라고 적는다. 핫스팟을 내던 규칙은 이제 취약점이나 보안 이슈를 낸다. 기본 Quality Gate(Sonar way)의 새 코드 조건은 넷이다. 새 이슈 0건, 새 핫스팟 전부 검토, 커버리지 80% 이상, 중복 3% 이하.

## 내 저장소의 322건은 에이전트가 고치고 사람이 판정했다

대상은 Next.js·Spring·Python 으로 된 내 서비스 저장소다. 코드는 31,413줄(ncloc)이다. 집 서버의 SonarQube Community Build 로 재스캔했더니 Quality Gate 가 실패했다. 열린 이슈는 322건이었다.

| 지표 | 처음 | 끝 |
|---|---:|---:|
| 버그 | 6 | 0 |
| 취약점 | 6 | 0 |
| 검토 대기 핫스팟 | 5 | 0 |
| 코드 스멜 | 305 | 0 |
| Quality Gate | 실패 | 실패 (새 코드 커버리지 78.8%) |

Community Build 라 AI CodeFix 는 없었다. 범용 코딩 에이전트를 썼다. 웹·서버·데이터 영역마다 서브에이전트 하나가 자기 worktree 에서 일했다. 규칙은 둘이었다. 동작을 바꾸지 않는다. 규칙 위반 1건에 수정 1건이다. 커밋은 영역과 규칙 묶음 단위로 44개를 넘었다.

고치는 동안 이슈가 새로 생겼다. 1차 재스캔에서 35건, 3차에서 1건이다. 줄이 옮겨지거나 고친 코드가 다른 규칙에 걸린 것이다. 그래서 처리한 건수는 322 + 35 + 1 = 358건이다. 그중 289건은 고쳤고 69건은 Won't Fix 나 SAFE 로 닫았다.

69건이 도구가 못 하는 판단이다. 예를 들면 이렇다.

- Quartz 잡의 생성자 주입 권고 15건은 따를 수 없다. 프레임워크가 리플렉션으로 잡을 만든다.
- 같은 클래스 안의 호출을 프록시로 바꾸라는 권고 3건은 미뤘다. 없던 트랜잭션 경계가 켜져 동작이 바뀐다.
- null 을 돌려주지 말라는 권고 6건은 따르지 않았다. 그 null 이 장애 폴백을 뜻하고, 테스트가 그 값을 고정한다.

교차 리뷰도 사람이 설계했다. Claude 의 코드 리뷰 에이전트 둘과 Codex 가 따로 diff 를 읽었다. 리뷰는 동작이 바뀐 곳 2건을 잡았다. 실패한 배치가 `continue` 를 잃어 로그와 대기를 타게 된 곳, 화면 목록의 복합 key 가 겹칠 수 있게 된 곳이다. 정규식 `.+?` 를 `.{1,200}?` 로 바꾼 것은 동작 변경이지만 ReDoS 방어로 받아들였다.

마지막 줄도 볼 만하다. 이슈를 0건으로 만들었는데 Quality Gate 는 그대로 실패였다. 새 코드 커버리지 78.8% 가 기준 80% 에 못 미쳤기 때문이다. 이슈 수와 게이트 통과는 다른 조건이다.

해석은 이렇다. 도구가 정한 것은 "무엇이 규칙 위반인가"까지였다. 고치지 않을 69건(358건의 19%)을 고르는 일, 고친 뒤 동작이 같은지 확인하는 일은 사람과 리뷰가 했다. 유료 판의 Remediation Agent 도 같은 구조다. PR 을 열면 사람이 리뷰하고 머지한다. 나는 유료 판을 써 보지 않았으므로, 그 에이전트가 이 69건을 어떻게 다루는지는 확인하지 못했다.

## 출처를 확인한 방법론은 여섯 가지다

아래는 책·논문·공식 페이지로 출처를 확인한 것이다. 링크는 2026-11-18 에 모두 열렸다.

| 방법 | 출처 | 핵심 |
|---|---|---|
| 특성화 테스트 | Michael Feathers, 『Working Effectively with Legacy Code』, Prentice Hall, 2004 | 고치기 전에 현재 동작을 테스트로 고정한다 |
| Strangler Fig | Martin Fowler, [Strangler Fig](https://martinfowler.com/bliki/StranglerFigApplication.html) (2004 원문, 2024-08-22 개정) | 새 구현으로 조금씩 옮긴 뒤 옛 구현을 걷어 낸다 |
| 모노레포와 Rosie | Potvin·Levenberg, [Why Google Stores Billions of Lines of Code in a Single Repository](https://research.google/pubs/why-google-stores-billions-of-lines-of-code-in-a-single-repository/), CACM, 2016 | 큰 패치를 작게 나눠 따로 테스트하고 리뷰에 올린다 |
| Tricorder | Sadowski 외, [Tricorder: Building a Program Analysis Ecosystem](https://research.google/pubs/tricorder-building-a-program-analysis-ecosystem/), ICSE, 2015 | 정적분석 결과를 코드 리뷰 화면에 붙인다 |
| ClangMR | Wright 외, [Large-Scale Automated Refactoring Using ClangMR](https://research.google/pubs/large-scale-automated-refactoring-using-clangmr/), ICSM, 2013 | API 가 바뀌면 호출부를 도구로 한꺼번에 바꾼다 |
| LLM 마이그레이션 | Ziftci 외, [Migrating Code At Scale With LLMs At Google](https://arxiv.org/abs/2504.09691), arXiv, 2025 | 대규모 코드 이전의 변경 대부분을 LLM 이 만들고 사람이 검토한다 |

마지막 줄은 새로 더했다. 개발자 셋이 12개월 동안 마이그레이션 39건을 했다. 제출한 코드 변경 595건 가운데 74.45% 를 LLM 이 만들었다. 개발자들은 예전의 손 작업보다 전체 시간이 약 50% 줄었다고 추정했다. 구글도 고치는 손은 LLM 에 넘겼다. 무엇을 옮길지 정하고 결과를 검토하는 일은 사람에게 남았다.

## 회사 이름을 붙인 일반론은 추측이다

국내 대기업의 내부 리팩토링 절차는 대부분 공개돼 있지 않다. 그래서 "네카라쿠배도 이렇게 한다"는 말은 업계 일반론에 회사 이름을 붙인 추측이다. "레거시 자바를 코틀린으로 옮기는 쪽을 선호한다" 같은 세부도 공개 사례를 찾아 확인하기 전에는 추측이다.

이 구분이 필요한 이유는 AI 에게 물으면 두 종류가 같은 말투로 섞여 나오기 때문이다. 출처를 요구하면 걸러진다. 이번에 다시 확인하면서도 같은 일을 겪었다. 옛 글이 SonarQube 의 기능으로 적은 핫스팟은 폐지되는 중이었다. 출처가 있던 주장도 시간이 지나면 낡는다.

## 규모와 무관하게 순서는 같다

자동화 수준은 구글과 대다수 조직이 크게 다르다. 순서는 같다. 322건 사례를 옆에 붙이면 이렇다.

| 단계 | 하는 일 | 322건 사례 |
|---|---|---|
| 식별 | 정적분석으로 위반·복잡도·중복에 점수를 매긴다 | 규칙 ID 별 대상 목록을 이슈 본문에 자동으로 만들었다 |
| 안전망 | 현재 동작을 테스트로 고정한다 | 웹 테스트 390개, 서버 빌드, 데이터 테스트 40개가 있었다 |
| 점진적 변경 | 작은 단위로 고친다 | 영역 × 규칙 묶음 단위 커밋 |
| 재검증 | 테스트와 게이트를 다시 통과시킨다 | 재스캔 네 번, 교차 리뷰 세 갈래 |

안전망이 없는 코드라면 둘째 단계부터 막힌다. 고친 뒤 동작이 바뀌었는지 알 길이 없기 때문이다. 이것은 사람이 고치든 AI CodeFix 가 고치든 같다.

## 이 판단이 틀리는 경우

- 규칙 위반을 정리하는 일이라면 유료 판의 Remediation Agent 가 사람 손을 크게 줄일 수 있다. 수정 뒤 새 이슈가 생기지 않는지 재분석까지 한다고 문서가 적는다. 이 글의 경계는 구조를 바꾸는 리팩토링에 관한 것이다.
- 구글의 74.45% 는 구글의 코드베이스와 사내 도구에서 나온 값이다. 일반 조직에 그대로 옮기면 안 된다.
- Won't Fix 비율 19% 는 내 저장소 한 곳의 값이다. 프레임워크 제약이 적은 코드라면 훨씬 낮을 수 있다.

## 정리

정적분석 도구를 들이는 것과 리팩토링이 되는 것은 다른 일이다. 지금의 SonarQube 는 유료 판에 고치는 손을 달았지만, 그 손이 다루는 단위는 규칙 위반 한 건이다. 고치지 않을 것을 고르고, 동작이 같은지 확인하고, 구조를 나누는 일은 사람이 한다.

다음에 Quality Gate 가 실패하면 먼저 하나를 본다. 실패 조건이 이슈 수인지, 커버리지 같은 다른 조건인지다. 이슈만 0으로 만들어서는 통과하지 않는 게이트가 있다.

이 글은 2026-08-26 에 옛 블로그에 올린 글을 2026-11-18 에 다시 확인하고 고쳐 쓴 것이다.
