use std::collections::BTreeSet;

use log::debug;
use strum::EnumIter;

/// Language codes following the [ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3) standard.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
enum Lang {
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
        let detector = whatlang::Detector::with_allowlist(allowlist);

        let info = detector.detect(text)?;
        debug!("whatlang information: {info:#?}");
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
            assert_eq!(Ok(lang), whatlang::Lang::from(lang).try_into());
        }
    }
}
