use itertools::{Itertools, Position};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
struct UnicodeSegmenter {}

impl UnicodeSegmenter {
    fn split_sentence_indices<'text>(
        &self,
        text: &'text str,
    ) -> impl Iterator<Item = (usize, &'text str)> {
        text.split_sentence_bound_indices()
            .map(|(index, sentence)| (index, sentence))
    }

    fn split_word_indices<'text>(
        &self,
        text: &'text str,
    ) -> impl Iterator<Item = (usize, Position<&'text str>)> {
        use Position::{First, Last, Middle, Only};

        self.split_sentence_indices(text)
            .flat_map(|(start, sentence)| {
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

    fn split_token_indices<'text>(
        &self,
        text: &'text str,
    ) -> impl Iterator<Item = (usize, Position<UnicodeToken<'text>>)> {
        use Position::{First, Last, Middle, Only};

        self.split_word_indices(text)
            .map(|(index, item)| match item {
                First(word) => (index, First(UnicodeToken::from(word))),
                Middle(word) => (index, Middle(UnicodeToken::from(word))),
                Last(word) => (index, Last(UnicodeToken::from(word))),
                Only(word) => (index, Only(UnicodeToken::from(word))),
            })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum UnicodeToken<'text> {
    Alphabetic(&'text str),
    Numeric(&'text str),
    Whitespace(&'text str),
    Other(&'text str),
}

impl<'text> From<&'text str> for UnicodeToken<'text> {
    fn from(word: &'text str) -> Self {
        match word {
            word if word.chars().all(char::is_alphabetic) => Self::Alphabetic(word),
            word if word.chars().all(char::is_numeric) => Self::Numeric(word),
            word if word.trim().is_empty() => Self::Whitespace(word),
            word => Self::Other(word),
        }
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
        use Position::{First, Last, Middle};

        let text = "Mr. Fox jumped. [...] The dog was too lazy.";
        let sents: Vec<_> = UnicodeSegmenter::default()
            .split_word_indices(text)
            .collect();
        assert_eq!(
            sents,
            &[
                (0, First("Mr")),
                (2, Middle(".")),
                (3, Last(" ")),
                (4, First("Fox")),
                (7, Middle(" ")),
                (8, Middle("jumped")),
                (14, Middle(".")),
                (15, Last(" ")),
                (16, First("[")),
                (17, Middle(".")),
                (18, Middle(".")),
                (19, Middle(".")),
                (20, Middle("]")),
                (21, Last(" ")),
                (22, First("The")),
                (25, Middle(" ")),
                (26, Middle("dog")),
                (29, Middle(" ")),
                (30, Middle("was")),
                (33, Middle(" ")),
                (34, Middle("too")),
                (37, Middle(" ")),
                (38, Middle("lazy")),
                (42, Last(".")),
            ]
        );
    }

    #[test]
    fn simple_token_usage() {
        use Position::{First, Last, Middle};
        use UnicodeToken::{Alphabetic, Other, Whitespace};

        let text = "Mr. Fox jumped. [...] The dog was too lazy.";
        let sents: Vec<_> = UnicodeSegmenter::default()
            .split_token_indices(text)
            .collect();
        assert_eq!(
            sents,
            &[
                (0, First(Alphabetic("Mr"))),
                (2, Middle(Other("."))),
                (3, Last(Whitespace(" "))),
                (4, First(Alphabetic("Fox"))),
                (7, Middle(Whitespace(" "))),
                (8, Middle(Alphabetic("jumped"))),
                (14, Middle(Other("."))),
                (15, Last(Whitespace(" "))),
                (16, First(Other("["))),
                (17, Middle(Other("."))),
                (18, Middle(Other("."))),
                (19, Middle(Other("."))),
                (20, Middle(Other("]"))),
                (21, Last(Whitespace(" "))),
                (22, First(Alphabetic("The"))),
                (25, Middle(Whitespace(" "))),
                (26, Middle(Alphabetic("dog"))),
                (29, Middle(Whitespace(" "))),
                (30, Middle(Alphabetic("was"))),
                (33, Middle(Whitespace(" "))),
                (34, Middle(Alphabetic("too"))),
                (37, Middle(Whitespace(" "))),
                (38, Middle(Alphabetic("lazy"))),
                (42, Last(Other("."))),
            ]
        );
    }
}
