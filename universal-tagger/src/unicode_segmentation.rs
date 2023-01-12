use itertools::{Itertools, Position};
use unicode_segmentation::UnicodeSegmentation;

#[inline]
fn word_position_indices(text: &str) -> impl Iterator<Item = (usize, Position<&str>)> {
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

#[inline]
pub fn token_position_indices(text: &str) -> impl Iterator<Item = (usize, Position<UnicodeToken>)> {
    use Position::{First, Last, Middle, Only};
    use UnicodeToken::Whitespace;

    let iter = coalesce_tokens(isolated_token_position_indices(text)).coalesce(
        |fst @ (first_index, first), snd @ (second_index, second)| match (first, second) {
            (First(first), Last(whitespace @ Whitespace(_))) => {
                Err(((first_index, Only(first)), (second_index, Only(whitespace))))
            }
            (First(whitespace @ Whitespace(_)), Last(second)) => Err((
                (first_index, Only(whitespace)),
                (second_index, Only(second)),
            )),
            (First(whitespace @ Whitespace(_)), Middle(second)) => Err((
                (first_index, Only(whitespace)),
                (second_index, First(second)),
            )),
            (Middle(first), Last(whitespace @ Whitespace(_))) => {
                Err(((first_index, Last(first)), (second_index, Only(whitespace))))
            }
            _ => Err((fst, snd)),
        },
    );

    coalesce_tokens(iter)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnicodeToken<'text> {
    Whitespace(&'text str),
    Alphabetic(&'text str),
    Numeric(&'text str),
    Other(&'text str),
}

impl<'text> From<&'text str> for UnicodeToken<'text> {
    #[inline]
    fn from(word: &'text str) -> Self {
        match word {
            word if word.trim().is_empty() => Self::Whitespace(word),
            word if word.chars().all(char::is_alphabetic) => Self::Alphabetic(word),
            word if word.chars().all(char::is_numeric) || word.parse::<f64>().is_ok() => {
                Self::Numeric(word)
            }
            word => Self::Other(word),
        }
    }
}

impl<'text> AsRef<str> for UnicodeToken<'text> {
    #[inline]
    fn as_ref(&self) -> &str {
        use UnicodeToken::{Alphabetic, Numeric, Other, Whitespace};

        match self {
            Whitespace(word) | Alphabetic(word) | Numeric(word) | Other(word) => word,
        }
    }
}

impl<'text> UnicodeToken<'text> {
    #[inline]
    const fn same_kind(first: &Self, second: &Self) -> bool {
        use UnicodeToken::{Alphabetic, Numeric, Other, Whitespace};

        matches!(
            (first, second),
            (Whitespace(_), Whitespace(_))
                | (Alphabetic(_), Alphabetic(_))
                | (Numeric(_), Numeric(_))
                | (Other(_), Other(_))
        )
    }

    /// Coalesce two tokens.
    ///
    /// They have to be from the same string, be of the same kind, and be next to each other in the string.
    #[inline]
    unsafe fn coalesce_with(self, other: Self) -> Self {
        use std::{slice, str};

        let (first, second) = (self.as_ref(), other.as_ref());

        let ptr = first.as_ptr();
        let len = first.len() + second.len();

        let slice = slice::from_raw_parts(ptr, len);
        let utf8 = str::from_utf8_unchecked(slice);

        utf8.into()
    }
}

#[inline]
fn coalesce_tokens<'text, I>(
    iter: I,
) -> impl Iterator<Item = (usize, Position<UnicodeToken<'text>>)>
where
    I: Iterator<Item = (usize, Position<UnicodeToken<'text>>)>,
{
    use Position::{First, Last, Middle, Only};

    iter.coalesce(
        |fst @ (first_index, first), snd @ (_, second)| match (first, second) {
            (First(first), Last(second)) if UnicodeToken::same_kind(&first, &second) => {
                Ok((first_index, Only(unsafe { first.coalesce_with(second) })))
            }
            (First(first), Middle(second)) if UnicodeToken::same_kind(&first, &second) => {
                Ok((first_index, First(unsafe { first.coalesce_with(second) })))
            }
            (Last(first), Only(second)) if UnicodeToken::same_kind(&first, &second) => {
                Ok((first_index, Last(unsafe { first.coalesce_with(second) })))
            }
            (Middle(first), Last(second)) if UnicodeToken::same_kind(&first, &second) => {
                Ok((first_index, Last(unsafe { first.coalesce_with(second) })))
            }
            (Middle(first), Middle(second)) if UnicodeToken::same_kind(&first, &second) => {
                Ok((first_index, Middle(unsafe { first.coalesce_with(second) })))
            }
            (Only(first), First(second)) if UnicodeToken::same_kind(&first, &second) => {
                Ok((first_index, First(unsafe { first.coalesce_with(second) })))
            }
            (Only(first), Only(second)) if UnicodeToken::same_kind(&first, &second) => {
                Ok((first_index, Only(unsafe { first.coalesce_with(second) })))
            }
            (First(_) | Middle(_), First(_) | Only(_))
            | (Last(_) | Only(_), Last(_) | Middle(_)) => {
                unreachable!("impossible case: ({first:?}, {second:?})")
            }
            _ => Err((fst, snd)),
        },
    )
}

#[cfg(test)]
mod tests {
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
        use UnicodeToken::{Alphabetic, Numeric, Other, Whitespace};

        let text = "Mr. Fox jumped.\n\t[...]\nThe dog had $2.50.";
        let sents: Vec<_> = isolated_token_position_indices(text).collect();
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
                (15, Last(Whitespace("\n"))),
                (16, First(Whitespace("\t"))),
                (17, Middle(Other("["))),
                (18, Middle(Other("."))),
                (19, Middle(Other("."))),
                (20, Middle(Other("."))),
                (21, Middle(Other("]"))),
                (22, Last(Whitespace("\n"))),
                (23, First(Alphabetic("The"))),
                (26, Middle(Whitespace(" "))),
                (27, Middle(Alphabetic("dog"))),
                (30, Middle(Whitespace(" "))),
                (31, Middle(Alphabetic("had"))),
                (34, Middle(Whitespace(" "))),
                (35, Middle(Other("$"))),
                (36, Middle(Numeric("2.50"))),
                (40, Last(Other("."))),
            ]
        );
    }

    #[test]
    fn simple_token_usage() {
        use Position::{First, Last, Middle, Only};
        use UnicodeToken::{Alphabetic, Numeric, Other, Whitespace};

        let text = "Mr.  Fox  jumped. \n \t [...] \n The  dog  had  $2.50. ";
        let sents: Vec<_> = token_position_indices(text).collect();
        assert_eq!(
            sents,
            &[
                (0, First(Alphabetic("Mr"))),
                (2, Last(Other("."))),
                (3, Only(Whitespace("  "))),
                (5, First(Alphabetic("Fox"))),
                (8, Middle(Whitespace("  "))),
                (10, Middle(Alphabetic("jumped"))),
                (16, Last(Other("."))),
                (17, Only(Whitespace(" \n \t "))),
                (22, Only(Other("[...]"))),
                (27, Only(Whitespace(" \n "))),
                (30, First(Alphabetic("The"))),
                (33, Middle(Whitespace("  "))),
                (35, Middle(Alphabetic("dog"))),
                (38, Middle(Whitespace("  "))),
                (40, Middle(Alphabetic("had"))),
                (43, Middle(Whitespace("  "))),
                (45, Middle(Other("$"))),
                (46, Middle(Numeric("2.50"))),
                (50, Last(Other("."))),
                (51, Only(Whitespace(" "))),
            ]
        );
    }
}
