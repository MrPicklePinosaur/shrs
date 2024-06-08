
use thiserror::Error;

use crate::ParserError;

#[derive(Error, Debug)]
pub enum PosixError {
    /// Error when attempting file redirection
    #[error("Redirection Error: {0}")]
    Redirect(std::io::Error),
    /// Error emitted by hook
    #[error("Hook Error:")]
    Hook(),
    /// Issue parsing command
    #[error("Parse failed: {0}")]
    Parse(ParserError),
    /// Issue evaluating command
    #[error("Failed evaluating command: {0}")]
    Eval(anyhow::Error),
    /// Command not found
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    /// Job manager specific error
    #[error("Job manager error: {0}")]
    Job(anyhow::Error),
}
