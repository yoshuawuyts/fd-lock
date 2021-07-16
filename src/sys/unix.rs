use libc::{flock, LOCK_EX, LOCK_NB, LOCK_UN};
use std::io::{self, Error, ErrorKind};
use std::ops;
use std::os::unix::io::AsRawFd;

/// A guard that unlocks the file descriptor when it goes out of scope.
///
/// # Panics
///
/// Dropping this type may panic if the lock fails to unlock.
#[derive(Debug)]
pub struct FileLockGuard<'FileLock, T: AsRawFd> {
    lock: &'FileLock mut FileLock<T>,
}

impl<T: AsRawFd> ops::Deref for FileLockGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.t
    }
}

impl<T: AsRawFd> ops::DerefMut for FileLockGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock.t
    }
}

impl<T: AsRawFd> Drop for FileLockGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let fd = self.lock.t.as_raw_fd();
        if unsafe { flock(fd, LOCK_UN | LOCK_NB) } != 0 {
            panic!("Could not unlock the file descriptor");
        }
    }
}

/// A file descriptor lock.
#[derive(Debug)]
pub struct FileLock<T: AsRawFd> {
    t: T,
}

impl<T: AsRawFd> FileLock<T> {
    /// Create a new instance.
    #[inline]
    pub fn new(t: T) -> Self {
        FileLock { t }
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
    pub fn lock(&mut self) -> io::Result<FileLockGuard<'_, T>> {
        let fd = self.t.as_raw_fd();
        match unsafe { flock(fd, LOCK_EX) } {
            0 => Ok(FileLockGuard { lock: self }),
            _ => Err(Error::last_os_error()),
        }
    }

    /// Attempts to acquire an advisory lock.
    ///
    /// Unlike `FileLock::lock` this function will never block, but instead will
    /// return an error if the lock cannot be acquired.
    ///
    /// # Errors
    ///
    /// If the lock is already held and `ErrorKind::WouldBlock` error is
    /// returned. This may also return an error if the operation was interrupted
    /// by a signal handler.
    #[inline]
    pub fn try_lock(&mut self) -> Result<FileLockGuard<'_, T>, Error> {
        let fd = self.t.as_raw_fd();
        match unsafe { flock(fd, LOCK_EX | LOCK_NB) } {
            0 => Ok(FileLockGuard { lock: self }),
            _ => match Error::last_os_error().kind() {
                ErrorKind::AlreadyExists | ErrorKind::WouldBlock => {
                    Err(ErrorKind::WouldBlock.into())
                }
                kind => Err(kind.into()),
            },
        }
    }
}
