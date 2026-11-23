use std::fmt;
use std::path::{Path, PathBuf};

/// Errors from compiling, loading or configuring a dictionary. Segmentation itself never fails.
#[derive(Debug)]
pub enum Error {
    /// Reading or writing a file failed.
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    /// A dictionary source file (char.def, unk.def, matrix.def, *.csv, user CSV) is malformed.
    Source {
        path: PathBuf,
        line: usize,
        message: String,
    },
    /// A compiled dictionary file is not in the expected format.
    Format(String),
    /// The dictionaries do not fit together (e.g. a user entry's context id is outside the matrix).
    Invalid(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub(crate) fn io(path: &Path, source: std::io::Error) -> Self {
        Error::Io {
            path: path.to_path_buf(),
            source,
        }
    }

    pub(crate) fn source(path: &Path, line: usize, message: impl Into<String>) -> Self {
        Error::Source {
            path: path.to_path_buf(),
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Error::Source {
                path,
                line,
                message,
            } if *line > 0 => {
                write!(f, "{}:{line}: {message}", path.display())
            }
            Error::Source { path, message, .. } => write!(f, "{}: {message}", path.display()),
            Error::Format(message) => write!(f, "invalid dictionary file: {message}"),
            Error::Invalid(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}
