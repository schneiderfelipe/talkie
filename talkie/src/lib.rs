use std::marker::PhantomData;

use rustyline::{error::ReadlineError, Editor};
use thiserror::Error;

pub struct Input<T> {
    editor: Editor<()>,
    prompt: String,
    phantom: PhantomData<T>,
}

impl<T: Text> Input<T> {
    pub fn new() -> Result<Self, InputError> {
        Ok(Self {
            editor: Editor::new()?,
            prompt: String::new(),
            phantom: PhantomData,
        })
    }

    pub fn interact_text(&mut self) -> Result<T, InputError> {
        let line = self.editor.readline(&self.prompt)?;
        Ok(T::from_text(line)?)
    }
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("readline error {0}")]
    Readline(#[from] ReadlineError),
    #[error("text error {0}")]
    Text(#[from] TextError),
}

#[derive(Debug, Error)]
#[error("got {text}, found {message}")]
pub struct TextError {
    text: String,
    message: String,
}

pub trait Text: Sized {
    fn from_text(s: String) -> Result<Self, TextError>;
}

impl Text for String {
    fn from_text(s: String) -> Result<Self, TextError> {
        Ok(s)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
