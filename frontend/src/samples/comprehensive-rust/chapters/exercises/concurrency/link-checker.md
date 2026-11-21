# 멀티스레드 링크 검사기

새로 배운것들을 활용해서 멀티 스레드 링크 검사기를 만듭니다. 이 검사기는 웹페이지 안에 있는 링크들이 유효한지 확인합니다. 그리고 재귀적으로 동일 도메인의 다른 모든 페이지가 유효한지 확인합니다.

이를 위해서 [`reqwest`](https://docs.rs/reqwest/)와 같은 HTTP 클라이언트가 필요합니다. 새로운 로컬 프로젝트를 만들고 [`reqwest`](https://docs.rs/reqwest/)를 의존성에 추가하십시요:

``` shell
cargo new link-checker
cd link-checker
cargo add --features blocking,rustls-tls reqwest
```

> 만일 `cargo add` 커맨드가 `error: no such subcommand` 로 실패한다면 `Cargo.toml` 파일을 직접 수정해도 됩니다. 아래에 전체 의존성 내용이 있습니다.

링크를 찾기 위해서 [`scraper`](https://docs.rs/scraper/)도 추가합니다:

``` shell
cargo add scraper
```

마지막으로 오류 처리하는 방법으로 [`thiserror`](https://docs.rs/thiserror/)도 추가합니다:

``` shell
cargo add thiserror
```

모든 `cargo add`가 끝나면 `Cargo.toml`에 아래 내용이 추가됩니다:

``` toml
[package]
name = "link-checker"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
reqwest = { version = "0.11.12", features = ["blocking", "rustls-tls"] }
scraper = "0.13.0"
thiserror = "1.0.37"
```

이제 `https://www.google.org/` 같은 웹 페이지를 탐색할 수 있습니다.

`rc/main.rs`파일은 아래와 같습니다:

컴파일 오류 예제

``` rust
use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("요청 오류: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("잘못된 http 응답: {0}")]
    BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand {
    url: Url,
    extract_links: bool,
}

fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
    println!("{:#} 확인 중", command.url);
    let response = client.get(command.url.clone()).send()?;
    if !response.status().is_success() {
        return Err(Error::BadResponse(response.status().to_string()));
    }

    let mut link_urls = Vec::new();
    if !command.extract_links {
        return Ok(link_urls);
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    for href in href_values {
        match base_url.join(href) {
            Ok(link_url) => {
                link_urls.push(link_url);
            }
            Err(err) => {
                println!("{base_url:#}에서: 파싱할 수 없는 {href:?}: {err}을(를) 무시함");
            }
        }
    }
    Ok(link_urls)
}

fn main() {
    let client = Client::new();
    let start_url = Url::parse("https://www.google.org").unwrap();
    let crawl_command = CrawlCommand{ url: start_url, extract_links: true };
    match visit_page(&client, &crawl_command) {
        Ok(links) => println!("링크: {links:#?}"),
        Err(err) => println!("링크를 추출할 수 없습니다: {err:#}"),
    }
}
```

아래 커맨드로 소스를 실행합니다

``` shell
cargo run
```

## 태스크

- 스레드를 사용하여 링크를 병렬로 확인합니다: URL을 채널로 보내서 몇 개의 스레드가 URL을 병렬로 체크하도록 합니다.
- `www.google.org`도메인의 모든 페이지를 재귀적으로 확인하기 위해 코드를 확장해서 작성합니다: 차단당하지 않도록 100페이지 정도로 제한을 두시기 바랍니다.
