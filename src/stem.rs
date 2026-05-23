//! Arabic stemmer — prefix/suffix stripping.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Arabic stemmer that removes common prefixes and suffixes.
///
/// Based on the Lucene ArabicStemmer — a simple prefix/suffix removal approach
/// that does not perform root extraction.
#[derive(Clone, Debug, Default)]
pub struct ArabicStemFilter;

impl ArabicStemFilter {
    pub fn new() -> Self {
        Self
    }
}

/// Arabic definite article and common prefixes.
const PREFIXES: &[&str] = &[
    "\u{0627}\u{0644}",   // al (definite article)
    "\u{0648}\u{0627}\u{0644}", // wal
    "\u{0628}\u{0627}\u{0644}", // bal
    "\u{0643}\u{0627}\u{0644}", // kal
    "\u{0641}\u{0627}\u{0644}", // fal
    "\u{0644}\u{0644}",   // ll
];

/// Common Arabic suffixes.
const SUFFIXES: &[&str] = &[
    "\u{0647}\u{0627}",   // ha
    "\u{0627}\u{0646}",   // an
    "\u{0627}\u{062A}",   // at
    "\u{0648}\u{0646}",   // wn
    "\u{064A}\u{0646}",   // yn
    "\u{064A}\u{0629}",   // yp
    "\u{064A}\u{0647}",   // yh
    "\u{0629}",           // p (teh marbuta)
    "\u{0647}",           // h
];

impl TokenFilter for ArabicStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let chars: Vec<char> = text.chars().collect();
        if chars.len() < 4 {
            return (false, None);
        }

        let mut result = text.to_string();
        let mut changed = false;

        // Strip prefixes
        for &prefix in PREFIXES {
            if result.starts_with(prefix) {
                let remaining: String = result.chars().skip(prefix.chars().count()).collect();
                if remaining.chars().count() >= 2 {
                    result = remaining;
                    changed = true;
                    break;
                }
            }
        }

        // Strip suffixes
        for &suffix in SUFFIXES {
            if result.ends_with(suffix) {
                let remaining_len = result.chars().count() - suffix.chars().count();
                if remaining_len >= 2 {
                    let remaining: String = result.chars().take(remaining_len).collect();
                    result = remaining;
                    changed = true;
                    break;
                }
            }
        }

        if changed {
            token.term = Cow::Owned(result);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_al_prefix() {
        let f = ArabicStemFilter::new();
        // "الكتاب" -> "كتاب"
        let mut token = Token::new("\u{0627}\u{0644}\u{0643}\u{062A}\u{0627}\u{0628}", 0, 12, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "\u{0643}\u{062A}\u{0627}\u{0628}");
    }

    #[test]
    fn test_short_word() {
        let f = ArabicStemFilter::new();
        let mut token = Token::new("\u{0641}\u{064A}", 0, 4, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "\u{0641}\u{064A}");
    }
}
