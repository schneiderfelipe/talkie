//! A language-agnostic natural-language tagger written in Rust.
//!
//! This is in early stages of development.

mod language;
mod language_detection;
mod stop_words;
mod tagger;
mod unicode_segmentation;

pub use language::Language;
pub use language_detection::LanguageDetector;
pub use tagger::Tagger;

pub use crate::unicode_segmentation::{token_positions, Token};
