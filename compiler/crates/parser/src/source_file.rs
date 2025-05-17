use crate::Tokens;
use std::path::PathBuf;

pub struct SourceFile {
    pub path: String,
    pub contents: String,
}

impl SourceFile {
    pub fn new_from_path(path: PathBuf) -> std::io::Result<Self> {
        Ok(Self {
            path: path.display().to_string(),
            contents: std::fs::read_to_string(path)?,
        })
    }

    pub fn new_from_source<S: ToString>(path: S, contents: S) -> Self {
        Self {
            path: path.to_string(),
            contents: contents.to_string(),
        }
    }

    pub fn tokens(&self) -> Tokens {
        Tokens::new(self)
    }
}
