use fd_lock::FdLock;
use std::fs::File;
use std::io::ErrorKind;

use tempfile::tempdir;

#[test]
fn double_lock() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("lockfile");

    let mut l0 = FdLock::new(File::create(&path).unwrap());
    let mut l1 = FdLock::new(File::open(path).unwrap());

    let g0 = l0.try_lock().unwrap();

    let err = l1.try_lock().unwrap_err();
    assert!(matches!(dbg!(err.kind()), ErrorKind::AlreadyExists));

    drop(g0);
}
