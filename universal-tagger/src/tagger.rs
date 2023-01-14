use std::collections::BTreeSet;

use itertools::Position;
use unicase::UniCase;

use crate::{token_positions, Language, Token};

pub struct Tagger {
    pub lang: Language,
    stop_words: BTreeSet<UniCase<String>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tag {
    StopWord,
}

impl Tagger {
    #[must_use]
    pub fn new(lang: Language) -> Self {
        let stop_words = crate::stop_words::get(lang);
        Self { lang, stop_words }
    }

    pub fn tag<'tagger, 'text: 'tagger>(
        &'tagger self,
        text: &'text str,
    ) -> impl Iterator<Item = (Position<usize>, Token<'text>, Option<Tag>)> + 'tagger {
        token_positions(text).map(|(position, token)| (position, token, self.tag_token(token)))
    }

    fn tag_token(&self, token: Token) -> Option<Tag> {
        let s = token.as_ref();
        // TODO: detect abbreviations and/or acronyms first?
        if self.stop_words.contains(&UniCase::new(s.into())) {
            Some(Tag::StopWord)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn tag_tokens() {
        use Position::{First, Last, Middle};
        use Token::{Letter, Punctuation, Separator};

        use crate::LanguageDetector;

        let text = "Colorless green ideas sleep furiously.";
        let lang = LanguageDetector::empty()
            .allow(Language::Eng)
            .allow(Language::Cmn)
            .detect(text)
            .unwrap();
        assert_eq!(lang, Language::Eng);
        let words: Vec<_> = Tagger::new(lang).tag(text).collect();
        assert_eq!(
            words,
            &[
                (First(0), Letter("Colorless"), None),
                (Middle(9), Separator(" "), None),
                (Middle(10), Letter("green"), None),
                (Middle(15), Separator(" "), None),
                (Middle(16), Letter("ideas"), None),
                (Middle(21), Separator(" "), None),
                (Middle(22), Letter("sleep"), None),
                (Middle(27), Separator(" "), None),
                (Middle(28), Letter("furiously"), None),
                (Last(37), Punctuation("."), None),
            ]
        );
    }
}
