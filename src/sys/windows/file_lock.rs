use std::io::{self, Error, ErrorKind};
use std::os::windows::io::AsRawHandle;

use winapi::um::fileapi::LockFileEx;
use winapi::um::minwinbase::{LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY};

use super::utils::{syscall, Overlapped};
use super::{FileLockReadGuard, FileLockWriteGuard};

#[derive(Debug)]
pub struct FileLock<T: AsRawHandle> {
    pub(crate) inner: T,
}

impl<T: AsRawHandle> FileLock<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        FileLock { inner }
    }

    #[inline]
    pub fn read(&self) -> io::Result<FileLockReadGuard<'_, T>> {
        // See: https://stackoverflow.com/a/9186532, https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex
        let handle = self.inner.as_raw_handle();
        let overlapped = Overlapped::zero();
        let flags = 0;
        syscall(unsafe { LockFileEx(handle, flags, 0, 1, 0, overlapped.raw()) })?;
        Ok(FileLockReadGuard { lock: self })
    }

    #[inline]
    pub fn try_read(&self) -> io::Result<FileLockReadGuard<'_, T>> {
        let handle = self.inner.as_raw_handle();
        let overlapped = Overlapped::zero();
        let flags = LOCKFILE_FAIL_IMMEDIATELY;

        syscall(unsafe { LockFileEx(handle, flags, 0, 1, 0, overlapped.raw()) })
            .map_err(|_| Error::from(ErrorKind::WouldBlock))?;
        Ok(FileLockReadGuard { lock: self })
    }

    #[inline]
    pub fn write(&mut self) -> io::Result<FileLockWriteGuard<'_, T>> {
        // See: https://stackoverflow.com/a/9186532, https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex
        let handle = self.inner.as_raw_handle();
        let overlapped = Overlapped::zero();
        let flags = LOCKFILE_EXCLUSIVE_LOCK;
        syscall(unsafe { LockFileEx(handle, flags, 0, 1, 0, overlapped.raw()) })?;
        Ok(FileLockWriteGuard { lock: self })
    }

    #[inline]
    pub fn try_write(&mut self) -> io::Result<FileLockWriteGuard<'_, T>> {
        let handle = self.inner.as_raw_handle();
        let overlapped = Overlapped::zero();
        let flags = LOCKFILE_FAIL_IMMEDIATELY | LOCKFILE_EXCLUSIVE_LOCK;

        syscall(unsafe { LockFileEx(handle, flags, 0, 1, 0, overlapped.raw()) })
            .map_err(|_| Error::from(ErrorKind::WouldBlock))?;
        Ok(FileLockWriteGuard { lock: self })
    }

    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.inner
    }
}
