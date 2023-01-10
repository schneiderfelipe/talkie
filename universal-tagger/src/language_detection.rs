use whatlang::{detect, Lang};

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
