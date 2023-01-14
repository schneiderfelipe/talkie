use std::collections::BTreeSet;

use cfg_if::cfg_if;

use crate::Language;

/// Language detector.
#[derive(Debug)]
pub struct LanguageDetector {
    langs: BTreeSet<Language>,
}

impl Default for LanguageDetector {
    #[inline]
    fn default() -> Self {
        Self::all()
    }
}

impl LanguageDetector {
    #[inline]
    #[must_use]
    pub fn empty() -> Self {
        Self {
            langs: BTreeSet::default(),
        }
    }

    #[inline]
    #[must_use]
    pub fn all() -> Self {
        use strum::IntoEnumIterator;

        let langs = Language::iter().collect();
        Self { langs }
    }

    #[inline]
    pub fn allow(&mut self, lang: Language) -> &mut Self {
        self.langs.insert(lang);
        self
    }

    #[inline]
    pub fn deny(&mut self, lang: Language) -> &mut Self {
        self.langs.remove(&lang);
        self
    }

    #[inline]
    pub fn languages<L: From<Language>>(&self) -> impl Iterator<Item = L> + '_ {
        self.langs.iter().map(|&lang| L::from(lang))
    }

    /// Detect a natural language.
    ///
    /// This returns [`None`] whenever the detection fails, its result
    /// is unreliable or it is probably a language we don't support at the moment.
    ///
    /// # Panics
    /// This function panics if the list of allowed languages is less than two.
    #[inline]
    #[must_use]
    pub fn detect(&self, text: &str) -> Option<Language> {
        assert!(
            self.langs.len() > 1,
            "Detector needs at least two languages to choose from"
        );

        cfg_if! {
            if #[cfg(all(feature = "whatlang", feature = "lingua"))] {
                self.detect_whatlang(text).or_else(|| self.detect_lingua(text)) // NOTE: best of both worlds
            } else if #[cfg(feature = "whatlang")] {
                self.detect_whatlang(text) // NOTE: more performance
            } else if #[cfg(feature = "lingua")] {
                self.detect_lingua(text) // NOTE: higher accuracy
            } else {
               compile_error!("Either feature \"whatlang\" or \"lingua\" must be enabled for this crate.")
            }
        }
    }

    #[cfg(feature = "whatlang")]
    #[inline]
    fn detect_whatlang(&self, text: &str) -> Option<Language> {
        let detector = whatlang::Detector::with_allowlist(self.languages().collect());

        let info = detector.detect(text)?;
        log::debug!("whatlang information: {info:#?}");

        if info.is_reliable() {
            let lang = info.lang();

            lang.try_into().ok()
        } else {
            None
        }
    }

    #[cfg(feature = "lingua")]
    #[inline]
    fn detect_lingua(&self, text: &str) -> Option<Language> {
        let detector =
            lingua::LanguageDetectorBuilder::from_languages(&self.languages().collect::<Vec<_>>())
                .build();

        let lang = detector.detect_language_of(text)?;
        log::debug!("lingua language: {lang:#?}");

        lang.try_into().ok()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[cfg(feature = "esperanto")]
    #[test]
    fn simple_usage() {
        let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
        let lang = LanguageDetector::default().detect(text).unwrap();
        assert_eq!(lang, Language::Epo);
    }

    #[cfg(all(feature = "english", feature = "russian"))]
    #[test]
    fn allow() {
        let text = "There is no reason not to learn Esperanto.";
        let lang = LanguageDetector::empty()
            .allow(Language::Eng)
            .allow(Language::Rus)
            .detect(text)
            .unwrap();
        assert_eq!(lang, Language::Eng);
    }
}
