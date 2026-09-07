//! Reference stemmer re-exported from `pizza-analysis-core`.
//!
//! The crate used to carry its own approximate stemmer; the validated
//! reference port (differential-tested against the upstream algorithm)
//! lives in analysis-core and is shared by every consumer now.

pub use pizza_analysis_core::ArabicStemTokenFilter as ArabicStemFilter;
