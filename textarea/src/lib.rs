use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use anyhow::{Error, Result};
use rustyline::{Cmd, Editor, KeyEvent};

pub struct Textarea<T> {
    editor: Editor<()>,
    prompt: String,
    phantom: PhantomData<T>,
}

impl<T> Textarea<T> {
    /// Create a new [`Textarea`] object.
    ///
    /// # Errors
    ///
    /// This function returns an error if [`rustyline::Editor::new`]
    /// returns one.
    pub fn new() -> Result<Self> {
        let mut editor = Editor::new().map_err(Error::msg)?;

        #[cfg(feature = "multiline")]
        editor.bind_sequence(KeyEvent::alt('\r'), Cmd::Newline);

        Ok(Self {
            editor,
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
