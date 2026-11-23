# Rust 강의 21편 — 성능 재기: criterion 과 프로파일러

> 코드가 빨라졌는지, 어디가 느린지 어떻게 확인할까?

답부터 적습니다. 두 도구를 나눠 씁니다. "A 와 B 중 무엇이 빠른가"는 벤치마크(criterion)로 반복 측정해 신뢰 구간으로 봅니다. "프로그램이 어디서 시간을 쓰나"는 프로파일러로 호출 스택을 떠 봅니다. 둘 다 입력을 `black_box` 로 가리지 않으면 컴파일러가 일을 지워 버립니다. 실측에서 가리지 않은 합계는 0.22나노초, 가린 합계는 566나노초였습니다. 앞의 숫자는 빠른 것이 아니라 아무 일도 하지 않은 것입니다.

> [그림] 왼쪽에 criterion 결과 세 쌍이 막대로 있다. 레코드 1MB 훑기는 빌림 5.3µs 와 복사 65.6µs, u32 천 번 읽기는 안전 308.5ns 와 unsafe 308.9ns 로 같고, 벤치 함정은 black_box 없이 0.22ns, 있으면 566ns 이다. 오른쪽 sample 2초 쌓은 막대는 할당·해제 47%, 복사 30%, 나머지이고 검수 로봇이 그 막대를 본다

왼쪽은 벤치마크, 오른쪽은 프로파일러의 결과입니다. 질문이 다르니 도구도 다릅니다.

Rust 강의 21편입니다. 이 연재는 `Instant` 로 손수 재 왔습니다. [5편](https://blog.doksam.com/blog/rust-course-05-parser-cli/)의 빌림 대 복사, [19편](https://blog.doksam.com/blog/rust-course-19-layout-unsafe/)의 안전한 판 대 `unsafe` 판을 이번에 criterion 0.8.2 로 다시 잽니다. 측정은 2026-11-18, Apple M4 Pro, rustc 1.100.0 에서 했습니다.

## criterion 은 반복해 재고 신뢰 구간을 냅니다

개발 의존성으로 넣고 `benches/` 에 벤치 파일을 둡니다. `Cargo.toml` 에 `[[bench]] name = "parse"`, `harness = false` 를 적어 기본 테스트 하네스 대신 criterion 이 `main` 을 맡게 합니다.

```rust
use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use perf::*;

fn records(c: &mut Criterion) {
    let data = sample_1mb();
    let mut g = c.benchmark_group("records_1mb");
    g.bench_function("borrowed", |b| b.iter(|| count_borrowed(black_box(&data))));
    g.bench_function("copied", |b| b.iter(|| collect_copied(black_box(&data))));
    g.finish();
}

criterion_group!(benches, records, read_u32, pitfall);
criterion_main!(benches);
```

`cargo bench` 의 출력입니다(빌드는 처음 한 번 8.7초).

```console
records_1mb/borrowed    time:   [5.2830 µs 5.2872 µs 5.2923 µs]
records_1mb/copied      time:   [65.490 µs 65.621 µs 65.747 µs]
read_u32_x1000/safe     time:   [308.03 ns 308.51 ns 309.02 ns]
read_u32_x1000/unsafe   time:   [308.08 ns 308.85 ns 309.86 ns]
```

세 숫자는 신뢰 구간의 아래, 추정값, 위입니다. criterion 은 데이터를 데우고 여러 번 표본을 뽑아 통계로 줄입니다. 그래서 손으로 한 번 잰 값보다 흔들림이 작습니다. 표본에서 벗어난 값(outlier)도 셉니다. 이번에는 100번 가운데 3~10번이었습니다.

결과를 이 연재의 앞선 측정과 맞춰 봤습니다.

| 비교 | 앞선 손 측정 | criterion | 해석 |
|---|---|---|---|
| 1MB 레코드: 빌림 대 복사 | 5편: 2.1ms 대 17.9ms (256번 반복) | 5.29µs 대 65.6µs | 캐시에 들면 빌림이 12배 빠르다. 방향이 같다 |
| u32 읽기: 안전 대 `unsafe` | 19편: 재지 않음 | 308.5ns 대 308.9ns | 차이가 신뢰 구간 안이다 |

둘째 줄이 중요합니다. 범위를 확인하는 안전한 판과 `read_unaligned` 를 쓴 `unsafe` 판이 같았습니다. "`unsafe` 가 빠르다"는 믿음으로 코드를 바꾸기 전에 재야 하는 이유입니다.

## 같은 벤치를 다시 돌리면 변화를 판정합니다

criterion 은 지난 결과를 `target/criterion/` 에 남기고 다음 실행과 비교합니다. 코드를 바꾸지 않고 다시 돌렸습니다.

```console
read_u32_x1000/safe     time:   [307.83 ns 308.33 ns 308.81 ns]
                        change: [−0.2708% −0.0592% +0.1431%] (p = 0.58 > 0.05)
                        No change in performance detected.
read_u32_x1000/unsafe   time:   [309.18 ns 309.64 ns 310.11 ns]
                        change: [+0.1560% +0.4062% +0.6625%] (p = 0.00 < 0.05)
                        Change within noise threshold.
```

둘째 줄은 통계적으로는 달라졌지만(p < 0.05) 기본 잡음 기준(±1%) 안이라 의미 없는 변화로 판정했습니다. 코드를 바꾸지 않아도 0.4% 는 움직입니다. 그래서 "1% 빨라졌다" 같은 주장은 이 잡음보다 커야 믿습니다.

## black_box 를 빼면 일을 안 한 코드가 빨라 보입니다

```rust
    g.bench_function("no_black_box", |b| b.iter(|| (0..10_000u64).sum::<u64>()));
    g.bench_function("black_box", |b| b.iter(|| black_box(&v).iter().sum::<u64>()));
```

```console
pitfall_sum_10k/no_black_box
                        time:   [222.55 ps 222.84 ps 223.23 ps]
pitfall_sum_10k/black_box
                        time:   [564.71 ns 565.63 ns 566.60 ns]
```

첫째는 0.22나노초입니다. 1만 개를 더하는 데 CPU 클록 한 번 남짓밖에 안 걸렸다는 뜻이니 실제로 더하지 않은 것입니다. 입력이 상수라 컴파일러가 컴파일할 때 합을 계산해 상수로 바꿨다고 보는 것이 자연스럽습니다. `black_box` 는 "이 값을 컴파일러가 모르는 것처럼 다뤄라"는 표시입니다. 1편부터 이 연재의 손 측정에 `black_box` 를 넣은 이유가 이것입니다. 결과가 믿기지 않게 빠르면 먼저 이 함정을 의심합니다.

## 어디가 느린지는 프로파일러로 봅니다

벤치마크는 "얼마나"를 알려 주고 "왜"는 알려 주지 않습니다. 복사 판이 왜 느린지 보려고, 두 판을 4초 동안 번갈아 도는 프로그램을 macOS 에 기본으로 있는 `sample` 로 2초 떴습니다. 1밀리초마다 호출 스택을 찍는 표본 수집 프로파일러입니다.

```console
$ sample <pid> 2 -file hot.sample.txt
Sort by top of stack, same collapsed (when >= 5):
        _platform_memmove  (in libsystem_platform.dylib)        339
        _xzm_free  (in libsystem_malloc.dylib)        280
        _RNvCscLSrwpA01cJ_3hot4main  (in hot)        251
        _xzm_xzone_malloc_tiny  (in libsystem_malloc.dylib)        147
        _platform_memset  (in libsystem_platform.dylib)        68
```

스택 맨 위에 찍힌 표본 1,523개 가운데 할당·해제 함수가 709개(47%), 메모리 복사·채우기가 458개(30%)였습니다. 복사 판의 시간은 바이트를 읽는 데가 아니라 `Vec` 을 만들고 버리고 옮기는 데 들었습니다. 5편 "한 단계 더"의 해석을 프로파일러가 뒷받침합니다. 해결책도 여기서 나옵니다. 복사가 꼭 필요하면 버퍼를 다시 쓰고, 아니면 빌립니다.

## flamegraph 는 개념만 적습니다

flamegraph 는 이런 호출 스택 표본을 쌓아, 가로 폭이 시간 비중인 그림으로 그립니다. 넓은 칸이 시간을 많이 쓴 함수입니다. `cargo flamegraph` 가 흔히 쓰이고, 리눅스에서는 `perf`, macOS 에서는 `dtrace` 로 표본을 모읍니다. 이 맥에는 `cargo flamegraph` 가 깔려 있지 않았고 `dtrace` 는 관리자 권한이 필요해서 돌리지 않았습니다. 위 `sample` 출력이 같은 정보를 글로 보여 준 것입니다. 그림을 원하면 리눅스 서버에서 `perf record` 와 flamegraph 도구로 뜹니다.

## 면접에서 이렇게 묻는다

**Q1. 벤치마크에서 흔한 실수는?**
입력을 `black_box` 로 가리지 않아 컴파일러가 일을 지우는 것입니다. 실측에서 0.22ns 가 나왔습니다. 디버그 빌드로 재기, 한 번만 재기, 데우지 않고 재기도 흔한 실수입니다.

**Q2. criterion 은 손으로 `Instant` 를 쓰는 것과 무엇이 다른가요?**
데우기, 표본 반복, 신뢰 구간, outlier 표시, 지난 결과와의 비교와 잡음 기준 판정을 해 줍니다. 실측에서 바꾸지 않은 코드도 0.4% 움직였고 criterion 은 이를 잡음으로 판정했습니다.

**Q3. 벤치마크와 프로파일러는 언제 쓰나요?**
두 구현을 비교할 때는 벤치마크, 프로그램 전체에서 시간이 새는 곳을 찾을 때는 프로파일러입니다. 프로파일러로 뜨거운 곳을 찾고, 고친 뒤 벤치마크로 확인합니다.

**Q4. `unsafe` 로 바꾸면 빨라지나요?**
재 보기 전에는 모릅니다. 범위 확인 판과 `read_unaligned` 판이 308.5ns 대 308.9ns 로 같았습니다. 컴파일러가 안전한 판의 검사를 이미 지우는 경우가 많습니다(8편).

**Q5. 할당을 줄이는 방법은?**
버퍼를 다시 쓰고(`clear()` 후 재사용), 미리 용량을 잡고(`with_capacity`), 복사 대신 빌립니다. 프로파일에서 복사 판 시간의 47% 가 할당·해제였습니다.

## 연습

**연습 1.** `collect_copied` 를 버퍼 하나를 다시 쓰는 판으로 바꾸고 criterion 으로 비교할 계획을 세우세요.

<details>
<summary>풀이 보기</summary>

`fn sum_copied_reuse(data: &[u8], buf: &mut Vec<u8>) -> usize` 처럼 호출자가 버퍼를 넘기고, 레코드마다 `buf.clear(); buf.extend_from_slice(body);` 로 같은 버퍼를 씁니다. 벤치에서는 `let mut buf = Vec::new();` 를 `b.iter` 밖에 두고 같은 그룹에 `copied_reuse` 로 더합니다. 이 판은 이번에 재지 않았습니다. 프로파일의 할당·해제 47% 가 사라질 것으로 예상하지만, 예상은 측정으로 확인해야 합니다.

</details>

**연습 2.** 벤치 결과가 `time: [1.2 ps 1.3 ps 1.4 ps]` 처럼 나왔다면 무엇을 먼저 의심하나요?

<details>
<summary>풀이 보기</summary>

일을 안 한 것입니다. 1피코초는 CPU 한 클록보다 짧습니다. 입력과 결과를 `black_box` 로 가렸는지, 결과를 버리는 코드라 통째로 지워지지 않았는지 봅니다. `b.iter` 의 클로저가 값을 돌려주게 하면 criterion 이 그 값을 `black_box` 로 받습니다.

</details>

## 한 단계 더 — 측정의 조건을 함께 적습니다

이 연재의 모든 수치에는 날짜와 장비를 적었습니다. 성능 수치는 조건이 바뀌면 그대로 낡기 때문입니다. 같은 비교가 데이터가 캐시에 들면 빌림이 8~12배 빨랐고(5편 1MB, 이 편), 캐시를 넘는 256MB 에서는 거꾸로 복사가 빨랐습니다(5편). 6편에서 LTO 를 켜자 이 도구는 느려졌습니다. 해석하면 성능 판단은 "무엇이 빠르다"가 아니라 "이 조건에서 이 정도"입니다. 그래서 측정을 보고할 때는 입력 크기, 빌드 프로필, 장비, 날짜를 함께 적고, 결정을 바꿀 수 있는 조건(예: 데이터가 캐시를 넘는가)을 한 줄 남깁니다.

## 정리

비교는 criterion 으로, 원인은 프로파일러로 찾습니다. 입력은 `black_box` 로 가리고, 잡음보다 큰 차이만 믿습니다. 결과에는 조건을 함께 적습니다.

22편에서는 FFI 를 다룹니다. C 에서 Rust 를, Rust 에서 C 를 부르고, 패닉이 경계를 넘으면 어떻게 되는지 실제로 돌려 봅니다.
