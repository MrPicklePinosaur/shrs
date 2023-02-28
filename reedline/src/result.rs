use std::fmt::Display;

use thiserror::Error;

/// non-public (for now)
#[derive(Error, Debug)]
pub(crate) enum ReedlineErrorVariants {
    // todo: we should probably be more specific here
    #[cfg(any(feature = "sqlite", feature = "sqlite-dynlib"))]
    #[error("error within history database: {0}")]
    HistoryDatabaseError(String),
    #[error("error within history: {0}")]
    OtherHistoryError(&'static str),
    #[error("the history {history} does not support feature {feature}")]
    HistoryFeatureUnsupported {
        history: &'static str,
        feature: &'static str,
    },
    #[error("I/O error: {0}")]
    IOError(std::io::Error),
    #[error("public user thrown error")]
    DummyError,
}

/// separate struct to not expose anything to the public (for now)
#[derive(Debug)]
pub struct ReedlineError(pub(crate) ReedlineErrorVariants);

// hack to allow those not in this crate to implement History trait
impl ReedlineError {
    pub fn dummy() -> ReedlineError {
        ReedlineError(ReedlineErrorVariants::DummyError)
    }
}

impl Display for ReedlineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ReedlineError {}

// for now don't expose the above error type to the public
pub type Result<T> = std::result::Result<T, ReedlineError>;
