use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use anyhow::{Error, Result};
use rustyline::Editor;

pub struct Input<T> {
    editor: Editor<()>,
    prompt: String,
    phantom: PhantomData<T>,
}

impl<T> Input<T> {
    /// Create a new [`Input`] object.
    ///
    /// # Errors
    ///
    /// This function returns an error if [`rustyline::Editor::new`]
    /// returns one.
    pub fn new() -> Result<Self> {
        Ok(Self {
            editor: Editor::new().map_err(Error::msg)?,
            prompt: String::new(),
            phantom: PhantomData,
        })
    }

    /// Interact with the user.
    ///
    /// # Errors
    ///
    /// This function returns an error if either [`rustyline::Editor::readline`] or [`FromStr::from_str`]
    /// returns one.
    pub fn interact_text(&mut self) -> Result<T>
    where
        T: FromStr,
        T::Err: Debug + Display + Send + Sync + 'static,
    {
        let line = self.editor.readline(&self.prompt).map_err(Error::msg)?;
        let value = line.parse().map_err(Error::msg)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
