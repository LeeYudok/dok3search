# Comprehensive Rust 한국어 자료

Google Android 팀과 기여자가 만든 Rust 강의를 검색용 Markdown으로 옮겼다. 2026-11-18(KST)에 게시된 인쇄본과 목차에서 337개 장을 가져왔다. `index.json`에 두 입력 파일의 SHA-256을 기록했다. [강의](https://google.github.io/comprehensive-rust/ko/) · [원본 저장소](https://github.com/google/comprehensive-rust)

`chapters/` 아래 파일 하나가 강의 한 장이다. 코드 블록, 발표자 노트, 그림을 유지하고 장 사이 링크는 상대 경로로 바꿨다. 공식 한국어판에 남아 있는 영어는 그대로 담았다. 온라인 PDF, 연습 템플릿, 외부 참고 문서의 링크는 원본 주소를 유지한다.

문서와 그림은 [CC BY 4.0](LICENSE-CC-BY), 예제 코드는 [Apache 2.0](LICENSE-APACHE)을 따른다. [원본 라이선스 설명](https://github.com/google/comprehensive-rust#license)

## 자료 변환

Python 3.11 이상과 Beautiful Soup, lxml, Pandoc이 있는 준비 환경에서 실행한다. 입력 폴더에 한국어판 `print.html`·`toc.html`, 원본 저장소의 `book.toml`·`LICENSE`(`LICENSE-APACHE`로 저장)·`LICENSE-CC-BY`, 인쇄본에서 참조한 그림 세 개를 `assets/` 아래 원래 경로로 둔다. 변환기는 이 폴더에서만 파일을 읽는다.

```sh
python3 scripts/import-comprehensive-rust.py <snapshot-dir> <output-dir>
```

직접 가져오는 그림 경로는 `android/interoperability/cpp/overview.svg`, `chromium/build-rules/vscode.png`, `exercises/chromium/chwomium.png`다. 인쇄본에 내장된 SVG 그림은 변환기가 추출한다. [인쇄본](https://google.github.io/comprehensive-rust/ko/print.html) · [목차](https://google.github.io/comprehensive-rust/ko/toc.html)
