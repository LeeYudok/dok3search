# 설치

Chromium을 빌드하고 실행할 수 있는지 확인합니다. 코드가 비교적 최신이라면(커밋 위치 1223636 이후, 2023년 11월에 해당) 어떤 플랫폼이나 빌드 플래그 집합도 괜찮습니다.

``` shell
gn gen out/Debug
autoninja -C out/Debug chrome
out/Debug/chrome # or on Mac, out/Debug/Chromium.app/Contents/MacOS/Chromium
```

개발 속도를 빠르게 하고 싶다면 디버그 빌드를 사용하는 것이 좋습니다. 이게 기본값입니다.

아직 빌드하지 않았다면 [Chromium 빌드 방법](https://www.chromium.org/developers/how-tos/get-the-code/)을 참고하세요. 주의: Chromium 빌드 환경 셋업은 시간이 걸립니다.

또한 Visual Studio Code가 설치되어 있는 것이 좋습니다.

## About the exercises

과정의 이 부분에는 서로 연계되는 일련의 연습문제가 있습니다. 마지막에 한꺼번에 하지 않고 과정 전반에 연습문제가 흩어져 있습니다. 특정 부분을 완료할 시간이 없더라도 걱정하지 마세요. 다음번에 따라잡을 수 있습니다.
