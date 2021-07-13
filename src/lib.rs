mod entry;

use self::entry::Entry;
pub use self::entry::{BrokenSymlink, Dir, File};

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

#[derive(Default, Debug, Clone)]
pub struct RandDirBuilder {
    entries: Vec<Entry>,
}

// TODO: allow for user to specify the prefix
impl RandDirBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO: can portions of this be deduped by calling out to `Dir` stuff?
    pub fn dir(self, dir: Dir) -> Self {
        self.entry(Entry::Dir(dir))
    }

    pub fn file(self, file: File) -> Self {
        self.entry(Entry::File(file))
    }

    pub fn broken_symlink(self, broken_symlink: BrokenSymlink) -> Self {
        self.entry(Entry::BrokenSymlink(broken_symlink))
    }

    fn entry(mut self, entry: Entry) -> Self {
        self.entries.push(entry);
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

        let base = root.join("base");
        let symlinks = root.join("symlinks");
        let broken_symlinks = root.join("broken-symlinks");
        fs::create_dir(&base)?;
        fs::create_dir(&symlinks)?;
        fs::create_dir(&broken_symlinks)?;

        // Generate all the entries
        // TODO: pass down properties for the entries to inherit from
        for entry in self.entries {
            entry.try_build_at(&base, &symlinks, &broken_symlinks)?;
        }

        Ok(RandDir {
            root: temp_dir,
            base,
        })
    }
}
