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
            Lang::Ara => whatlang::Lang::Ara,
            Lang::Ben => whatlang::Lang::Ben,
            Lang::Cmn => whatlang::Lang::Cmn,
            Lang::Deu => whatlang::Lang::Deu,
            Lang::Eng => whatlang::Lang::Eng,
            Lang::Epo => whatlang::Lang::Epo,
            Lang::Fra => whatlang::Lang::Fra,
            Lang::Hin => whatlang::Lang::Hin,
            Lang::Ind => whatlang::Lang::Ind,
            Lang::Ita => whatlang::Lang::Ita,
            Lang::Jpn => whatlang::Lang::Jpn,
            Lang::Pan => whatlang::Lang::Pan,
            Lang::Por => whatlang::Lang::Por,
            Lang::Rus => whatlang::Lang::Rus,
            Lang::Spa => whatlang::Lang::Spa,
            Lang::Tur => whatlang::Lang::Tur,
            Lang::Urd => whatlang::Lang::Urd,
        }
    }
}

impl TryFrom<whatlang::Lang> for Lang {
    type Error = whatlang::Lang;

    fn try_from(lang: whatlang::Lang) -> Result<Self, Self::Error> {
        match lang {
            whatlang::Lang::Ara => Ok(Lang::Ara),
            whatlang::Lang::Ben => Ok(Lang::Ben),
            whatlang::Lang::Cmn => Ok(Lang::Cmn),
            whatlang::Lang::Deu => Ok(Lang::Deu),
            whatlang::Lang::Eng => Ok(Lang::Eng),
            whatlang::Lang::Epo => Ok(Lang::Epo),
            whatlang::Lang::Fra => Ok(Lang::Fra),
            whatlang::Lang::Hin => Ok(Lang::Hin),
            whatlang::Lang::Ind => Ok(Lang::Ind),
            whatlang::Lang::Ita => Ok(Lang::Ita),
            whatlang::Lang::Jpn => Ok(Lang::Jpn),
            whatlang::Lang::Pan => Ok(Lang::Pan),
            whatlang::Lang::Por => Ok(Lang::Por),
            whatlang::Lang::Rus => Ok(Lang::Rus),
            whatlang::Lang::Spa => Ok(Lang::Spa),
            whatlang::Lang::Tur => Ok(Lang::Tur),
            whatlang::Lang::Urd => Ok(Lang::Urd),
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
