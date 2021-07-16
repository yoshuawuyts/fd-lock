use winapi::um::fileapi::UnlockFile;

use std::ops;
use std::os::windows::prelude::*;

use crate::FileLock;

/// RAII structure used to release the shared read access of a lock when
/// dropped.
///
/// This structure is created by the [`read`] and [`try_read`] methods on
/// [`FileLock`].
///
/// [`read`]: FileLock::read
/// [`try_read`]: FileLock::try_read
///
/// # Panics
///
/// Dropping this type may panic if the lock fails to unlock.
#[must_use = "if unused the FileLock will immediately unlock"]
#[derive(Debug)]
pub struct FileLockReadGuard<'file_lock, T: AsRawHandle> {
    pub(crate) lock: &'file_lock FileLock<T>,
}

impl<T: AsRawHandle> ops::Deref for FileLockReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.inner
    }
}

impl<T: AsRawHandle> Drop for FileLockReadGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let handle = self.lock.inner.as_raw_handle();
        if unsafe { !UnlockFile(handle, 0, 0, 1, 0) } == 0 {
            panic!("Could not unlock the file descriptor");
        }
    }
}
