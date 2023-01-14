use std::collections::BTreeSet;

use unicase::UniCase;

use crate::Language;

#[inline]
pub fn get(lang: Language) -> BTreeSet<UniCase<String>> {
    let lang: stop_words::LANGUAGE = lang.into();
    stop_words::get(lang)
        .into_iter()
        .map(UniCase::new)
        .collect()
}
