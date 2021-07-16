//! Advisory reader-writer locks for files.
//!
//! # Notes on Advisory Locks
//!
//! "advisory locks" are locks which programs must opt-in to adhere to. This
//! means that they can be used to coordinate file access, but not prevent
//! access. Use this to coordinate file access between multiple instances of the
//! same program. But do not use this to prevent actors from accessing or
//! modifying files.
//!
//! # Example
//!
//! ```rust
//! use fd_lock::FdLock;
//! # use tempfile::tempfile;
//! # use std::io::{self, prelude::*};
//! # use std::fs::File;
//!
//! # fn main() -> io::Result<()> {
//! // Lock a file and write to it.
//! let mut f = FdLock::new(tempfile()?);
//! f.write()?.write_all(b"chashu cat")?;
//!
//! // A lock can also be held across multiple operations.
//! let mut f = f.write()?;
//! f.write_all(b"nori cat")?;
//! f.write_all(b"bird!")?;
//! # Ok(())}
//! ```

#![forbid(future_incompatible)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples)]

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
pub use windows::*;
