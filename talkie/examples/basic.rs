use anyhow::Result;
use talkie::Input;

fn main() -> Result<()> {
    let input: String = Input::new()?.interact_text()?;
    println!("{input}");
    Ok(())
}
