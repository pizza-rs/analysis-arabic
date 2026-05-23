//! Arabic stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Arabic stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "أ",
    "ألا",
    "أما",
    "أن",
    "أنت",
    "أنه",
    "أنها",
    "أو",
    "أى",
    "أي",
    "أيضا",
    "إذا",
    "إلا",
    "إلى",
    "إلي",
    "إما",
    "إن",
    "إنه",
    "إنها",
    "ا",
    "اذا",
    "الآن",
    "الا",
    "الان",
    "التى",
    "التي",
    "الذى",
    "الذي",
    "الذين",
    "الى",
    "الي",
    "اما",
    "ان",
    "انت",
    "انه",
    "انها",
    "او",
    "اى",
    "اي",
    "ايضا",
    "ب",
    "بأن",
    "بان",
    "بعد",
    "بعض",
    "به",
    "بها",
    "بين",
    "بينما",
    "تكون",
    "تلك",
    "ثم",
    "جميع",
    "حتى",
    "حيث",
    "خلال",
    "ذلك",
    "ضمن",
    "على",
    "عليه",
    "عليها",
    "عن",
    "عند",
    "عندما",
    "غير",
    "ف",
    "فأن",
    "فان",
    "فما",
    "فهو",
    "فهى",
    "فهي",
    "في",
    "فيه",
    "فيها",
    "قبل",
    "قد",
    "كان",
    "كانت",
    "كل",
    "كما",
    "لا",
    "لدى",
    "لك",
    "لكن",
    "لم",
    "لن",
    "له",
    "لها",
    "ما",
    "مع",
    "من",
    "منذ",
    "منه",
    "منها",
    "نحو",
    "هذا",
    "هذه",
    "هناك",
    "هو",
    "هى",
    "هي",
    "و",
    "وأن",
    "وإن",
    "وان",
    "وفي",
    "وكان",
    "وكانت",
    "وكل",
    "ولا",
    "ولم",
    "ولن",
    "وما",
    "ومن",
    "وهو",
    "وهى",
    "وهي",
    "يكون",
    ];
    words.iter().copied().collect()
});

/// Removes Arabic stop words from the token stream.
#[derive(Clone, Debug)]
pub struct ArabicStopFilter {
    stop_words: HashSet<String>,
}

impl Default for ArabicStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl ArabicStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for ArabicStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 119);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = ArabicStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = ArabicStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = ArabicStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
