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

macro_rules! lexicon {
    ($($w:literal)|+ $(|)?) => {
        const LEXICON: &[&str] = &[$($w),+];

        fn is_lexicon_stem(word: &str) -> bool {
            matches!(word, $($w)|+)
        }
    };
}

lexicon! {
    "설계" |
    "검토" |
    "보완" |
    "결과" |
    "범위" |
    "결론" |
    "데이터" |
    "정합성" |
    "보안" |
    "원자적" |
    "트랜잭션" |
    "부재" |
    "변경" |
    "삭제" |
    "누락" |
    "접근" |
    "제어" |
    "색인" |
    "문서" |
    "캐시" |
    "불일치" |
    "도구" |
    "출력" |
    "신뢰" |
    "경계" |
    "제품" |
    "완성도" |
    "품질" |
    "검색" |
    "점수" |
    "과잉" |
    "일치" |
    "교재" |
    "원문" |
    "보존" |
    "경로" |
    "프로토콜" |
    "혼동" |
    "워크스페이스" |
    "인터페이스" |
    "우선" |
    "개발" |
    "원칙" |
    "오류" |
    "코드" |
    "계약" |
    "단계" |
    "산출물" |
    "의존성" |
    "방향" |
    "인덱스" |
    "세대" |
    "전환" |
    "롤백" |
    "폐쇄망" |
    "자료" |
    "반입" |
    "기준" |
    "워커" |
    "실행" |
    "가이드" |
    "증분" |
    "점검" |
    "요청" |
    "예시" |
    "가시성" |
    "권한" |
    "분류" |
    "테스트" |
    "설정" |
    "서비스" |
    "상태" |
    "승인" |
    "수집" |
    "오프라인" |
    "배포" |
    "파일" |
    "검증" |
    "스냅샷" |
    "기간" |
    "관리" |
    "한글" |
    "검색어" |
    "분석" |
    "복합어" |
    "실패" |
    "작업" |
    "재시도" |
    "섹션" |
    "조회" |
    "응답" |
    "내부" |
    "출처" |
    "등록" |
    "운영" |
    "매뉴얼" |
    "자체" |
    "개요" |
    "환경" |
    "변수" |
    "기동" |
    "모드" |
    "폴더" |
    "규칙" |
    "준비" |
    "연동" |
    "열람" |
    "영속" |
    "디렉터리" |
    "스키마" |
    "버전" |
    "재색인" |
    "백업" |
    "복구" |
    "장애" |
    "진단" |
    "아키텍처" |
    "구조" |
    "구성" |
    "구현" |
    "규격" |
    "명세" |
    "요약" |
    "목차" |
    "본문" |
    "항목" |
    "절차" |
    "방식" |
    "방법" |
    "정책" |
    "지침" |
    "규정" |
    "목록" |
    "카테고리" |
    "필터" |
    "조건" |
    "예제" |
    "샘플" |
    "화면" |
    "에러" |
    "경고" |
    "차단" |
    "허용" |
    "확인" |
    "비교" |
    "대조" |
    "성능" |
    "부하" |
    "사용자" |
    "관리자" |
    "계정" |
    "인증" |
    "토큰" |
    "외부" |
    "공개" |
    "네트워크" |
    "컨테이너" |
    "설치" |
    "번들" |
    "패키지" |
    "체크섬" |
    "무결성" |
    "활성" |
    "활성화" |
    "퇴역" |
    "커넥터" |
    "어댑터" |
    "파서" |
    "형태소" |
    "분석기" |
    "사전" |
    "질의" |
    "질의어" |
    "키워드" |
    "태그" |
    "심볼" |
    "식별자" |
    "함수" |
    "모듈" |
    "크레이트" |
    "라이선스" |
    "저작권" |
    "저장소" |
    "브랜치" |
    "커밋" |
    "이슈" |
    "에이전트" |
    "일관성" |
    "커서" |
    "페이지" |
    "쿼리" |
    "테이블" |
    "저널" |
    "추가" |
    "수정" |
    "초기화" |
    "자동" |
    "수동" |
    "주기" |
    "이력" |
    "감사" |
    "로그" |
    "표준" |
    "입력" |
    "호출" |
    "연결" |
    "통합" |
    "분리" |
    "격리" |
    "보호" |
    "누출" |
    "방지" |
    "해결" |
    "대응" |
    "조치" |
    "적용" |
    "유지" |
    "보장" |
    "지원" |
    "제공" |
    "포함" |
    "제외" |
    "중복" |
    "충돌" |
    "호환" |
    "기반" |
    "대상" |
    "전체" |
    "기본" |
    "필수" |
    "선택" |
    "최대" |
    "최소" |
    "단위" |
    "문자" |
    "한국어" |
    "국문" |
    "영문" |
    "영어" |
    "계좌" |
    "잔액" |
    "거래" |
    "내역" |
    "여신" |
    "심사" |
    "조기" |
    "경보" |
    "모형" |
    "신용" |
    "평가" |
    "고객" |
    "은행" |
    "입금" |
    "출금" |
    "예금" |
    "대출" |
    "금리" |
    "생성"
}

const ATOMIC: &[&str] = &["계좌조회", "거래내역", "데이터베이스"];
const GLUE: &[char] = &['및', '와', '과', '의', '에', '로', '별', '용', '간'];

fn is_hangul(c: char) -> bool {
    matches!(c, '가'..='힣')
}

/// Full-cover dictionary segmentation of an unspaced Hangul compound.
/// Returns the lexicon stems (glue syllables dropped), or empty when the word
/// is not fully covered by at least two stems.
fn segment_hangul_compound(word: &str) -> Vec<&'static str> {
    let chars: Vec<char> = word.chars().collect();
    let n = chars.len();
    if n < 4 || !chars.iter().copied().all(is_hangul) || ATOMIC.contains(&word) {
        return Vec::new();
    }
    // best[i] = (segment count, stem count, back pointer: (prev index, stem or glue)) covering chars[..i]
    type Cell = Option<(usize, usize, usize, Option<&'static str>)>;
    let mut best: Vec<Cell> = vec![None; n + 1];
    best[0] = Some((0, 0, 0, None));
    for i in 0..n {
        let Some((segs, stems, _, last)) = best[i] else {
            continue;
        };
        for len in 2..=5usize {
            if i + len > n {
                break;
            }
            let piece: String = chars[i..i + len].iter().collect();
            if let Some(stem) = LEXICON.iter().copied().find(|s| *s == piece) {
                debug_assert!(is_lexicon_stem(stem));
                let cand = (segs + 1, stems + 1, i, Some(stem));
                if best[i + len].is_none_or(|b| (cand.0, cand.1) < (b.0, b.1)) {
                    best[i + len] = Some(cand);
                }
            }
        }
        // Glue: strictly medial, only right after a stem (never adjacent to glue).
        if i > 0 && i + 1 < n && last.is_some() && GLUE.contains(&chars[i]) {
            let cand = (segs + 1, stems, i, None);
            if best[i + 1].is_none_or(|b| (cand.0, cand.1) < (b.0, b.1)) {
                best[i + 1] = Some(cand);
            }
        }
    }
    let mut out = Vec::new();
    let mut at = n;
    if best[n].is_none() {
        return out;
    }
    while at > 0 {
        let (_, _, prev, stem) = best[at].expect("path");
        if let Some(stem) = stem {
            out.push(stem);
        }
        at = prev;
    }
    out.reverse();
    if out.len() < 2 {
        return Vec::new();
    }
    out
}

pub(super) fn expansions(word: &str) -> impl Iterator<Item = &str> {
    let mut items: Vec<&str> = Vec::new();
    let direct = compound(word);
    if !direct.is_empty() {
        items.extend(direct.iter().copied());
        return items.into_iter();
    }
    let segmented = segment_hangul_compound(word);
    if !segmented.is_empty() {
        return segmented.into_iter().collect::<Vec<_>>().into_iter();
    }
    if !known_stem(word) && word.ends_with(is_hangul) {
        let stem = PARTICLES
            .iter()
            .filter_map(|particle| word.strip_suffix(particle))
            .find(|candidate| {
                candidate.chars().count() >= 2
                    && candidate.chars().all(is_hangul)
                    && (known_stem(candidate) || !segment_hangul_compound(candidate).is_empty())
            });
        if let Some(stem) = stem {
            items.push(stem);
            let parts = compound(stem);
            if parts.is_empty() {
                items.extend(segment_hangul_compound(stem));
            } else {
                items.extend(parts.iter().copied());
            }
        }
    }
    items.into_iter()
}
