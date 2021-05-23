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

    /// The object being operated on is out of memory. This does not mean the
    /// sandbox as a whole is out of memory.
    #[error("object out of memory")]
    ObjectOutOfMemory,

    /// The object being operated on is out of memory. This does not mean the
    /// sandbox as a whole is out of memory.
    #[error("range error (number was out of range)")]
    Range,

    /// An attribute node was attempted to be used when it is already in use
    /// for another element.
    #[error("attribute is already in use in another element")]
    InUseAttribute,

    /// An attribute node was attempted to be used when it is already in use
    /// for another element.
    #[error("attribute is already in use in another element")]
    NotFound,
}
