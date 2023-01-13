use std::error::Error;

use textarea::Textarea;
use universal_tagger::{token_position_indices, LanguageDetector};

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let text: String = Textarea::new()?.with_prompt("> ").interact_text()?;
        if text == "quit" {
            break;
        }

        let lang = LanguageDetector::default().detect(&text);
        println!("Language: {lang:?}");

        let tokens: Vec<_> = token_position_indices(&text).collect();
        println!("{tokens:#?}");
    }
    Ok(())
}
