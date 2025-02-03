use {
    anyhow::{Error, Result},
    culpa::throws,
    std::{
        fs::{self, DirEntry},
        path::Path,
    },
};

// one possible implementation of walking a directory only visiting files
#[throws]
pub fn visit_files<P: AsRef<Path>>(dir: P, cb: &mut dyn for<'r> FnMut(&'r DirEntry) -> Result<()>) {
    if dir.as_ref().is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_files(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
}
