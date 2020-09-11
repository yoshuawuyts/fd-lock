use fd_lock::{ErrorKind, FdLock};

use std::fs::File;

use tempfile::tempdir;

#[test]
fn double_lock() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("lockfile");

    let mut l0 = FdLock::new(File::create(&path).unwrap());
    let mut l1 = FdLock::new(File::open(path).unwrap());

    let g0 = l0.lock().unwrap();

    let err = l1.try_lock().unwrap_err();
    assert!(matches!(err.kind(), ErrorKind::Locked));

    drop(g0);
}
