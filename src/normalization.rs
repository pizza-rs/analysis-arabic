//! Arabic normalization — diacritics removal and letter form normalization.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Normalizes Arabic text (diacritics removal, alef/yeh/teh normalization).
///
/// Equivalent to Lucene's `ArabicNormalizationFilter`.
#[derive(Clone, Debug, Default)]
pub struct ArabicNormalizationFilter;

impl ArabicNormalizationFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for ArabicNormalizationFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.is_empty() {
            return (false, None);
        }

        let mut result = String::with_capacity(text.len());
        let mut changed = false;

        for c in text.chars() {
            match c {
                '\u{0622}' | '\u{0623}' | '\u{0625}' => {
                    result.push('\u{0627}');
                    changed = true;
                }
                '\u{0649}' => {
                    result.push('\u{064A}');
                    changed = true;
                }
                '\u{0629}' => {
                    result.push('\u{0647}');
                    changed = true;
                }
                '\u{0640}' | '\u{064B}' | '\u{064C}' | '\u{064D}' | '\u{064E}' |
                '\u{064F}' | '\u{0650}' | '\u{0651}' | '\u{0652}' => {
                    changed = true;
                }
                _ => {
                    result.push(c);
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
    fn test_alef_normalization() {
        let f = ArabicNormalizationFilter::new();
        let mut token = Token::new("\u{0622}\u{0623}\u{0625}", 0, 6, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "\u{0627}\u{0627}\u{0627}");
    }

    #[test]
    fn test_remove_diacritics() {
        let f = ArabicNormalizationFilter::new();
        let mut token = Token::new("\u{0627}\u{064E}\u{0644}\u{0650}", 0, 8, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "\u{0627}\u{0644}");
    }
}
