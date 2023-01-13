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
) -> impl Iterator<Item = (Position<usize>, UnicodeToken<'text>)>
where
    I: Iterator<Item = (Position<usize>, UnicodeToken<'text>)>,
{
    use Position::{First, Last, Middle, Only};

    iter.coalesce(
        |fst @ (first_position, first), snd @ (second_position, second)| match (
            first_position,
            second_position,
        ) {
            (First(first_index), Last(_)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((Only(first_index), unsafe { first.coalesce_with(second) }))
            }
            (First(first_index), Middle(_)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((First(first_index), unsafe { first.coalesce_with(second) }))
            }
            (Last(first_index), Only(_)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((Last(first_index), unsafe { first.coalesce_with(second) }))
            }
            (Middle(first_index), Last(_)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((Last(first_index), unsafe { first.coalesce_with(second) }))
            }
            (Middle(first_index), Middle(_)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((Middle(first_index), unsafe { first.coalesce_with(second) }))
            }
            (Only(first_index), First(_)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((First(first_index), unsafe { first.coalesce_with(second) }))
            }
            (Only(first_index), Only(_)) if UnicodeToken::can_merge(&first, &second) => {
                Ok((Only(first_index), unsafe { first.coalesce_with(second) }))
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
pub fn token_position_indices(text: &str) -> impl Iterator<Item = (Position<usize>, UnicodeToken)> {
    use Position::{First, Last, Middle, Only};
    use UnicodeToken::{Separator, SeparatorOrWhitespace, Whitespace};

    let iter = isolated_token_position_indices(text);
    // SAFETY: iter yields from the same string and items are adjacent.
    let iter = unsafe { coalesce_tokens(iter) };
    let iter = iter.coalesce(|fst, snd| match (fst, snd) {
        (
            (First(first_index), first),
            (Last(second_index), sep @ (Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_))),
        ) => Err(((Only(first_index), first), (Only(second_index), sep))),
        (
            (First(first_index), sep @ (Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_))),
            (Last(second_index), second),
        ) => Err(((Only(first_index), sep), (Only(second_index), second))),
        (
            (First(first_index), sep @ (Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_))),
            (Middle(second_index), second),
        ) => Err(((Only(first_index), sep), (First(second_index), second))),
        (
            (Middle(first_index), first),
            (Last(second_index), sep @ (Separator(_) | Whitespace(_) | SeparatorOrWhitespace(_))),
        ) => Err(((Last(first_index), first), (Only(second_index), sep))),
        _ => Err((fst, snd)),
    });

    // SAFETY: iter yields from the same string and items are adjacent.
    unsafe { coalesce_tokens(iter) }
}

#[inline]
fn word_position_indices(text: &str) -> impl Iterator<Item = (Position<usize>, &str)> {
    use unicode_segmentation::UnicodeSegmentation;
    use Position::{First, Last, Middle, Only};

    text.split_sentence_bound_indices()
        .flat_map(|(start, sentence)| {
            sentence
                .split_word_bound_indices()
                .with_position()
                .map(move |item| match item {
                    First((index, word)) => (First(start + index), word),
                    Middle((index, word)) => (Middle(start + index), word),
                    Last((index, word)) => (Last(start + index), word),
                    Only((index, word)) => (Only(start + index), word),
                })
        })
}

#[inline]
fn isolated_token_position_indices(
    text: &str,
) -> impl Iterator<Item = (Position<usize>, UnicodeToken)> {
    word_position_indices(text).map(|(index, word)| (index, UnicodeToken::from(word)))
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
                (First(0), "Mr"),
                (Middle(2), "."),
                (Last(3), " "),
                (First(4), "Fox"),
                (Middle(7), " "),
                (Middle(8), "jumped"),
                (Middle(14), "."),
                (Last(15), "  "),
                (First(17), "["),
                (Middle(18), "."),
                (Middle(19), "."),
                (Middle(20), "."),
                (Middle(21), "]"),
                (Last(22), "  "),
                (First(24), "The"),
                (Middle(27), " "),
                (Middle(28), "dog"),
                (Middle(31), " "),
                (Middle(32), "was"),
                (Middle(35), " "),
                (Middle(36), "too"),
                (Middle(39), " "),
                (Middle(40), "lazy"),
                (Middle(44), "."),
                (Last(45), "  "),
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
                (First(0), Letter("Mr")),
                (Middle(2), Punctuation(".")),
                (Last(3), Separator(" ")),
                (First(4), Letter("Fox")),
                (Middle(7), Separator(" ")),
                (Middle(8), Letter("jumped")),
                (Middle(14), Punctuation(".")),
                (Last(15), Whitespace("\n")),
                (First(16), Whitespace("\t")),
                (Middle(17), Punctuation("[")),
                (Middle(18), Punctuation(".")),
                (Middle(19), Punctuation(".")),
                (Middle(20), Punctuation(".")),
                (Middle(21), Punctuation("]")),
                (Last(22), Whitespace("\n")),
                (First(23), Letter("The")),
                (Middle(26), Separator(" ")),
                (Middle(27), Letter("dog")),
                (Middle(30), Separator(" ")),
                (Middle(31), Letter("had")),
                (Middle(34), Separator(" ")),
                (Middle(35), Symbol("$")),
                (Middle(36), Float("2.50")),
                (Last(40), Punctuation(".")),
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
                (First(0), Letter("Mr")),
                (Last(2), Punctuation(".")),
                (Only(3), Separator("  ")),
                (First(5), Letter("Fox")),
                (Middle(8), Separator("  ")),
                (Middle(10), Letter("jumped")),
                (Last(16), Punctuation(".")),
                (Only(17), SeparatorOrWhitespace(" \n \t ")),
                (Only(22), Punctuation("[...]")),
                (Only(27), SeparatorOrWhitespace(" \n ")),
                (First(30), Letter("The")),
                (Middle(33), Separator("  ")),
                (Middle(35), Letter("dog")),
                (Middle(38), Separator("  ")),
                (Middle(40), Letter("had")),
                (Middle(43), Separator("  ")),
                (Middle(45), Symbol("$")),
                (Middle(46), Float("2.50")),
                (Last(50), Punctuation(".")),
                (Only(51), Separator(" ")),
            ]
        );
    }
}
