//! A language-agnostic natural-language tagger written in Rust.
//!
//! This is in early stages of development.

mod language_detection;
mod unicode_segmentation;

pub use language_detection::LanguageDetector;

pub use crate::unicode_segmentation::token_position_indices;
