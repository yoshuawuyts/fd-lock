use libc::{flock, LOCK_EX, LOCK_NB, LOCK_SH};
use std::io::{self, Error, ErrorKind};
use std::os::unix::io::AsRawFd;

use super::utils::syscall;
use super::{FileLockReadGuard, FileLockWriteGuard};

#[derive(Debug)]
pub struct FileLock<T: AsRawFd> {
    pub(crate) inner: T,
}

impl<T: AsRawFd> FileLock<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        FileLock { inner }
    }

    #[inline]
    pub fn write(&mut self) -> io::Result<FileLockWriteGuard<'_, T>> {
        let fd = self.inner.as_raw_fd();
        syscall(unsafe { flock(fd, LOCK_EX) })?;
        Ok(FileLockWriteGuard::new(self))
    }

    #[inline]
    pub fn try_write(&mut self) -> Result<FileLockWriteGuard<'_, T>, Error> {
        let fd = self.inner.as_raw_fd();
        syscall(unsafe { flock(fd, LOCK_EX | LOCK_NB) }).map_err(|err| match err.kind() {
            ErrorKind::AlreadyExists => ErrorKind::WouldBlock.into(),
            _ => err,
        })?;
        Ok(FileLockWriteGuard::new(self))
    }

    #[inline]
    pub fn read(&self) -> io::Result<FileLockReadGuard<'_, T>> {
        let fd = self.inner.as_raw_fd();
        syscall(unsafe { flock(fd, LOCK_SH) })?;
        Ok(FileLockReadGuard::new(self))
    }

    #[inline]
    pub fn try_read(&self) -> Result<FileLockReadGuard<'_, T>, Error> {
        let fd = self.inner.as_raw_fd();
        syscall(unsafe { flock(fd, LOCK_SH | LOCK_NB) }).map_err(|err| match err.kind() {
            ErrorKind::AlreadyExists => ErrorKind::WouldBlock.into(),
            _ => err,
        })?;
        Ok(FileLockReadGuard::new(self))
    }

    #[inline]
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.inner
    }
}
