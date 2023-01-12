use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
struct UnicodeSegmenter {}

#[derive(Clone, Copy, Debug, PartialEq)]
enum UnicodeSentenceSegment<'text> {
    Sentence(&'text str),
    EndOfSentence,
}

impl UnicodeSegmenter {
    fn split_sentence_indices<'text>(
        &self,
        text: &'text str,
    ) -> impl Iterator<Item = (usize, UnicodeSentenceSegment<'text>)> {
        text.split_sentence_bound_indices()
            .flat_map(|(index, sentence)| {
                [
                    (index, UnicodeSentenceSegment::Sentence(sentence)),
                    (
                        index + sentence.len(),
                        UnicodeSentenceSegment::EndOfSentence,
                    ),
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
        let sents: Vec<_> = UnicodeSegmenter::default()
            .split_sentence_indices(text)
            .collect();
        assert_eq!(
            sents,
            &[
                (0, UnicodeSentenceSegment::Sentence("Mr. ")),
                (4, UnicodeSentenceSegment::EndOfSentence),
                (4, UnicodeSentenceSegment::Sentence("Fox jumped. ")),
                (16, UnicodeSentenceSegment::EndOfSentence),
                (16, UnicodeSentenceSegment::Sentence("[...] ")),
                (22, UnicodeSentenceSegment::EndOfSentence),
                (
                    22,
                    UnicodeSentenceSegment::Sentence("The dog was too lazy.")
                ),
                (43, UnicodeSentenceSegment::EndOfSentence),
            ]
        );
    }
}
