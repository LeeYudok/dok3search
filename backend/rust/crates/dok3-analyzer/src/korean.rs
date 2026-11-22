//! Dictionary-backed expansion for Korean banking and development terms.

const PARTICLES: &[&str] = &[
    "입니다",
    "에서",
    "에게",
    "으로",
    "부터",
    "까지",
    "처럼",
    "보다",
    "이다",
    "은",
    "는",
    "이",
    "가",
    "을",
    "를",
    "에",
    "로",
    "와",
    "과",
    "의",
    "도",
    "만",
];

fn compound(word: &str) -> &'static [&'static str] {
    match word {
        "계좌잔액조회" => &["계좌", "잔액", "조회"],
        "거래내역조회" => &["거래", "내역", "조회"],
        "여신심사기준" => &["여신", "심사", "기준"],
        "조기경보모형" => &["조기", "경보", "모형"],
        "신용평가모형" => &["신용", "평가", "모형"],
        "폐쇄망배포" => &["폐쇄망", "배포"],
        "세대전환" => &["세대", "전환"],
        "장애복구" => &["장애", "복구"],
        "변경저널" => &["변경", "저널"],
        "원문저장소" => &["원문", "저장소"],
        "권한범위" => &["권한", "범위"],
        "검색결과" => &["검색", "결과"],
        "색인생성" => &["색인", "생성"],
        _ => &[],
    }
}

// Ordinary words can end in the same syllables as particles. Only expand
// attested stems so words such as "데이터로더" and "데이터베이스" stay intact.
fn known_stem(word: &str) -> bool {
    !compound(word).is_empty()
        || matches!(
            word,
            "계좌"
                | "잔액"
                | "조회"
                | "계좌조회"
                | "거래"
                | "내역"
                | "거래내역"
                | "여신"
                | "심사"
                | "기준"
                | "조기"
                | "경보"
                | "모형"
                | "신용"
                | "평가"
                | "고객"
                | "은행"
                | "입금"
                | "출금"
                | "예금"
                | "대출"
                | "금리"
                | "폐쇄망"
                | "배포"
                | "세대"
                | "전환"
                | "장애"
                | "복구"
                | "변경"
                | "저널"
                | "원문"
                | "저장소"
                | "권한"
                | "범위"
                | "검색"
                | "결과"
                | "색인"
                | "생성"
                | "문서"
                | "인덱스"
                | "서버"
                | "코드"
                | "테스트"
                | "데이터"
                | "데이터베이스"
        )
}

pub(super) fn expansions(word: &str) -> impl Iterator<Item = &str> {
    let mut stem = None;
    let mut components = compound(word);
    if components.is_empty() && !known_stem(word) && word.ends_with(|c| matches!(c, '가'..='힣'))
    {
        stem = PARTICLES
            .iter()
            .filter_map(|particle| word.strip_suffix(particle))
            .find(|candidate| {
                candidate.chars().count() >= 2
                    && candidate.chars().all(|c| matches!(c, '가'..='힣'))
                    && known_stem(candidate)
            });
        if let Some(stem) = stem {
            components = compound(stem);
        }
    }
    stem.into_iter().chain(components.iter().copied())
}
