use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error:{0}")]
    Io(#[from] std::io::Error),
    #[error("Cargo metadata parsing error:{0}")]
    Metadata(#[from] cargo_metadata::Error),
    #[error("Dependency graph contains loops")]
    CyclicDependency,
    #[error("Cargo publish failed: crate {0}, error: {1}")]
    Publish(String, String),
    #[error("User cancels publish")]
    UserCancelled,
    #[error("Invalid command line arguments: {0}")]
    Dialog(dialoguer::Error),
    #[error("Cache serialization error:{0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Failed to read file `{path}`: {source}")]
    FileRead {
        path: String,
        source: std::io::Error,
    },
    #[error("Failed to write to file `{path}`: {source}")]
    FileWrite {
        path: String,
        source: std::io::Error,
    },
    #[error("Failed to parse TOML in `{path}`: {source}")]
    TomlParse {
        path: String,
        source: toml_edit::TomlError,
    },
    #[error("Dependencies in `{path}` are not sorted")]
    NotSorted { path: String },
    #[error("Invalid sort order '{0}', must be 'asc' or 'desc'")]
    InvalidSortOrder(String),
}

impl From<dialoguer::Error> for Error {
    fn from(err: dialoguer::Error) -> Self {
        Error::Dialog(err)
    }
}
