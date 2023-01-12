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
                use Position::*;

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
                (0, UnicodeWord::Only("Mr. ")),
                (4, UnicodeWord::Only("Fox jumped. ")),
                (16, UnicodeWord::Only("[...] ")),
                (22, UnicodeWord::Only("The dog was too lazy.")),
            ]
        );
    }
}
