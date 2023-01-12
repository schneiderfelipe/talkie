use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
struct UnicodeTokenizer {}

#[derive(Clone, Copy, Debug, PartialEq)]
enum UnicodeToken<'text> {
    Sentence(&'text str),
    EndOfSentence,
}

impl UnicodeTokenizer {
    fn split_sentence_indices<'text>(
        &self,
        text: &'text str,
    ) -> impl Iterator<Item = (usize, UnicodeToken<'text>)> {
        text.split_sentence_bound_indices()
            .flat_map(|(index, sentence)| {
                [
                    (index, UnicodeToken::Sentence(sentence)),
                    (index + sentence.len(), UnicodeToken::EndOfSentence),
                ]
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_usage() {
        let text = "Mr. Fox jumped. [...] The dog was too lazy.";
        let sents: Vec<_> = UnicodeTokenizer::default()
            .split_sentence_indices(text)
            .collect();
        assert_eq!(
            sents,
            &[
                (0, UnicodeToken::Sentence("Mr. ")),
                (4, UnicodeToken::EndOfSentence),
                (4, UnicodeToken::Sentence("Fox jumped. ")),
                (16, UnicodeToken::EndOfSentence),
                (16, UnicodeToken::Sentence("[...] ")),
                (22, UnicodeToken::EndOfSentence),
                (22, UnicodeToken::Sentence("The dog was too lazy.")),
                (43, UnicodeToken::EndOfSentence),
            ]
        );
    }
}
