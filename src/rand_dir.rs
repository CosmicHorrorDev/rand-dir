use crate::entry::{BrokenSymlink, Dir, Entry, File};

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use tempdir::TempDir;

#[derive(Debug)]
pub struct RandDir {
    root: TempDir,
    base: PathBuf,
}

impl RandDir {
    pub fn builder() -> RandDirBuilder {
        RandDirBuilder::default()
    }

    pub fn at(&self) -> &Path {
        &self.base
    }
}

#[derive(Debug, Clone)]
pub struct RandDirBuilder {
    inner: Dir,
}

impl Default for RandDirBuilder {
    fn default() -> Self {
        Self {
            inner: Dir::real().name("base"),
        }
    }
}

// TODO: allow for user to specify the prefix
impl RandDirBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dir(mut self, dir: Dir) -> Self {
        self.inner = self.inner.dir(dir);
        self
    }

    pub fn file(mut self, file: File) -> Self {
        self.inner = self.inner.file(file);
        self
    }

    pub fn broken_symlink(mut self, broken_symlink: BrokenSymlink) -> Self {
        self.inner = self.inner.broken_symlink(broken_symlink);
        self
    }

    // TODO: have some sort of filetree description object used for comparisons, or see if a there
    // is a good diff library that already covers that
    // The RandDir is laid out so there is:
    // <temp-dir> aka root
    // |- base (the specified contents)
    // |- symlinks (contains the directories pointed to by symlinks)
    // \- broken-symlinks (contains the non-existent destinations of broken symlinks)
    pub fn try_build(self) -> io::Result<RandDir> {
        // TODO: nest this one layer deeper so that we can guarantee places for the broken symlinks
        // to go
        // Create the general layout
        let temp_dir = TempDir::new("rand-dir-")?;
        let root = temp_dir.path();

        let symlinks = root.join("symlinks");
        let broken_symlinks = root.join("broken-symlinks");
        fs::create_dir(&symlinks)?;
        fs::create_dir(&broken_symlinks)?;

        // Generate all the entries
        // TODO: pass down properties for the entries to inherit from
        let base = root.join("base");
        self.inner.try_build_at(root, &symlinks, &broken_symlinks)?;

        Ok(RandDir {
            root: temp_dir,
            base,
        })
    }
}
