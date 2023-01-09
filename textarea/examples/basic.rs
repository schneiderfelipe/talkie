use anyhow::Result;
use textarea::Textarea;

fn main() -> Result<()> {
    let s: String = Textarea::new()?.with_prompt("> ").interact_text()?;
    println!("{s}");
    Ok(())
}
