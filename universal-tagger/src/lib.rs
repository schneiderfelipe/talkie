//! A language-agnostic natural-language tagger written in Rust.

mod language_detection;
mod unicode_segmentation;

pub use language_detection::LanguageDetector;

pub use crate::unicode_segmentation::split_token_indices;
