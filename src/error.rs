//! Error types for the linux component.

/// Errors in kernel version/config handling.
#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    /// Kernel Makefile could not be parsed.
    #[error("makefile parse error: {0}")]
    MakefileParse(String),

    /// Invalid kernel version string.
    #[error("invalid kernel version: {0}")]
    InvalidVersion(String),

    /// I/O error.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
