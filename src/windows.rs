use std::io::{self, Error, ErrorKind};
use std::mem;
use std::ops;
use std::os::windows::io::AsRawHandle;

use winapi::um::fileapi::{LockFile, LockFileEx, UnlockFile};
use winapi::um::minwinbase::{LOCKFILE_EXCLUSIVE_LOCK, OVERLAPPED};

/// A guard that unlocks the file descriptor when it goes out of scope.
///
/// # Panics
///
/// Dropping this type may panic if the lock fails to unlock.
#[derive(Debug)]
pub struct FdLockGuard<'fdlock, T: AsRawHandle> {
    lock: &'fdlock mut FdLock<T>,
}

impl<T: AsRawHandle> ops::Deref for FdLockGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.t
    }
}

impl<T: AsRawHandle> ops::DerefMut for FdLockGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock.t
    }
}

impl<T: AsRawHandle> Drop for FdLockGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let handle = self.lock.t.as_raw_handle();
        if unsafe { !UnlockFile(handle, 0, 0, 1, 0) } == 0 {
            panic!("Could not unlock the file descriptor");
        }
    }
}

/// A file descriptor lock.
#[derive(Debug)]
pub struct FdLock<T: AsRawHandle> {
    t: T,
}

impl<T: AsRawHandle> FdLock<T> {
    /// Create a new instance.
    #[inline]
    pub fn new(t: T) -> Self {
        FdLock { t }
    }

    /// Acquires a new lock, blocking the current thread until it's able to do so.
    ///
    /// This function will block the local thread until it is available to acquire the lock. Upon
    /// returning, the thread is the only thread with the lock held. An RAII guard is returned to allow
    /// scoped unlock of the lock. When the guard goes out of scope, the lock will be unlocked.
    ///
    /// # Errors
    ///
    /// On Unix this may return an error if the operation was interrupted by a signal handler.
    #[inline]
    pub fn lock(&mut self) -> Result<FdLockGuard<'_, T>, Error> {
        // See: https://stackoverflow.com/a/9186532, https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfileex
        let handle = self.t.as_raw_handle();
        let overlapped = Overlapped::zero();
        let flags = LOCKFILE_EXCLUSIVE_LOCK;
        match unsafe { LockFileEx(handle, flags, 0, 1, 0, overlapped.raw()) } {
            0 => Err(ErrorKind::Other.into()),
            _ => Ok(FdLockGuard { lock: self }),
        }
    }

    /// Attempts to acquire this lock.
    ///
    /// Unlike `FdLock::lock` this function will never block, but instead will
    /// return an error if the lock cannot be acquired.
    ///
    /// # Errors
    ///
    /// If the lock is already held and `ErrorKind::WouldBlock` error is returned.
    #[inline]
    pub fn try_lock(&mut self) -> io::Result<FdLockGuard<'_, T>> {
        let handle = self.t.as_raw_handle();
        match unsafe { LockFile(handle, 0, 0, 1, 0) } {
            1 => Ok(FdLockGuard { lock: self }),
            _ => {
                let err = Error::last_os_error();
                Err(Error::new(ErrorKind::WouldBlock, format!("{}", err)))
            }
        }
    }
}

/// A wrapper around `OVERLAPPED` to provide "rustic" accessors and
/// initializers.
struct Overlapped(OVERLAPPED);

impl Overlapped {
    /// Creates a new zeroed out instance of an overlapped I/O tracking state.
    ///
    /// This is suitable for passing to methods which will then later get
    /// notified via an I/O Completion Port.
    fn zero() -> Overlapped {
        Overlapped(unsafe { mem::zeroed() })
    }

    /// Gain access to the raw underlying data
    fn raw(&self) -> *mut OVERLAPPED {
        &self.0 as *const _ as *mut _
    }
}
