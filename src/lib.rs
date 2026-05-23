#![cfg_attr(not(feature = "std"), no_std)]
//! Arabic language analysis for Pizza search engine.
//!
//! Provides a full-featured Arabic analyzer with normalization (diacritics
//! removal, letter form normalization), stemming (prefix/suffix removal),
//! and stop words.
//!
//! # Components
//!
//! - [`ArabicNormalizationFilter`] — Removes diacritics, normalizes letter forms
//! - [`ArabicStemFilter`] — Removes definite articles and common affixes
//! - [`ArabicStopFilter`] — Arabic stop words filter
extern crate alloc;
mod normalization;
mod stem;
mod stop;

pub mod register;

pub use normalization::ArabicNormalizationFilter;
pub use register::register_all;
pub use stem::ArabicStemFilter;
pub use stop::ArabicStopFilter;
