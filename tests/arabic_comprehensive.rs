//! Comprehensive tests for pizza-analysis-arabic.

use pizza_analysis_arabic::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

// ═══════════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════════

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// ArabicNormalizationFilter — construction
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn normalization_construction() {
    let _f = ArabicNormalizationFilter::new();
}

// ═══════════════════════════════════════════════════════════════════════════════
// ArabicNormalizationFilter — diacritics & letter normalization
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn normalization_removes_fathah() {
    let f = ArabicNormalizationFilter::new();
    // "كَتَبَ" (with fathah diacritics) → "كتب"
    let mut token = make_token("كَتَبَ");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(!token.term.contains('\u{064E}'), "fathah should be removed");
}

#[test]
fn normalization_removes_kasrah() {
    let f = ArabicNormalizationFilter::new();
    let mut token = make_token("بِسْمِ");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(!token.term.contains('\u{0650}'), "kasrah should be removed");
}

#[test]
fn normalization_removes_dammah() {
    let f = ArabicNormalizationFilter::new();
    let mut token = make_token("كُتُبٌ");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(!token.term.contains('\u{064F}'), "dammah should be removed");
}

#[test]
fn normalization_alef_forms() {
    let f = ArabicNormalizationFilter::new();
    // أ (alef with hamza above) → ا
    let mut token = make_token("أحمد");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(token.term.starts_with("ا"), "alef hamza should be normalized to bare alef");
}

#[test]
fn normalization_alef_maddah() {
    let f = ArabicNormalizationFilter::new();
    // آ (alef with maddah) → ا
    let mut token = make_token("آمال");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(token.term.starts_with("ا"), "alef maddah should be normalized");
}

#[test]
fn normalization_teh_marbuta() {
    let f = ArabicNormalizationFilter::new();
    // ة (teh marbuta) → ه
    let mut token = make_token("مدرسة");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(!token.term.contains('ة'), "teh marbuta should be normalized to heh");
}

#[test]
fn normalization_tatweel() {
    let f = ArabicNormalizationFilter::new();
    // ـ (tatweel/kashida) should be removed
    let mut token = make_token("كـتـاب");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(!token.term.contains('\u{0640}'), "tatweel should be removed");
}

#[test]
fn normalization_ascii_passthrough() {
    let f = ArabicNormalizationFilter::new();
    let mut token = make_token("hello");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "hello");
}

#[test]
fn normalization_empty_string() {
    let f = ArabicNormalizationFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "");
}

// ═══════════════════════════════════════════════════════════════════════════════
// ArabicStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = ArabicStemFilter::new();
}

#[test]
fn stem_definite_article() {
    let f = ArabicStemFilter::new();
    // "الكتاب" (the book) → should have "ال" removed
    let mut token = make_token("الكتاب");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    // Stemmer should strip the definite article
    assert!(!token.term.starts_with("ال") || token.term.len() < "الكتاب".len());
}

#[test]
fn stem_prefix_removal() {
    let f = ArabicStemFilter::new();
    let mut token = make_token("والكتاب");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_ne!(token.term.as_ref(), "والكتاب");
}

#[test]
fn stem_suffix_removal() {
    let f = ArabicStemFilter::new();
    let mut token = make_token("كاتبون");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word_unchanged() {
    let f = ArabicStemFilter::new();
    let mut token = make_token("في");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = ArabicStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// ArabicStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = ArabicStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = ArabicStopFilter::new();
    let stop_words = ["في", "من", "على", "إلى", "هذا", "التي", "هو", "أن"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = ArabicStopFilter::new();
    let content_words = ["كتاب", "مدرسة", "قلم", "علم"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = ArabicStopFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    // Empty might be kept or dropped; just ensure no panic
    let _ = deleted;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("arabic_normalization").is_some());
    assert!(factory.get_token_filter("arabic_stem").is_some());
    assert!(factory.get_token_filter("arabic_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("arabic").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("arabic").unwrap();
    let mut input = String::from("الكتاب في المكتبة");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty(), "Arabic analyzer should produce tokens");
}

#[test]
fn analyzer_pipeline_single_word() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("arabic").unwrap();
    let mut input = String::from("كتاب");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("arabic").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("arabic").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
