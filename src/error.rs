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
}

impl From<dialoguer::Error> for Error {
    fn from(err: dialoguer::Error) -> Self {
        Error::Dialog(err)
    }
}
