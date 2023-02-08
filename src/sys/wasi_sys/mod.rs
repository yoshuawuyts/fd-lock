//! From:
//! https://github.com/sunfishcode/rust-wasi/blob/6c37a764b571f73c1dd6dac4bc5360093c651f6d/wit/wasi-filesystem.wit.md

mod read_guard;
mod rw_lock;
mod write_guard;

use std::io::{Error, ErrorKind};

pub use read_guard::RwLockReadGuard;
pub use rw_lock::RwLock;
pub use write_guard::RwLockWriteGuard;

/// Convert from a WASI fs error to a Rust io Error
pub(crate) fn io_try<T>(res: Result<T, wasi::wasi_filesystem::Errno>) -> std::io::Result<T> {
    res.map_err(|err| match err {
        wasi::wasi_filesystem::Errno::Again => Error::new(ErrorKind::WouldBlock, err),
        _ => Error::new(ErrorKind::Other, err),
    })
}
