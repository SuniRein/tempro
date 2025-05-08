use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

static WORK_DIR_LOCK: Mutex<()> = Mutex::new(());

pub fn with_current_dir<P, F, R>(path: P, closure: F) -> R
where
    P: AsRef<Path>,
    F: FnOnce() -> R,
{
    let old_wd = RestoreWd::capture(WORK_DIR_LOCK.lock().unwrap());
    env::set_current_dir(&path)
        .unwrap_or_else(|err| panic!("failed to change cwd to {:?}: {err}", path.as_ref()));
    let result = closure();
    drop(old_wd);
    result
}

struct RestoreWd<'a> {
    original: PathBuf,
    _guard: MutexGuard<'a, ()>,
}

impl<'a> RestoreWd<'a> {
    fn capture(guard: MutexGuard<'a, ()>) -> Self {
        let original = env::current_dir().unwrap();
        Self {
            original,
            _guard: guard,
        }
    }
}

impl Drop for RestoreWd<'_> {
    fn drop(&mut self) {
        if let Err(e) = env::set_current_dir(&self.original) {
            eprintln!("failed to restore cwd to {:?}: {e}", self.original);
        }
    }
}
