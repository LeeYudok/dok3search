# 연습문제: Builder 타입

이 예에서는 모든 데이터를 소유하는 복잡한 데이터 타입을 구현합니다. 편의 함수를 사용하여 새로운 값을 하나씩 빌드하도록 지원하기 위해 ’빌더 패턴’을 사용합니다.

누락된 부분을 채우세요.

패닉 발생 예제

``` rust
#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// 소프트웨어 패키지를 나타냅니다.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    fn as_dependency(&self) -> Dependency {
        todo!("1")
    }
}

/// 패키지용 빌더입니다. `build()`를 사용하여 `Package` 자체를 만듭니다.
struct PackageBuilder(Package);

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        todo!("2")
    }

    /// 패키지 버전을 설정합니다.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// 패키지 작성자를 설정합니다.
    fn authors(mut self, authors: Vec<String>) -> Self {
        todo!("3")
    }

    /// 종속 항목을 추가합니다.
    fn dependency(mut self, dependency: Dependency) -> Self {
        todo!("4")
    }

    /// 언어를 설정합니다. 설정하지 않으면 언어가 기본적으로 None으로 설정됩니다.
    fn language(mut self, language: Language) -> Self {
        todo!("5")
    }

    fn build(self) -> Package {
        self.0
    }
}

fn main() {
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");
    let log =
        PackageBuilder::new("log").version("0.4").language(Language::Rust).build();
    println!("log: {log:?}");
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");
}
```
