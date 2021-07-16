use std::ops;

use crate::sys;

/// RAII structure used to release the exclusive write access of a lock when
/// dropped.
///
/// This structure is created by the [`write`] and [`try_write`] methods
/// on [`FileLock`].
///
/// [`write`]: crate::FileLock::write
/// [`try_write`]: crate::FileLock::try_write
/// [`FileLock`]: crate::FileLock
///
/// # Panics
///
/// Dropping this type may panic if the lock fails to unlock.
#[must_use = "if unused the FileLock will immediately unlock"]
#[derive(Debug)]
pub struct FileLockWriteGuard<'file_lock, T: sys::AsRaw> {
    guard: sys::FileLockWriteGuard<'file_lock, T>,
}

impl<'file_lock, T: sys::AsRaw> FileLockWriteGuard<'file_lock, T> {
    pub(crate) fn new(guard: sys::FileLockWriteGuard<'file_lock, T>) -> Self {
        Self { guard }
    }
}

impl<T: sys::AsRaw> ops::Deref for FileLockWriteGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.guard.deref()
    }
}

impl<T: sys::AsRaw> ops::DerefMut for FileLockWriteGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut()
    }
}

impl<T: sys::AsRaw> Drop for FileLockWriteGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {}
}
