use {
    crate::*,
    thiserror::Error,
    std::{
        io,
        path::PathBuf,
    },
};

#[derive(Debug, Error)]
pub enum RhitError {
    #[error("No hit found in {0:?}")]
    NoHitInPaths(Vec<PathBuf>),
    #[error("No log file found")]
    NoLogFileFound,
    #[error("Path not found: {0:?}")]
    PathNotFound(PathBuf),
    #[error("IO error: {0:?}")]
    Io(#[from] io::Error),
    #[error("Date time parsing error: {0:?}")]
    DateTime(#[from] ParseDateTimeError),
    #[error("status filter parsing error: {0:?}")]
    StatusFilter(#[from] ParseStatusFilterError),
    #[error("String filter parsing error: {0:?}")]
    StrFilter(#[from] ParseStrFilterError),
    #[error("time filter parsing error: {0:?}")]
    TimeFilter(#[from] ParseTimeFilterError),
}
