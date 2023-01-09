use anyhow::Result;
use textarea::Textarea;

fn main() -> Result<()> {
    let s: String = Textarea::new()?.interact_text()?;
    println!("{s}");
    Ok(())
}
