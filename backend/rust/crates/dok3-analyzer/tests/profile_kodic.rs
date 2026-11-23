use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use dok3_analyzer::kodic::Dictionary;
use dok3_analyzer::{
    AnalyzeOptions, AnalyzerConfig, AnalyzerEngine, AnalyzerProfile, BUILTIN_ANALYZER_VERSION,
};

struct TempDir(PathBuf);

impl TempDir {
    fn new(prefix: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn write_minimal_kodic_source(dir: &Path) {
    fs::write(
        dir.join("char.def"),
        "\
DEFAULT 0 1 0
SPACE 0 1 0
HANGUL 1 1 0
ALPHA 1 1 0
NUMERIC 1 1 0
0x0020 SPACE
0xAC00..0xD7A3 HANGUL
0x0041..0x005A ALPHA
0x0061..0x007A ALPHA
0x0030..0x0039 NUMERIC
",
    )
    .unwrap();

    fs::write(
        dir.join("unk.def"),
        "\
DEFAULT,0,0,5000,SY,*,*,*,*,*,*,*
SPACE,0,0,0,SP,*,*,*,*,*,*,*
HANGUL,1,1,4000,NNG,*,*,*,*,*,*,*
ALPHA,1,1,4000,SL,*,*,*,*,*,*,*
NUMERIC,1,1,4000,SN,*,*,*,*,*,*,*
",
    )
    .unwrap();

    fs::write(
        dir.join("matrix.def"),
        "\
3 3
0 0 0
0 1 0
0 2 0
1 0 0
1 1 0
1 2 500
2 0 0
2 1 0
2 2 500
",
    )
    .unwrap();

    fs::write(
        dir.join("NNG.csv"),
        "\
폐쇄,1,1,100,NNG,*,*,폐쇄,*,*,*,*
개발,1,1,100,NNG,*,*,개발,*,*,*,*
폐쇄개발,1,1,50,NNG,*,*,폐쇄개발,Compound,NNG,NNG,폐쇄/NNG/*+개발/NNG/*
설계,1,1,100,NNG,*,*,설계,*,*,*,*
검토,1,1,100,NNG,*,*,검토,*,*,*,*
보완,1,1,100,NNG,*,*,보완,*,*,*,*
결과,1,1,100,NNG,*,*,결과,*,*,*,*
",
    )
    .unwrap();

    fs::write(
        dir.join("JKO.csv"),
        "\
을,2,2,100,JKO,*,*,을,*,*,*,*
를,2,2,100,JKO,*,*,를,*,*,*,*
은,2,2,100,JX,*,*,은,*,*,*,*
는,2,2,100,JX,*,*,는,*,*,*,*
",
    )
    .unwrap();
}

#[test]
fn compiles_minimal_kodic_and_switches_profiles() {
    let tmp = TempDir::new("dok3-kodic-test");
    let src = tmp.path().join("src");
    fs::create_dir_all(&src).unwrap();
    write_minimal_kodic_source(&src);

    let bin = tmp.path().join("ko-dic.bin");
    let dict = Dictionary::compile(&src).expect("compile minimal ko-dic");
    dict.save(&bin).expect("save minimal ko-dic.bin");

    let opts = AnalyzeOptions {
        split_identifiers: true,
        char_ngrams: false,
    };

    // Builtin keeps legacy heuristic behavior and version_tag == 2.
    let builtin = AnalyzerEngine::builtin();
    assert_eq!(builtin.version_tag(), BUILTIN_ANALYZER_VERSION);
    let builtin_terms = builtin.analyze_text("폐쇄개발", opts).terms;
    assert!(!builtin_terms.contains(&"폐쇄".to_string()));

    // Lindera emits surface morphemes including particles.
    let lindera = AnalyzerEngine::from_config(&AnalyzerConfig {
        profile: AnalyzerProfile::Lindera,
        dict_path: Some(bin.clone()),
        user_dict_path: None,
    })
    .unwrap();
    assert_ne!(lindera.version_tag(), BUILTIN_ANALYZER_VERSION);
    let lindera_terms = lindera
        .analyze_text("폐쇄개발을 ORA-00904 snake_case", opts)
        .terms;
    assert!(lindera_terms.contains(&"폐쇄개발".to_string()));
    assert!(lindera_terms.contains(&"을".to_string()));
    assert!(lindera_terms.contains(&"ora-00904".to_string()));
    assert!(lindera_terms.contains(&"snake".to_string()));
    assert!(lindera_terms.contains(&"case".to_string()));

    // NoriMixed emits compound surface + parts and drops J*/E*/S*.
    let nori_mixed = AnalyzerEngine::from_config(&AnalyzerConfig {
        profile: AnalyzerProfile::NoriMixed,
        dict_path: Some(bin.clone()),
        user_dict_path: None,
    })
    .unwrap();
    assert_ne!(nori_mixed.version_tag(), lindera.version_tag());
    let mixed_terms = nori_mixed.analyze_text("폐쇄개발을", opts).terms;
    assert!(mixed_terms.contains(&"폐쇄개발".to_string()));
    assert!(mixed_terms.contains(&"폐쇄".to_string()));
    assert!(mixed_terms.contains(&"개발".to_string()));
    assert!(!mixed_terms.contains(&"을".to_string()));

    // NoriDiscard decomposes compound into parts without re-emitting the compound surface.
    let nori_discard = AnalyzerEngine::from_config(&AnalyzerConfig {
        profile: AnalyzerProfile::NoriDiscard,
        dict_path: Some(bin),
        user_dict_path: None,
    })
    .unwrap();
    assert_ne!(nori_discard.version_tag(), nori_mixed.version_tag());
    let discard_terms = nori_discard.analyze_text("폐쇄개발을", opts).terms;
    assert!(discard_terms.contains(&"폐쇄".to_string()));
    assert!(discard_terms.contains(&"개발".to_string()));
    assert!(!discard_terms.contains(&"폐쇄개발".to_string()));
    assert!(!discard_terms.contains(&"을".to_string()));
}

#[test]
fn requires_dict_path_for_kodic_profiles() {
    for profile in [
        AnalyzerProfile::Lindera,
        AnalyzerProfile::NoriMixed,
        AnalyzerProfile::NoriDiscard,
    ] {
        let err = AnalyzerEngine::from_config(&AnalyzerConfig {
            profile,
            dict_path: None,
            user_dict_path: None,
        })
        .err()
        .expect("missing dict_path must fail");
        assert!(err.contains("DOK3_DICT_PATH"));
    }
}

#[test]
fn local_real_kodic_segments_unregistered_compounds_and_code() {
    let real_bin = Path::new("/Users/dok123/workspace/gitlab.doksam.com/tantivy/dict/ko-dic.bin");
    if !real_bin.is_file() {
        return;
    }

    let engine = AnalyzerEngine::from_config(&AnalyzerConfig {
        profile: AnalyzerProfile::NoriMixed,
        dict_path: Some(real_bin.to_path_buf()),
        user_dict_path: None,
    })
    .expect("load real ko-dic.bin");

    let opts = AnalyzeOptions {
        split_identifiers: true,
        char_ngrams: false,
    };
    let terms_compound = engine.analyze_text("폐쇄개발", opts).terms;
    assert!(terms_compound.contains(&"폐쇄".to_string()));
    assert!(terms_compound.contains(&"개발".to_string()));

    let terms_long = engine.analyze_text("설계검토및보완결과", opts).terms;
    assert!(terms_long.contains(&"설계".to_string()));
    assert!(terms_long.contains(&"검토".to_string()));
    assert!(terms_long.contains(&"보완".to_string()));
    assert!(terms_long.contains(&"결과".to_string()));

    let terms_code = engine.analyze_text("ORA-00904 camelCaseName", opts).terms;
    assert!(terms_code.contains(&"ora-00904".to_string()));
    assert!(terms_code.contains(&"camel".to_string()));
    assert!(terms_code.contains(&"case".to_string()));
    assert!(terms_code.contains(&"name".to_string()));
}
