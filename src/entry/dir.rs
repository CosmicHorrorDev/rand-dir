use std::{
    collections::BTreeSet,
    ffi::OsString,
    fs, io,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{
    entry::{BrokenSymlink, CommonProp, Entry, File},
    utils::{gen_petname, next_global_counter},
};

use once_cell::sync::Lazy;

// Since the symlinks and broken symlinks all point to entries stored in the same directory so to
// prevent naming conflicts the entries are made unique with a globally incremented counter
static GLOBAL_SYMLINK_COUNTER: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(1)));

#[derive(Default, Debug, Clone)]
pub struct Dir {
    kind: DirKind,
    pub(crate) common_prop: CommonProp,
    prop: DirProp,
}

#[derive(Debug, Clone, Copy)]
enum DirKind {
    Normal,
    Symlink,
}

impl Default for DirKind {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default, Debug, Clone)]
struct DirProp {
    entries: BTreeSet<Entry>,
}

impl Dir {
    fn new(kind: DirKind) -> Self {
        Self {
            kind,
            ..Self::default()
        }
    }

    pub fn real() -> Self {
        Self::new(DirKind::Normal)
    }

    pub fn symlink() -> Self {
        Self::new(DirKind::Symlink)
    }

    pub fn name(mut self, name: impl Into<OsString>) -> Self {
        self.common_prop.set_name(name);
        self
    }

    pub fn permissions(mut self, permissions: fs::Permissions) -> Self {
        self.common_prop.set_permissions(permissions);
        self
    }

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
        self.prop.entries.insert(entry);
        self
    }

    // TODO: actually set permissions if provided
    pub(super) fn try_build_at(
        self,
        at: &Path,
        symlinks: &Path,
        broken_symlinks: &Path,
    ) -> io::Result<()> {
        let Self {
            common_prop: CommonProp { name, .. },
            kind,
            prop: DirProp { entries },
        } = self;

        // TODO: handle the case of a duplicate name
        let dir_name = name.unwrap_or_else(|| {
            match kind {
                DirKind::Normal => format!("dir-{}", gen_petname()),
                DirKind::Symlink => {
                    let current_val = next_global_counter(&GLOBAL_SYMLINK_COUNTER);
                    format!("symlink-dest-{}", current_val)
                }
            }
            .into()
        });

        // A regular directory will just have it's contents made in `at` while a symlink will have
        // it's contents in a destination directory in `symlinks`
        let dir_loc = match kind {
            DirKind::Normal => at,
            DirKind::Symlink => symlinks,
        }
        .join(&dir_name);
        fs::create_dir(&dir_loc)?;

        // Build all the entries
        for entry in entries {
            entry.try_build_at(&dir_loc, symlinks, broken_symlinks)?;
        }

        // TODO: have this support windows too
        // Now actually create the symlink
        if let DirKind::Symlink = kind {
            let original = dir_loc;
            let link = at.join(&format!("symlink-{}", gen_petname()));
            std::os::unix::fs::symlink(original, link)?;
        }

        Ok(())
    }
}
