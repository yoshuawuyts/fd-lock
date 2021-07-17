use libc::{flock, LOCK_UN};
use std::ops;
use std::os::unix::io::AsRawFd;

use super::utils::syscall;
use super::FileLock;

#[derive(Debug)]
pub struct FileLockWriteGuard<'lock, T: AsRawFd> {
    lock: &'lock mut FileLock<T>,
}

impl<'lock, T: AsRawFd> FileLockWriteGuard<'lock, T> {
    pub(crate) fn new(lock: &'lock mut FileLock<T>) -> Self {
        Self { lock }
    }
}

impl<T: AsRawFd> ops::Deref for FileLockWriteGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.inner
    }
}

impl<T: AsRawFd> ops::DerefMut for FileLockWriteGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock.inner
    }
}

impl<T: AsRawFd> Drop for FileLockWriteGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let fd = self.lock.inner.as_raw_fd();
        let _ = syscall(unsafe { flock(fd, LOCK_UN) });
    }
}
