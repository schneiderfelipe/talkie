use itertools::{Itertools, Position};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnicodeToken<'text> {
    Separator(&'text str),
    Whitespace(&'text str),
    SeparatorOrWhitespace(&'text str),
    Letter(&'text str),
    Punctuation(&'text str),
    Number(&'text str),
    Float(&'text str),
    Symbol(&'text str),
    Mark(&'text str),
    LetterOrPunctuation(&'text str),
    NumberOrPunctuation(&'text str),
    LetterOrNumber(&'text str),
    LetterOrMark(&'text str),
    NumberOrMark(&'text str),
    Other(&'text str),
    LetterOrOther(&'text str),
    NumberOrOther(&'text str),
    OtherOrPunctuation(&'text str),
}

impl<'text> From<&'text str> for UnicodeToken<'text> {
    #[inline]
    fn from(word: &'text str) -> Self {
        use unicode_categories::UnicodeCategories;

        match word {
            word if word.chars().all(char::is_separator) => Self::Separator(word),
            word if word.chars().all(|c| c.is_whitespace() && !c.is_separator()) => {
                Self::Whitespace(word)
            }
            word if word.chars().all(|c| c.is_whitespace() || c.is_separator()) => {
                Self::SeparatorOrWhitespace(word)
            }
            // TODO: do we need alphabetic?
            word if word.chars().all(char::is_letter) => Self::Letter(word),
            word if word.chars().all(char::is_punctuation) => Self::Punctuation(word),
            // TODO: do we need numeric?
            word if word.chars().all(char::is_number) => Self::Number(word),
            word if word.parse::<f64>().is_ok() => Self::Float(word),
            word if word.chars().all(char::is_symbol) => Self::Symbol(word),
            word if word.chars().all(char::is_mark) => Self::Mark(word),
            word if word.chars().all(|c| c.is_letter() || c.is_punctuation()) => {
                Self::LetterOrPunctuation(word)
            }
            word if word.chars().all(|c| c.is_number() || c.is_punctuation()) => {
                Self::NumberOrPunctuation(word)
            }
            // TODO: do we need alphanumeric?
            word if word.chars().all(|c| c.is_letter() || c.is_number()) => {
                Self::LetterOrNumber(word)
            }
            word if word.chars().all(|c| c.is_letter() || c.is_mark()) => Self::LetterOrMark(word),
            word if word.chars().all(|c| c.is_number() || c.is_mark()) => Self::NumberOrMark(word),
            word if word.chars().all(char::is_other) => Self::Other(word),
            word if word.chars().all(|c| c.is_letter() || c.is_other()) => {
                Self::LetterOrOther(word)
            }
            word if word.chars().all(|c| c.is_number() || c.is_other()) => {
                Self::NumberOrOther(word)
            }
            word if word.chars().all(|c| c.is_punctuation() || c.is_other()) => {
                Self::OtherOrPunctuation(word)
            }
            word => unreachable!("possibly missing case: {word:?}"),
        }
    }
}

impl<'text> AsRef<str> for UnicodeToken<'text> {
    #[inline]
    fn as_ref(&self) -> &str {
        use UnicodeToken::{
            Float, Letter, LetterOrMark, LetterOrNumber, LetterOrOther, LetterOrPunctuation, Mark,
            Number, NumberOrMark, NumberOrOther, NumberOrPunctuation, Other, OtherOrPunctuation,
            Punctuation, Separator, SeparatorOrWhitespace, Symbol, Whitespace,
        };

        match self {
            Separator(word)
            | Whitespace(word)
            | SeparatorOrWhitespace(word)
            | Letter(word)
            | Punctuation(word)
            | Number(word)
            | Float(word)
            | Symbol(word)
            | Mark(word)
            | LetterOrPunctuation(word)
            | NumberOrPunctuation(word)
            | LetterOrNumber(word)
            | LetterOrMark(word)
            | NumberOrMark(word)
            | Other(word)
            | LetterOrOther(word)
            | NumberOrOther(word)
            | OtherOrPunctuation(word) => word,
        }
    }
}

impl<'text> UnicodeToken<'text> {
    #[inline]
    const fn can_merge(first: &Self, second: &Self) -> bool {
        use UnicodeToken::{
            Float, Letter, LetterOrMark, LetterOrNumber, LetterOrOther, LetterOrPunctuation, Mark,
            Number, NumberOrMark, NumberOrOther, NumberOrPunctuation, Other, OtherOrPunctuation,
            Punctuation, Separator, SeparatorOrWhitespace, Symbol, Whitespace,
        };

        matches!(
            (first, second),
            (
                Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_),
                Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_)
            ) | (Letter(_), Letter(_))
                | (Punctuation(_), Punctuation(_))
                | (Number(_), Number(_))
                | (Float(_), Float(_))
                | (Symbol(_), Symbol(_))
                | (Mark(_), Mark(_))
                | (LetterOrPunctuation(_), LetterOrPunctuation(_))
                | (NumberOrPunctuation(_), NumberOrPunctuation(_))
                | (LetterOrNumber(_), LetterOrNumber(_))
                | (LetterOrMark(_), LetterOrMark(_))
                | (NumberOrMark(_), NumberOrMark(_))
                | (Other(_), Other(_))
                | (LetterOrOther(_), LetterOrOther(_))
                | (NumberOrOther(_), NumberOrOther(_))
                | (OtherOrPunctuation(_), OtherOrPunctuation(_))
        )
    }

    /// Coalesce two tokens.
    ///
    /// # Safety
    ///
    /// The pointers have to be from the same string and be next to each other
    /// in the original string.
    #[inline]
    unsafe fn coalesce_with(self, other: Self) -> Self {
        use std::{slice, str};

        let (first, second) = (self.as_ref(), other.as_ref());

        let ptr = first.as_ptr();
        let len = first.len() + second.len();

        let utf8 = unsafe {
            let slice = slice::from_raw_parts(ptr, len);
            str::from_utf8_unchecked(slice)
        };

        utf8.into()
    }
}

/// Coalesce adjacent tokens.
///
/// # Safety
///
/// The whole iterator should yield pointers from the same string,
/// and adjacent yielded items should be next to each other
/// in the original string.
#[inline]
unsafe fn coalesce_tokens<'text, I>(
    iter: I,
) -> impl Iterator<Item = (usize, Position<UnicodeToken<'text>>)>
where
    I: Iterator<Item = (usize, Position<UnicodeToken<'text>>)>,
{
    use Position::{First, Last, Middle, Only};

    iter.coalesce(
        |fst @ (first_index, first), snd @ (_, second)| match (first, second) {
            (First(first), Last(second)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((first_index, Only(unsafe { first.coalesce_with(second) })))
            }
            (First(first), Middle(second)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((first_index, First(unsafe { first.coalesce_with(second) })))
            }
            (Last(first), Only(second)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((first_index, Last(unsafe { first.coalesce_with(second) })))
            }
            (Middle(first), Last(second)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((first_index, Last(unsafe { first.coalesce_with(second) })))
            }
            (Middle(first), Middle(second)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((first_index, Middle(unsafe { first.coalesce_with(second) })))
            }
            (Only(first), First(second)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((first_index, First(unsafe { first.coalesce_with(second) })))
            }
            (Only(first), Only(second)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((first_index, Only(unsafe { first.coalesce_with(second) })))
            }
            (First(_) | Middle(_), First(_) | Only(_))
            | (Last(_) | Only(_), Last(_) | Middle(_)) => {
                unreachable!("possibly missing case: ({first:?}, {second:?})")
            }
            _ => Err((fst, snd)),
        },
    )
}

#[inline]
pub fn token_position_indices(text: &str) -> impl Iterator<Item = (usize, Position<UnicodeToken>)> {
    use Position::{First, Last, Middle, Only};
    use UnicodeToken::{Separator, SeparatorOrWhitespace, Whitespace};

    let iter = isolated_token_position_indices(text);
    // SAFETY: iter yields from the same string and items are adjacent.
    let iter = unsafe { coalesce_tokens(iter) };
    let iter = iter.coalesce(|fst @ (first_index, first), snd @ (second_index, second)| {
        match (first, second) {
            (
                First(first),
                Last(sep @ (Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_))),
            ) => Err(((first_index, Only(first)), (second_index, Only(sep)))),
            (
                First(sep @ (Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_))),
                Last(second),
            ) => Err(((first_index, Only(sep)), (second_index, Only(second)))),
            (
                First(sep @ (Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_))),
                Middle(second),
            ) => Err(((first_index, Only(sep)), (second_index, First(second)))),
            (
                Middle(first),
                Last(sep @ (Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_))),
            ) => Err(((first_index, Last(first)), (second_index, Only(sep)))),
            _ => Err((fst, snd)),
        }
    });

    // SAFETY: iter yields from the same string and items are adjacent.
    unsafe { coalesce_tokens(iter) }
}

#[inline]
fn word_position_indices(text: &str) -> impl Iterator<Item = (usize, Position<&str>)> {
    use unicode_segmentation::UnicodeSegmentation;
    use Position::{First, Last, Middle, Only};

    text.split_sentence_bound_indices()
        .flat_map(|(start, sentence)| {
            sentence
                .split_word_bound_indices()
                .with_position()
                .map(move |item| {
                    let (index, word) = match item {
                        First((index, word)) => (index, First(word)),
                        Middle((index, word)) => (index, Middle(word)),
                        Last((index, word)) => (index, Last(word)),
                        Only((index, word)) => (index, Only(word)),
                    };
                    (start + index, word)
                })
        })
}

#[inline]
fn isolated_token_position_indices(
    text: &str,
) -> impl Iterator<Item = (usize, Position<UnicodeToken>)> {
    use Position::{First, Last, Middle, Only};

    word_position_indices(text).map(|(index, item)| {
        let item = match item {
            First(word) => First(UnicodeToken::from(word)),
            Middle(word) => Middle(UnicodeToken::from(word)),
            Last(word) => Last(UnicodeToken::from(word)),
            Only(word) => Only(UnicodeToken::from(word)),
        };
        (index, item)
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn simple_word_usage() {
        use Position::{First, Last, Middle};

        let text = "Mr. Fox jumped.  [...]  The dog was too lazy.  ";
        let sents: Vec<_> = word_position_indices(text).collect();
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
                (15, Last("  ")),
                (17, First("[")),
                (18, Middle(".")),
                (19, Middle(".")),
                (20, Middle(".")),
                (21, Middle("]")),
                (22, Last("  ")),
                (24, First("The")),
                (27, Middle(" ")),
                (28, Middle("dog")),
                (31, Middle(" ")),
                (32, Middle("was")),
                (35, Middle(" ")),
                (36, Middle("too")),
                (39, Middle(" ")),
                (40, Middle("lazy")),
                (44, Middle(".")),
                (45, Last("  ")),
            ]
        );
    }

    #[test]
    fn simple_isolated_token_usage() {
        use Position::{First, Last, Middle};
        use UnicodeToken::{Float, Letter, Punctuation, Separator, Symbol, Whitespace};

        let text = "Mr. Fox jumped.\n\t[...]\nThe dog had $2.50.";
        let sents: Vec<_> = isolated_token_position_indices(text).collect();
        assert_eq!(
            sents,
            &[
                (0, First(Letter("Mr"))),
                (2, Middle(Punctuation("."))),
                (3, Last(Separator(" "))),
                (4, First(Letter("Fox"))),
                (7, Middle(Separator(" "))),
                (8, Middle(Letter("jumped"))),
                (14, Middle(Punctuation("."))),
                (15, Last(Whitespace("\n"))),
                (16, First(Whitespace("\t"))),
                (17, Middle(Punctuation("["))),
                (18, Middle(Punctuation("."))),
                (19, Middle(Punctuation("."))),
                (20, Middle(Punctuation("."))),
                (21, Middle(Punctuation("]"))),
                (22, Last(Whitespace("\n"))),
                (23, First(Letter("The"))),
                (26, Middle(Separator(" "))),
                (27, Middle(Letter("dog"))),
                (30, Middle(Separator(" "))),
                (31, Middle(Letter("had"))),
                (34, Middle(Separator(" "))),
                (35, Middle(Symbol("$"))),
                (36, Middle(Float("2.50"))),
                (40, Last(Punctuation("."))),
            ]
        );
    }

    #[test]
    fn simple_token_usage() {
        use Position::{First, Last, Middle, Only};
        use UnicodeToken::{Float, Letter, Punctuation, Separator, SeparatorOrWhitespace, Symbol};

        let text = "Mr.  Fox  jumped. \n \t [...] \n The  dog  had  $2.50. ";
        let sents: Vec<_> = token_position_indices(text).collect();
        assert_eq!(
            sents,
            &[
                (0, First(Letter("Mr"))),
                (2, Last(Punctuation("."))),
                (3, Only(Separator("  "))),
                (5, First(Letter("Fox"))),
                (8, Middle(Separator("  "))),
                (10, Middle(Letter("jumped"))),
                (16, Last(Punctuation("."))),
                (17, Only(SeparatorOrWhitespace(" \n \t "))),
                (22, Only(Punctuation("[...]"))),
                (27, Only(SeparatorOrWhitespace(" \n "))),
                (30, First(Letter("The"))),
                (33, Middle(Separator("  "))),
                (35, Middle(Letter("dog"))),
                (38, Middle(Separator("  "))),
                (40, Middle(Letter("had"))),
                (43, Middle(Separator("  "))),
                (45, Middle(Symbol("$"))),
                (46, Middle(Float("2.50"))),
                (50, Last(Punctuation("."))),
                (51, Only(Separator(" "))),
            ]
        );
    }
}
