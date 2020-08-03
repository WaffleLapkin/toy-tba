use serde::Deserialize;

use std::path::PathBuf;

/// This object represents the contents of a file to be uploaded.
///
/// [The official docs](https://core.telegram.org/bots/api#inputfile).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, serde::Serialize)]
pub enum InputFile {
    File(PathBuf),
    Memory {
        file_name: String,
        data: std::borrow::Cow<'static, [u8]>,
    },
    Url(String),
    FileId(String),
}

impl InputFile {
    pub fn file(path: PathBuf) -> Self {
        Self::File(path)
    }

    pub fn url<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self::Url(url.into())
    }

    pub fn file_id<T>(file_id: T) -> Self
    where
        T: Into<String>,
    {
        Self::FileId(file_id.into())
    }

    pub fn as_file(&self) -> Option<&PathBuf> {
        match self {
            Self::File(path) => Some(path),
            _ => None,
        }
    }

    pub fn as_url(&self) -> Option<&String> {
        match self {
            Self::Url(url) => Some(url),
            _ => None,
        }
    }

    pub fn as_file_id(&self) -> Option<&String> {
        match self {
            Self::FileId(id) => Some(id),
            _ => None,
        }
    }
}

impl From<InputFile> for Option<PathBuf> {
    fn from(file: InputFile) -> Self {
        match file {
            InputFile::File(path) => Some(path),
            _ => None,
        }
    }
}
