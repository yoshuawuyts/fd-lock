use std::ops;

use crate::sys;

/// RAII structure used to release the shared read access of a lock when
/// dropped.
///
/// This structure is created by the [`read`] and [`try_read`] methods on
/// [`FileLock`].
///
/// [`read`]: crate::FileLock::read
/// [`try_read`]: crate::FileLock::try_read
/// [`FileLock`]: crate::FileLock
///
/// # Panics
///
/// Dropping this type may panic if the lock fails to unlock.
#[must_use = "if unused the FileLock will immediately unlock"]
#[derive(Debug)]
pub struct FileLockReadGuard<'file_lock, T: sys::AsRaw> {
    guard: sys::FileLockReadGuard<'file_lock, T>,
}

impl<'file_lock, T: sys::AsRaw> FileLockReadGuard<'file_lock, T> {
    pub(crate) fn new(guard: sys::FileLockReadGuard<'file_lock, T>) -> Self {
        Self { guard }
    }
}

impl<T: sys::AsRaw> ops::Deref for FileLockReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

/// Release the lock.
impl<T: sys::AsRaw> Drop for FileLockReadGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {}
}
