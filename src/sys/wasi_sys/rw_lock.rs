use super::{io_try, RwLockReadGuard, RwLockWriteGuard};
use std::io::{self, Error};
use std::os::fd::{AsFd, BorrowedFd};
use std::os::wasi::io::{AsRawFd, RawFd};
use wasi::wasi_filesystem::{lock_exclusive, lock_shared, try_lock_exclusive, try_lock_shared};

#[derive(Debug)]
pub struct RwLock<T: AsRawFd> {
    pub(crate) inner: T,
}

impl<T: AsRawFd> RwLock<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        RwLock { inner }
    }

    #[inline]
    pub fn write(&mut self) -> io::Result<RwLockWriteGuard<'_, T>> {
        io_try(lock_exclusive(self.as_raw_fd() as u32))?;
        Ok(RwLockWriteGuard::new(self))
    }

    #[inline]
    pub fn try_write(&mut self) -> Result<RwLockWriteGuard<'_, T>, Error> {
        io_try(try_lock_exclusive(self.as_raw_fd() as u32))?;
        Ok(RwLockWriteGuard::new(self))
    }

    #[inline]
    pub fn read(&self) -> io::Result<RwLockReadGuard<'_, T>> {
        io_try(lock_shared(self.as_raw_fd() as u32))?;
        Ok(RwLockReadGuard::new(self))
    }

    #[inline]
    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, Error> {
        io_try(try_lock_shared(self.as_raw_fd() as u32))?;
        Ok(RwLockReadGuard::new(self))
    }

    #[inline]
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.inner
    }
}

impl<T: AsRawFd> AsRawFd for RwLock<T> {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}
impl<T: AsRawFd> AsFd for RwLock<T> {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}
