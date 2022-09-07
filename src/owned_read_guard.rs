use std::{ops, sync::Arc};

use crate::{sys, RwLock};

/// Onwed version of `RwLockReadGuard`
///
/// # Panics
///
/// Dropping this type may panic if the lock fails to unlock.
#[must_use = "if unused the RwLock will immediately unlock"]
#[derive(Debug)]
pub struct OwnedRwLockReadGuard<T: sys::AsRaw> {
    lock: Arc<RwLock<T>>,
}

impl<T: sys::AsRaw> OwnedRwLockReadGuard<T> {
    pub(crate) fn new(lock: Arc<RwLock<T>>) -> Self {
        Self { lock }
    }
}

impl<T: sys::AsRaw> ops::Deref for OwnedRwLockReadGuard<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.lock.inner
    }
}

/// Release the lock.
impl<T: sys::AsRaw> Drop for OwnedRwLockReadGuard<T> {
    #[inline]
    fn drop(&mut self) {
        let _ = self.lock.lock.release_lock();
    }
}
