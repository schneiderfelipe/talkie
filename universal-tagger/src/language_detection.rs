use std::collections::BTreeSet;

use cfg_if::cfg_if;
use strum::EnumIter;

/// Language codes following the [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
enum Lang {
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
    /// ਪੰਜਾਬੀ (Punjabi).
    #[cfg(feature = "punjabi")]
    Pan,
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

#[cfg(feature = "whatlang")]
impl From<Lang> for whatlang::Lang {
    fn from(lang: Lang) -> Self {
        match lang {
            #[cfg(feature = "arabic")]
            Lang::Ara => Self::Ara,
            #[cfg(feature = "bengali")]
            Lang::Ben => Self::Ben,
            #[cfg(feature = "mandarin")]
            Lang::Cmn => Self::Cmn,
            #[cfg(feature = "german")]
            Lang::Deu => Self::Deu,
            #[cfg(feature = "english")]
            Lang::Eng => Self::Eng,
            #[cfg(feature = "esperanto")]
            Lang::Epo => Self::Epo,
            #[cfg(feature = "french")]
            Lang::Fra => Self::Fra,
            #[cfg(feature = "hindi")]
            Lang::Hin => Self::Hin,
            #[cfg(feature = "indonesian")]
            Lang::Ind => Self::Ind,
            #[cfg(feature = "italian")]
            Lang::Ita => Self::Ita,
            #[cfg(feature = "japanese")]
            Lang::Jpn => Self::Jpn,
            #[cfg(feature = "punjabi")]
            Lang::Pan => Self::Pan,
            #[cfg(feature = "portuguese")]
            Lang::Por => Self::Por,
            #[cfg(feature = "russian")]
            Lang::Rus => Self::Rus,
            #[cfg(feature = "spanish")]
            Lang::Spa => Self::Spa,
            #[cfg(feature = "turkish")]
            Lang::Tur => Self::Tur,
            #[cfg(feature = "urdu")]
            Lang::Urd => Self::Urd,
        }
    }
}

#[cfg(feature = "lingua")]
impl From<Lang> for lingua::Language {
    fn from(lang: Lang) -> Self {
        match lang {
            #[cfg(feature = "arabic")]
            Lang::Ara => Self::Arabic,
            #[cfg(feature = "bengali")]
            Lang::Ben => Self::Bengali,
            #[cfg(feature = "mandarin")]
            Lang::Cmn => Self::Chinese,
            #[cfg(feature = "german")]
            Lang::Deu => Self::German,
            #[cfg(feature = "english")]
            Lang::Eng => Self::English,
            #[cfg(feature = "esperanto")]
            Lang::Epo => Self::Esperanto,
            #[cfg(feature = "french")]
            Lang::Fra => Self::French,
            #[cfg(feature = "hindi")]
            Lang::Hin => Self::Hindi,
            #[cfg(feature = "indonesian")]
            Lang::Ind => Self::Indonesian,
            #[cfg(feature = "italian")]
            Lang::Ita => Self::Italian,
            #[cfg(feature = "japanese")]
            Lang::Jpn => Self::Japanese,
            #[cfg(feature = "punjabi")]
            Lang::Pan => Self::Punjabi,
            #[cfg(feature = "portuguese")]
            Lang::Por => Self::Portuguese,
            #[cfg(feature = "russian")]
            Lang::Rus => Self::Russian,
            #[cfg(feature = "spanish")]
            Lang::Spa => Self::Spanish,
            #[cfg(feature = "turkish")]
            Lang::Tur => Self::Turkish,
            #[cfg(feature = "urdu")]
            Lang::Urd => Self::Urdu,
        }
    }
}

#[cfg(feature = "whatlang")]
impl TryFrom<whatlang::Lang> for Lang {
    type Error = whatlang::Lang;

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
            #[cfg(feature = "punjabi")]
            whatlang::Lang::Pan => Ok(Self::Pan),
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
impl TryFrom<lingua::Language> for Lang {
    type Error = lingua::Language;

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
            #[cfg(feature = "punjabi")]
            lingua::Language::Punjabi => Ok(Self::Pan),
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

/// Language detector.
#[derive(Debug)]
struct Detector {
    langs: BTreeSet<Lang>,
}

impl Default for Detector {
    fn default() -> Self {
        Self::all()
    }
}

impl Detector {
    fn empty() -> Self {
        Self {
            langs: BTreeSet::default(),
        }
    }

    fn all() -> Self {
        use strum::IntoEnumIterator;

        let langs = Lang::iter().collect();
        Self { langs }
    }

    fn allow(&mut self, lang: Lang) -> &mut Self {
        self.langs.insert(lang);
        self
    }

    fn deny(&mut self, lang: Lang) -> &mut Self {
        self.langs.remove(&lang);
        self
    }

    fn languages<L: From<Lang>>(&self) -> Vec<L> {
        self.langs.iter().map(|&lang| L::from(lang)).collect()
    }

    /// Detect a natural language.
    ///
    /// This returns [`None`] whenever the detection fails, its result
    /// is unreliable or it is probably a language we don't support at the moment.
    fn detect(&self, text: &str) -> Option<Lang> {
        self.detect_impl(text)
    }

    cfg_if! {
        if #[cfg(feature = "whatlang")] {
            fn detect_impl(&self, text: &str) -> Option<Lang> {
                let detector = whatlang::Detector::with_allowlist(self.languages());

                let info = detector.detect(text)?;
                log::debug!("whatlang information: {info:#?}");

                if info.is_reliable() {
                    let lang = info.lang();

                    lang.try_into().ok()
                } else {
                    None
                }
            }
        } else if #[cfg(feature = "lingua")] {
            fn detect_impl(&self, text: &str) -> Option<Lang> {
                let detector = lingua::LanguageDetectorBuilder::from_languages(&self.languages()).build();

                let lang = detector.detect_language_of(text)?;
                log::debug!("lingua language: {lang:#?}");

                lang.try_into().ok()
            }
        } else {
            fn detect_impl(&self, text: &str) -> Option<Lang> {
                compile_error!("Either feature \"whatlang\" or \"lingua\" must be enabled for this crate.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "esperanto")]
    #[test]
    fn simple_usage() {
        let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
        let lang = Detector::default().detect(text).unwrap();
        assert_eq!(lang, Lang::Epo);
    }

    #[cfg(all(feature = "english", feature = "russian"))]
    #[test]
    fn allow() {
        let text = "There is no reason not to learn Esperanto.";
        let lang = Detector::empty()
            .allow(Lang::Eng)
            .allow(Lang::Rus)
            .detect(text)
            .unwrap();
        assert_eq!(lang, Lang::Eng);
    }

    #[test]
    fn conversion_roundtrip() {
        use strum::IntoEnumIterator;

        for lang in Lang::iter() {
            #[cfg(feature = "whatlang")]
            assert_eq!(Ok(lang), whatlang::Lang::from(lang).try_into());

            #[cfg(feature = "lingua")]
            assert_eq!(Ok(lang), lingua::Language::from(lang).try_into());
        }
    }
}
