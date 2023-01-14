use strum::EnumIter;

/// Language codes following the [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    /// العربية (Arabic).
    #[cfg(feature = "arabic")]
    Ara,
    /// বাংলা (Bengali).
    #[cfg(feature = "bengali")]
    Ben,
    /// 普通话 (Mandarin).
    #[cfg(feature = "mandarin")]
    Cmn,
    /// Deutsch (German).
    #[cfg(feature = "german")]
    Deu,
    /// English (English).
    #[cfg(feature = "english")]
    Eng,
    /// Esperanto (Esperanto).
    #[cfg(feature = "esperanto")]
    Epo,
    /// Français (French).
    #[cfg(feature = "french")]
    Fra,
    /// हिन्दी (Hindi).
    #[cfg(feature = "hindi")]
    Hin,
    /// Bahasa Indonesia (Indonesian).
    #[cfg(feature = "indonesian")]
    Ind,
    /// Italiano (Italian).
    #[cfg(feature = "italian")]
    Ita,
    /// 日本語 (Japanese).
    #[cfg(feature = "japanese")]
    Jpn,
    /// Português (Portuguese).
    #[cfg(feature = "portuguese")]
    Por,
    /// Русский (Russian).
    #[cfg(feature = "russian")]
    Rus,
    /// Español (Spanish).
    #[cfg(feature = "spanish")]
    Spa,
    /// Türkçe (Turkish).
    #[cfg(feature = "turkish")]
    Tur,
    /// اُردُو (Urdu).
    #[cfg(feature = "urdu")]
    Urd,
}

impl From<Language> for stop_words::LANGUAGE {
    #[inline]
    fn from(lang: Language) -> Self {
        match lang {
            #[cfg(feature = "arabic")]
            Language::Ara => Self::Arabic,
            #[cfg(feature = "bengali")]
            Language::Ben => Self::Bengali,
            #[cfg(feature = "mandarin")]
            Language::Cmn => Self::Chinese,
            #[cfg(feature = "german")]
            Language::Deu => Self::German,
            #[cfg(feature = "english")]
            Language::Eng => Self::English,
            #[cfg(feature = "esperanto")]
            Language::Epo => Self::Esperanto,
            #[cfg(feature = "french")]
            Language::Fra => Self::French,
            #[cfg(feature = "hindi")]
            Language::Hin => Self::Hindi,
            #[cfg(feature = "indonesian")]
            Language::Ind => Self::Indonesian,
            #[cfg(feature = "italian")]
            Language::Ita => Self::Italian,
            #[cfg(feature = "japanese")]
            Language::Jpn => Self::Japanese,
            #[cfg(feature = "portuguese")]
            Language::Por => Self::Portuguese,
            #[cfg(feature = "russian")]
            Language::Rus => Self::Russian,
            #[cfg(feature = "spanish")]
            Language::Spa => Self::Spanish,
            #[cfg(feature = "turkish")]
            Language::Tur => Self::Turkish,
            #[cfg(feature = "urdu")]
            Language::Urd => Self::Urdu,
        }
    }
}

#[cfg(feature = "whatlang")]
impl From<Language> for whatlang::Lang {
    #[inline]
    fn from(lang: Language) -> Self {
        match lang {
            #[cfg(feature = "arabic")]
            Language::Ara => Self::Ara,
            #[cfg(feature = "bengali")]
            Language::Ben => Self::Ben,
            #[cfg(feature = "mandarin")]
            Language::Cmn => Self::Cmn,
            #[cfg(feature = "german")]
            Language::Deu => Self::Deu,
            #[cfg(feature = "english")]
            Language::Eng => Self::Eng,
            #[cfg(feature = "esperanto")]
            Language::Epo => Self::Epo,
            #[cfg(feature = "french")]
            Language::Fra => Self::Fra,
            #[cfg(feature = "hindi")]
            Language::Hin => Self::Hin,
            #[cfg(feature = "indonesian")]
            Language::Ind => Self::Ind,
            #[cfg(feature = "italian")]
            Language::Ita => Self::Ita,
            #[cfg(feature = "japanese")]
            Language::Jpn => Self::Jpn,
            #[cfg(feature = "portuguese")]
            Language::Por => Self::Por,
            #[cfg(feature = "russian")]
            Language::Rus => Self::Rus,
            #[cfg(feature = "spanish")]
            Language::Spa => Self::Spa,
            #[cfg(feature = "turkish")]
            Language::Tur => Self::Tur,
            #[cfg(feature = "urdu")]
            Language::Urd => Self::Urd,
        }
    }
}

#[cfg(feature = "lingua")]
impl From<Language> for lingua::Language {
    #[inline]
    fn from(lang: Language) -> Self {
        match lang {
            #[cfg(feature = "arabic")]
            Language::Ara => Self::Arabic,
            #[cfg(feature = "bengali")]
            Language::Ben => Self::Bengali,
            #[cfg(feature = "mandarin")]
            Language::Cmn => Self::Chinese,
            #[cfg(feature = "german")]
            Language::Deu => Self::German,
            #[cfg(feature = "english")]
            Language::Eng => Self::English,
            #[cfg(feature = "esperanto")]
            Language::Epo => Self::Esperanto,
            #[cfg(feature = "french")]
            Language::Fra => Self::French,
            #[cfg(feature = "hindi")]
            Language::Hin => Self::Hindi,
            #[cfg(feature = "indonesian")]
            Language::Ind => Self::Indonesian,
            #[cfg(feature = "italian")]
            Language::Ita => Self::Italian,
            #[cfg(feature = "japanese")]
            Language::Jpn => Self::Japanese,
            #[cfg(feature = "portuguese")]
            Language::Por => Self::Portuguese,
            #[cfg(feature = "russian")]
            Language::Rus => Self::Russian,
            #[cfg(feature = "spanish")]
            Language::Spa => Self::Spanish,
            #[cfg(feature = "turkish")]
            Language::Tur => Self::Turkish,
            #[cfg(feature = "urdu")]
            Language::Urd => Self::Urdu,
        }
    }
}

impl TryFrom<stop_words::LANGUAGE> for Language {
    type Error = stop_words::LANGUAGE;

    #[inline]
    fn try_from(lang: stop_words::LANGUAGE) -> Result<Self, Self::Error> {
        match lang {
            #[cfg(feature = "arabic")]
            stop_words::LANGUAGE::Arabic => Ok(Self::Ara),
            #[cfg(feature = "bengali")]
            stop_words::LANGUAGE::Bengali => Ok(Self::Ben),
            #[cfg(feature = "mandarin")]
            stop_words::LANGUAGE::Chinese => Ok(Self::Cmn),
            #[cfg(feature = "german")]
            stop_words::LANGUAGE::German => Ok(Self::Deu),
            #[cfg(feature = "english")]
            stop_words::LANGUAGE::English => Ok(Self::Eng),
            #[cfg(feature = "esperanto")]
            stop_words::LANGUAGE::Esperanto => Ok(Self::Epo),
            #[cfg(feature = "french")]
            stop_words::LANGUAGE::French => Ok(Self::Fra),
            #[cfg(feature = "hindi")]
            stop_words::LANGUAGE::Hindi => Ok(Self::Hin),
            #[cfg(feature = "indonesian")]
            stop_words::LANGUAGE::Indonesian => Ok(Self::Ind),
            #[cfg(feature = "italian")]
            stop_words::LANGUAGE::Italian => Ok(Self::Ita),
            #[cfg(feature = "japanese")]
            stop_words::LANGUAGE::Japanese => Ok(Self::Jpn),
            #[cfg(feature = "portuguese")]
            stop_words::LANGUAGE::Portuguese => Ok(Self::Por),
            #[cfg(feature = "russian")]
            stop_words::LANGUAGE::Russian => Ok(Self::Rus),
            #[cfg(feature = "spanish")]
            stop_words::LANGUAGE::Spanish => Ok(Self::Spa),
            #[cfg(feature = "turkish")]
            stop_words::LANGUAGE::Turkish => Ok(Self::Tur),
            #[cfg(feature = "urdu")]
            stop_words::LANGUAGE::Urdu => Ok(Self::Urd),
            lang => Err(lang),
        }
    }
}

#[cfg(feature = "whatlang")]
impl TryFrom<whatlang::Lang> for Language {
    type Error = whatlang::Lang;

    #[inline]
    fn try_from(lang: whatlang::Lang) -> Result<Self, Self::Error> {
        match lang {
            #[cfg(feature = "arabic")]
            whatlang::Lang::Ara => Ok(Self::Ara),
            #[cfg(feature = "bengali")]
            whatlang::Lang::Ben => Ok(Self::Ben),
            #[cfg(feature = "mandarin")]
            whatlang::Lang::Cmn => Ok(Self::Cmn),
            #[cfg(feature = "german")]
            whatlang::Lang::Deu => Ok(Self::Deu),
            #[cfg(feature = "english")]
            whatlang::Lang::Eng => Ok(Self::Eng),
            #[cfg(feature = "esperanto")]
            whatlang::Lang::Epo => Ok(Self::Epo),
            #[cfg(feature = "french")]
            whatlang::Lang::Fra => Ok(Self::Fra),
            #[cfg(feature = "hindi")]
            whatlang::Lang::Hin => Ok(Self::Hin),
            #[cfg(feature = "indonesian")]
            whatlang::Lang::Ind => Ok(Self::Ind),
            #[cfg(feature = "italian")]
            whatlang::Lang::Ita => Ok(Self::Ita),
            #[cfg(feature = "japanese")]
            whatlang::Lang::Jpn => Ok(Self::Jpn),
            #[cfg(feature = "portuguese")]
            whatlang::Lang::Por => Ok(Self::Por),
            #[cfg(feature = "russian")]
            whatlang::Lang::Rus => Ok(Self::Rus),
            #[cfg(feature = "spanish")]
            whatlang::Lang::Spa => Ok(Self::Spa),
            #[cfg(feature = "turkish")]
            whatlang::Lang::Tur => Ok(Self::Tur),
            #[cfg(feature = "urdu")]
            whatlang::Lang::Urd => Ok(Self::Urd),
            lang => Err(lang),
        }
    }
}

#[cfg(feature = "lingua")]
impl TryFrom<lingua::Language> for Language {
    type Error = lingua::Language;

    #[inline]
    fn try_from(lang: lingua::Language) -> Result<Self, Self::Error> {
        match lang {
            #[cfg(feature = "arabic")]
            lingua::Language::Arabic => Ok(Self::Ara),
            #[cfg(feature = "bengali")]
            lingua::Language::Bengali => Ok(Self::Ben),
            #[cfg(feature = "mandarin")]
            lingua::Language::Chinese => Ok(Self::Cmn),
            #[cfg(feature = "german")]
            lingua::Language::German => Ok(Self::Deu),
            #[cfg(feature = "english")]
            lingua::Language::English => Ok(Self::Eng),
            #[cfg(feature = "esperanto")]
            lingua::Language::Esperanto => Ok(Self::Epo),
            #[cfg(feature = "french")]
            lingua::Language::French => Ok(Self::Fra),
            #[cfg(feature = "hindi")]
            lingua::Language::Hindi => Ok(Self::Hin),
            #[cfg(feature = "indonesian")]
            lingua::Language::Indonesian => Ok(Self::Ind),
            #[cfg(feature = "italian")]
            lingua::Language::Italian => Ok(Self::Ita),
            #[cfg(feature = "japanese")]
            lingua::Language::Japanese => Ok(Self::Jpn),
            #[cfg(feature = "portuguese")]
            lingua::Language::Portuguese => Ok(Self::Por),
            #[cfg(feature = "russian")]
            lingua::Language::Russian => Ok(Self::Rus),
            #[cfg(feature = "spanish")]
            lingua::Language::Spanish => Ok(Self::Spa),
            #[cfg(feature = "turkish")]
            lingua::Language::Turkish => Ok(Self::Tur),
            #[cfg(feature = "urdu")]
            lingua::Language::Urdu => Ok(Self::Urd),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn conversion_roundtrip() {
        use strum::IntoEnumIterator;

        for lang in Language::iter() {
            assert_eq!(
                Ok(lang),
                stop_words::LANGUAGE::from(lang)
                    .try_into()
                    // NOTE: stop_words::LANGUAGE doesn't implement PartialEq or Debug
                    .map_err(|err: stop_words::LANGUAGE| err.to_string())
            );

            #[cfg(feature = "whatlang")]
            assert_eq!(Ok(lang), whatlang::Lang::from(lang).try_into());

            #[cfg(feature = "lingua")]
            assert_eq!(Ok(lang), lingua::Language::from(lang).try_into());
        }
    }
}
