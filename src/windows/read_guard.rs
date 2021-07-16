use winapi::um::fileapi::UnlockFile;

use std::ops;
use std::os::windows::prelude::*;

use crate::FdLock;

/// RAII structure used to release the shared read access of a lock when
/// dropped.
///
/// This structure is created by the [`read`] and [`try_read`] methods on
/// [`FdLock`].
///
/// [`read`]: FdLock::read
/// [`try_read`]: FdLock::try_read
///
/// # Panics
///
/// Dropping this type may panic if the lock fails to unlock.
#[must_use = "if unused the FdLock will immediately unlock"]
#[derive(Debug)]
pub struct FdLockReadGuard<'fdlock, T: AsRawHandle> {
    pub(crate) lock: &'fdlock FdLock<T>,
}

impl<T: AsRawHandle> ops::Deref for FdLockReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.inner
    }
}

impl<T: AsRawHandle> Drop for FdLockReadGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let handle = self.lock.inner.as_raw_handle();
        if unsafe { !UnlockFile(handle, 0, 0, 1, 0) } == 0 {
            panic!("Could not unlock the file descriptor");
        }
    }
}
