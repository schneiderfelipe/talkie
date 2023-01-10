use whatlang::{detect, Lang};

#[non_exhaustive]
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

/// Detect a natural language.
///
/// This returns [`None`] whenever the detection fails or its result
/// is unreliable.
fn detect_language(text: &str) -> Option<Lang> {
    let info = detect(text)?;
    if info.is_reliable() {
        Some(info.lang())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
        let lang = detect_language(text).unwrap();
        assert_eq!(lang, Lang::Epo);
    }
}
