//! Errors that may occur while using rdom.

use thiserror::Error as ThisError;

/// A static rendering DOM exception
#[derive(Debug, ThisError)]
pub enum DomError {
    /// Received an invalid query selector. Static rendering only supports
    /// "body" and "html".
    #[error("invalid query selector")]
    InvalidQuerySelector,

    /// The object being operated on was created in a sandbox that has since
    /// disappeared.
    #[error("the sandbox was dropped")]
    SandboxDropped,
}
