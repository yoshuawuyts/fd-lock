use std::io::{self, Error, ErrorKind};
use std::os::windows::io::AsRawHandle;

use winapi::um::fileapi::LockFileEx;
use winapi::um::minwinbase::{LOCKFILE_EXCLUSIVE_LOCK, LOCKFILE_FAIL_IMMEDIATELY};

mod read_guard;
mod utils;
mod write_guard;

pub use read_guard::FdLockReadGuard;
pub use write_guard::FdLockWriteGuard;

use utils::{syscall, Overlapped};

/// Advisory reader-writer lock for files.
///
/// This type of lock allows a number of readers or at most one writer at any point
/// in time. The write portion of this lock typically allows modification of the
/// underlying data (exclusive access) and the read portion of this lock typically
/// allows for read-only access (shared access).
#[derive(Debug)]
pub struct FdLock<T: AsRawHandle> {
    inner: T,
}

impl<T: AsRawHandle> FdLock<T> {
    /// Create a new instance.
    #[inline]
    pub fn new(inner: T) -> Self {
        FdLock { inner }
    }

    /// Locks this lock with shared read access, blocking the current thread
    /// until it can be acquired.
    ///
    /// The calling thread will be blocked until there are no more writers which
    /// hold the lock. There may be other readers currently inside the lock when
    /// this method returns. This method does not provide any guarantees with
    /// respect to the ordering of whether contentious readers or writers will
    /// acquire the lock first.
    ///
    /// Returns an RAII guard which will release this thread's shared access
    /// once it is dropped.
    ///
    /// # Errors
    ///
    /// On Unix this may return an `ErrorKind::Interrupted` if the operation was
    /// interrupted by a signal handler.
    #[inline]
    pub fn read(&self) -> io::Result<FdLockReadGuard<'_, T>> {
        // See: https://stackoverflow.com/a/9186532, https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex
        let handle = self.inner.as_raw_handle();
        let overlapped = Overlapped::zero();
        let flags = 0;
        syscall(unsafe { LockFileEx(handle, flags, 0, 1, 0, overlapped.raw()) })?;
        Ok(FdLockReadGuard { lock: self })
    }

    /// Attempts to acquire this lock with shared read access.
    ///
    /// If the access could not be granted at this time, then `Err` is returned.
    /// Otherwise, an RAII guard is returned which will release the shared access
    /// when it is dropped.
    ///
    /// This function does not block.
    ///
    /// This function does not provide any guarantees with respect to the ordering
    /// of whether contentious readers or writers will acquire the lock first.
    ///
    /// # Errors
    ///
    /// If the lock is already held and `ErrorKind::WouldBlock` error is returned.
    /// On Unix this may return an `ErrorKind::Interrupted` if the operation was
    /// interrupted by a signal handler.
    #[inline]
    pub fn try_read(&self) -> io::Result<FdLockReadGuard<'_, T>> {
        let handle = self.inner.as_raw_handle();
        let overlapped = Overlapped::zero();
        let flags = LOCKFILE_FAIL_IMMEDIATELY;

        syscall(unsafe { LockFileEx(handle, flags, 0, 1, 0, overlapped.raw()) })
            .map_err(|_| Error::from(ErrorKind::WouldBlock))?;
        Ok(FdLockReadGuard { lock: self })
    }

    /// Locks this lock with exclusive write access, blocking the current thread
    /// until it can be acquired.
    ///
    /// This function will not return while other writers or other readers
    /// currently have access to the lock.
    ///
    /// Returns an RAII guard which will drop the write access of this rwlock
    /// when dropped.
    ///
    /// # Errors
    ///
    /// On Unix this may return an `ErrorKind::Interrupted` if the operation was
    /// interrupted by a signal handler.
    #[inline]
    pub fn write(&mut self) -> io::Result<FdLockWriteGuard<'_, T>> {
        // See: https://stackoverflow.com/a/9186532, https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex
        let handle = self.inner.as_raw_handle();
        let overlapped = Overlapped::zero();
        let flags = LOCKFILE_EXCLUSIVE_LOCK;
        syscall(unsafe { LockFileEx(handle, flags, 0, 1, 0, overlapped.raw()) })?;
        Ok(FdLockWriteGuard { lock: self })
    }

    /// Attempts to lock this lock with exclusive write access.
    ///
    /// If the lock could not be acquired at this time, then `Err` is returned.
    /// Otherwise, an RAII guard is returned which will release the lock when
    /// it is dropped.
    ///
    /// # Errors
    ///
    /// If the lock is already held and `ErrorKind::WouldBlock` error is returned.
    /// On Unix this may return an `ErrorKind::Interrupted` if the operation was
    /// interrupted by a signal handler.
    #[inline]
    pub fn try_write(&mut self) -> io::Result<FdLockWriteGuard<'_, T>> {
        let handle = self.inner.as_raw_handle();
        let overlapped = Overlapped::zero();
        let flags = LOCKFILE_FAIL_IMMEDIATELY | LOCKFILE_EXCLUSIVE_LOCK;

        syscall(unsafe { LockFileEx(handle, flags, 0, 1, 0, overlapped.raw()) })
            .map_err(|_| Error::from(ErrorKind::WouldBlock))?;
        Ok(FdLockWriteGuard { lock: self })
    }

    /// Consumes this `FdLock`, returning the underlying data.
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.inner
    }
}
