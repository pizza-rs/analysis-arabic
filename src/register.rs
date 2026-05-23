//! Register Arabic analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::{
    Analyzer, AnalysisFactory, LowercaseNormalizer, Normalizer, StandardTokenizer, TokenFilter,
    Tokenizer,
};

use crate::{ArabicNormalizationFilter, ArabicStemFilter, ArabicStopFilter};

/// Register Arabic token filters and the `"arabic"` analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("arabic_normalization", Box::new(ArabicNormalizationFilter::new()));
    factory.register_token_filter("arabic_stem", Box::new(ArabicStemFilter::new()));
    factory.register_token_filter("arabic_stop", Box::new(ArabicStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![Box::new(LowercaseNormalizer::new())];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(ArabicNormalizationFilter::new()),
        Box::new(ArabicStopFilter::new()),
        Box::new(ArabicStemFilter::new()),
    ];
    factory.register_analyzer("arabic", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("arabic_normalization").is_some());
        assert!(factory.get_token_filter("arabic_stem").is_some());
        assert!(factory.get_token_filter("arabic_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("arabic").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("arabic").unwrap();
        let mut input = String::from("الكتاب في المكتبة");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        // "في" is stop word
        assert!(!tokens.iter().any(|t| t.term == "في"));
        assert!(tokens.len() >= 1);
    }
}
