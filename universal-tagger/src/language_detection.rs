use std::collections::BTreeSet;

use strum::EnumIter;

/// Language codes following the [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
enum Language {
    /// العربية (Arabic).
    Ara,
    /// বাংলা (Bengali).
    Ben,
    /// 普通话 (Mandarin).
    Cmn,
    /// Deutsch (German).
    Deu,
    /// English (English).
    Eng,
    /// Esperanto (Esperanto).
    Epo,
    /// Français (French).
    Fra,
    /// हिन्दी (Hindi).
    Hin,
    /// Bahasa Indonesia (Indonesian).
    Ind,
    /// Italiano (Italian).
    Ita,
    /// 日本語 (Japanese).
    Jpn,
    /// ਪੰਜਾਬੀ (Punjabi).
    Pan,
    /// Português (Portuguese).
    Por,
    /// Русский (Russian).
    Rus,
    /// Español (Spanish).
    Spa,
    /// Türkçe (Turkish).
    Tur,
    /// اُردُو (Urdu).
    Urd,
}

impl From<Language> for whatlang::Lang {
    fn from(lang: Language) -> Self {
        match lang {
            Language::Ara => whatlang::Lang::Ara,
            Language::Ben => whatlang::Lang::Ben,
            Language::Cmn => whatlang::Lang::Cmn,
            Language::Deu => whatlang::Lang::Deu,
            Language::Eng => whatlang::Lang::Eng,
            Language::Epo => whatlang::Lang::Epo,
            Language::Fra => whatlang::Lang::Fra,
            Language::Hin => whatlang::Lang::Hin,
            Language::Ind => whatlang::Lang::Ind,
            Language::Ita => whatlang::Lang::Ita,
            Language::Jpn => whatlang::Lang::Jpn,
            Language::Pan => whatlang::Lang::Pan,
            Language::Por => whatlang::Lang::Por,
            Language::Rus => whatlang::Lang::Rus,
            Language::Spa => whatlang::Lang::Spa,
            Language::Tur => whatlang::Lang::Tur,
            Language::Urd => whatlang::Lang::Urd,
        }
    }
}

impl TryFrom<whatlang::Lang> for Language {
    type Error = whatlang::Lang;

    fn try_from(lang: whatlang::Lang) -> Result<Self, Self::Error> {
        match lang {
            whatlang::Lang::Ara => Ok(Language::Ara),
            whatlang::Lang::Ben => Ok(Language::Ben),
            whatlang::Lang::Cmn => Ok(Language::Cmn),
            whatlang::Lang::Deu => Ok(Language::Deu),
            whatlang::Lang::Eng => Ok(Language::Eng),
            whatlang::Lang::Epo => Ok(Language::Epo),
            whatlang::Lang::Fra => Ok(Language::Fra),
            whatlang::Lang::Hin => Ok(Language::Hin),
            whatlang::Lang::Ind => Ok(Language::Ind),
            whatlang::Lang::Ita => Ok(Language::Ita),
            whatlang::Lang::Jpn => Ok(Language::Jpn),
            whatlang::Lang::Pan => Ok(Language::Pan),
            whatlang::Lang::Por => Ok(Language::Por),
            whatlang::Lang::Rus => Ok(Language::Rus),
            whatlang::Lang::Spa => Ok(Language::Spa),
            whatlang::Lang::Tur => Ok(Language::Tur),
            whatlang::Lang::Urd => Ok(Language::Urd),
            lang => Err(lang),
        }
    }
}

/// Language detector.
struct Detector {
    languages: BTreeSet<Language>,
}

impl Default for Detector {
    fn default() -> Self {
        use strum::IntoEnumIterator;

        let languages = Language::iter().collect();
        Self { languages }
    }
}

impl Detector {
    /// Detect a natural language.
    ///
    /// This returns [`None`] whenever the detection fails, its result
    /// is unreliable or it is probably a language we don't support at the moment.
    fn detect(&self, text: &str) -> Option<Language> {
        let allowlist = self.languages.iter().map(|&lang| lang.into()).collect();
        let detector = whatlang::Detector::with_allowlist(allowlist);

        let info = detector.detect(text)?;
        if info.is_reliable() {
            info.lang().try_into().ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
        let lang = Detector::default().detect(text).unwrap();
        assert_eq!(lang, Language::Epo);

        let text = "There is no reason not to learn Esperanto.";
        let lang = Detector::default().detect(text).unwrap();
        assert_eq!(lang, Language::Eng);
    }

    #[test]
    fn conversion_roundtrip() {
        use strum::IntoEnumIterator;

        for lang in Language::iter() {
            assert_eq!(Ok(lang), whatlang::Lang::from(lang).try_into());
        }
    }
}
