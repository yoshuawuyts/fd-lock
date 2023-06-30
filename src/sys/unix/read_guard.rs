use rustix::fd::AsFd;
use rustix::fs::{flock, FlockOperation};
use std::ops;

use super::RwLock;

#[derive(Debug)]
pub struct RwLockReadGuard<'lock, T: AsFd> {
    lock: &'lock RwLock<T>,
}

impl<'lock, T: AsFd> RwLockReadGuard<'lock, T> {
    pub(crate) fn new(lock: &'lock RwLock<T>) -> Self {
        Self { lock }
    }
}

impl<T: AsFd> ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.inner
    }
}

impl<T: AsFd> Drop for RwLockReadGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let _ = flock(&self.lock.inner.as_fd(), FlockOperation::Unlock).ok();
    }
}
