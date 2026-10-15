//! Read-only Windows environment inspection and conservative compatibility reporting.
//!
//! No watermark operation, licensing change, elevation or persistence is implemented.

pub mod environment;

#[cfg(windows)]
mod windows;

#[cfg(not(windows))]
use environment::Environment;
use environment::Inspection;

/// Inspect this environment without requesting elevated privileges or changing it.
pub fn inspect() -> Inspection {
    #[cfg(windows)]
    let environment = windows::read_environment();
    #[cfg(not(windows))]
    let environment = Environment::other_platform();

    Inspection::new(environment)
}
