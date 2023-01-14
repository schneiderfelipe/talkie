use std::error::Error;

use textarea::Textarea;
use universal_tagger::{LanguageDetector, Tagger};

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let text: String = Textarea::new()?.with_prompt("> ").interact_text()?;
        if text == "quit" {
            break;
        }

        let lang = LanguageDetector::default().detect(&text);
        let lang = if let Some(lang) = lang {
            println!("Language is {lang:?}");
            lang
        } else {
            println!("Unknown language");
            continue;
        };

        let tokens: Vec<_> = Tagger::new(lang).tag(&text).collect();
        println!("{tokens:#?}");
    }
    Ok(())
}
