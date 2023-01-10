use std::collections::BTreeSet;

use log::debug;
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
            Lang::Ara => Self::Ara,
            Lang::Ben => Self::Ben,
            Lang::Cmn => Self::Cmn,
            Lang::Deu => Self::Deu,
            Lang::Eng => Self::Eng,
            Lang::Epo => Self::Epo,
            Lang::Fra => Self::Fra,
            Lang::Hin => Self::Hin,
            Lang::Ind => Self::Ind,
            Lang::Ita => Self::Ita,
            Lang::Jpn => Self::Jpn,
            Lang::Pan => Self::Pan,
            Lang::Por => Self::Por,
            Lang::Rus => Self::Rus,
            Lang::Spa => Self::Spa,
            Lang::Tur => Self::Tur,
            Lang::Urd => Self::Urd,
        }
    }
}

#[cfg(feature = "lingua")]
impl From<Lang> for lingua::Language {
    fn from(lang: Lang) -> Self {
        match lang {
            Lang::Ara => Self::Arabic,
            Lang::Ben => Self::Bengali,
            Lang::Cmn => Self::Chinese,
            Lang::Deu => Self::German,
            Lang::Eng => Self::English,
            Lang::Epo => Self::Esperanto,
            Lang::Fra => Self::French,
            Lang::Hin => Self::Hindi,
            Lang::Ind => Self::Indonesian,
            Lang::Ita => Self::Italian,
            Lang::Jpn => Self::Japanese,
            Lang::Pan => Self::Punjabi,
            Lang::Por => Self::Portuguese,
            Lang::Rus => Self::Russian,
            Lang::Spa => Self::Spanish,
            Lang::Tur => Self::Turkish,
            Lang::Urd => Self::Urdu,
        }
    }
}

#[cfg(feature = "whatlang")]
impl TryFrom<whatlang::Lang> for Lang {
    type Error = whatlang::Lang;

    fn try_from(lang: whatlang::Lang) -> Result<Self, Self::Error> {
        match lang {
            whatlang::Lang::Ara => Ok(Self::Ara),
            whatlang::Lang::Ben => Ok(Self::Ben),
            whatlang::Lang::Cmn => Ok(Self::Cmn),
            whatlang::Lang::Deu => Ok(Self::Deu),
            whatlang::Lang::Eng => Ok(Self::Eng),
            whatlang::Lang::Epo => Ok(Self::Epo),
            whatlang::Lang::Fra => Ok(Self::Fra),
            whatlang::Lang::Hin => Ok(Self::Hin),
            whatlang::Lang::Ind => Ok(Self::Ind),
            whatlang::Lang::Ita => Ok(Self::Ita),
            whatlang::Lang::Jpn => Ok(Self::Jpn),
            whatlang::Lang::Pan => Ok(Self::Pan),
            whatlang::Lang::Por => Ok(Self::Por),
            whatlang::Lang::Rus => Ok(Self::Rus),
            whatlang::Lang::Spa => Ok(Self::Spa),
            whatlang::Lang::Tur => Ok(Self::Tur),
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
            lingua::Language::Arabic => Ok(Self::Ara),
            lingua::Language::Bengali => Ok(Self::Ben),
            lingua::Language::Chinese => Ok(Self::Cmn),
            lingua::Language::German => Ok(Self::Deu),
            lingua::Language::English => Ok(Self::Eng),
            lingua::Language::Esperanto => Ok(Self::Epo),
            lingua::Language::French => Ok(Self::Fra),
            lingua::Language::Hindi => Ok(Self::Hin),
            lingua::Language::Indonesian => Ok(Self::Ind),
            lingua::Language::Italian => Ok(Self::Ita),
            lingua::Language::Japanese => Ok(Self::Jpn),
            lingua::Language::Punjabi => Ok(Self::Pan),
            lingua::Language::Portuguese => Ok(Self::Por),
            lingua::Language::Russian => Ok(Self::Rus),
            lingua::Language::Spanish => Ok(Self::Spa),
            lingua::Language::Turkish => Ok(Self::Tur),
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

    /// Detect a natural language.
    ///
    /// This returns [`None`] whenever the detection fails, its result
    /// is unreliable or it is probably a language we don't support at the moment.
    fn detect(&self, text: &str) -> Option<Lang> {
        let allowlist = self.langs.iter().map(|&lang| lang.into()).collect();
        self.detect_impl(text, allowlist)
    }

    #[cfg(feature = "whatlang")]
    fn detect_impl(&self, text: &str, allowlist: Vec<whatlang::Lang>) -> Option<Lang> {
        let detector = whatlang::Detector::with_allowlist(allowlist);

        let info = detector.detect(text)?;
        debug!("whatlang information: {info:#?}");
        if info.is_reliable() {
            info.lang().try_into().ok()
        } else {
            None
        }
    }

    #[cfg(feature = "lingua")]
    fn detect_impl(&self, text: &str, allowlist: Vec<lingua::Language>) -> Option<Lang> {
        let detector = lingua::LanguageDetectorBuilder::from_languages(&allowlist).build();

        let lang = detector.detect_language_of(text)?;
        debug!("lingua language: {lang:#?}");
        lang.try_into().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
        let lang = Detector::default().detect(text).unwrap();
        assert_eq!(lang, Lang::Epo);

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
