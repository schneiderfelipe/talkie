use std::error::Error;

use textarea::Textarea;
use universal_tagger::{token_position_indices, LanguageDetector};

fn main() -> Result<(), Box<dyn Error>> {
    let text: String = Textarea::new()?.with_prompt("> ").interact_text()?;
    let lang = LanguageDetector::default().detect(&text);
    println!("Language: {lang:?}");

    let tokens: Vec<_> = token_position_indices(&text).collect();
    println!("{tokens:#?}");
    Ok(())
}
