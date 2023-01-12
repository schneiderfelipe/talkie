use itertools::{Itertools, Position};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
struct UnicodeSegmenter {}

type UnicodeSentence<'text> = &'text str;
type UnicodeWord<'text> = Position<&'text str>;

impl UnicodeSegmenter {
    fn split_sentence_indices<'text>(
        &self,
        text: &'text str,
    ) -> impl Iterator<Item = (usize, UnicodeSentence<'text>)> {
        text.split_sentence_bound_indices()
            .map(|(index, sentence)| (index, sentence))
    }

    fn split_word_indices<'text>(
        &self,
        text: &'text str,
    ) -> impl Iterator<Item = (usize, UnicodeWord<'text>)> {
        self.split_sentence_indices(text)
            .flat_map(|(start, sentence)| {
                use Position::{First, Last, Middle, Only};

                sentence
                    .split_word_bound_indices()
                    .with_position()
                    .map(move |item| match item {
                        First((index, word)) => (start + index, First(word)),
                        Middle((index, word)) => (start + index, Middle(word)),
                        Last((index, word)) => (start + index, Last(word)),
                        Only((index, word)) => (start + index, Only(word)),
                    })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sentence_usage() {
        let text = "Mr. Fox jumped. [...] The dog was too lazy.";
        let sents: Vec<_> = UnicodeSegmenter::default()
            .split_sentence_indices(text)
            .collect();
        assert_eq!(
            sents,
            &[
                (0, "Mr. "),
                (4, "Fox jumped. "),
                (16, "[...] "),
                (22, "The dog was too lazy."),
            ]
        );
    }

    #[test]
    fn simple_word_usage() {
        let text = "Mr. Fox jumped. [...] The dog was too lazy.";
        let sents: Vec<_> = UnicodeSegmenter::default()
            .split_word_indices(text)
            .collect();
        assert_eq!(
            sents,
            &[
                (0, UnicodeWord::First("Mr")),
                (2, UnicodeWord::Middle(".")),
                (3, UnicodeWord::Last(" ")),
                (4, UnicodeWord::First("Fox")),
                (7, UnicodeWord::Middle(" ")),
                (8, UnicodeWord::Middle("jumped")),
                (14, UnicodeWord::Middle(".")),
                (15, UnicodeWord::Last(" ")),
                (16, UnicodeWord::First("[")),
                (17, UnicodeWord::Middle(".")),
                (18, UnicodeWord::Middle(".")),
                (19, UnicodeWord::Middle(".")),
                (20, UnicodeWord::Middle("]")),
                (21, UnicodeWord::Last(" ")),
                (22, UnicodeWord::First("The")),
                (25, UnicodeWord::Middle(" ")),
                (26, UnicodeWord::Middle("dog")),
                (29, UnicodeWord::Middle(" ")),
                (30, UnicodeWord::Middle("was")),
                (33, UnicodeWord::Middle(" ")),
                (34, UnicodeWord::Middle("too")),
                (37, UnicodeWord::Middle(" ")),
                (38, UnicodeWord::Middle("lazy")),
                (42, UnicodeWord::Last(".")),
            ]
        );
    }
}
