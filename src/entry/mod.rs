mod broken_symlink;
mod dir;
mod file;

use once_cell::sync::Lazy;

pub use self::{broken_symlink::BrokenSymlink, dir::Dir, file::File};

use std::{
    cmp::Ordering,
    fs, io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

// TODO: maybe abstract out this counter pattern
static GLOBAL_ENTRY_UNIQIFIER: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(0)));

/// This type is very similar to a plain `Option<PathBuf>` __except__ that the `None` variant holds
/// a `u64` that is used to ensure uniqueness. This allows for implementing `Ord` or `Eq` which is
/// required since a directories contents are represented with a `Set`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EntryName {
    Set(PathBuf),
    Generated(u64),
}

impl EntryName {
    fn unwrap_or_else<F>(self, f: F) -> PathBuf
    where
        F: FnOnce() -> PathBuf,
    {
        match self {
            Self::Set(path) => path,
            Self::Generated(_) => f(),
        }
    }
}

impl Default for EntryName {
    fn default() -> Self {
        let mut entry_uniqifier = GLOBAL_ENTRY_UNIQIFIER.lock().unwrap();
        let current_val = *entry_uniqifier;
        *entry_uniqifier += 1;

        Self::Generated(current_val)
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct CommonProp {
    name: EntryName,
    permissions: Option<fs::Permissions>,
}

impl CommonProp {
    fn set_name(&mut self, name: impl Into<PathBuf>) {
        self.name = EntryName::Set(name.into());
    }

    fn set_permissions(&mut self, permissions: fs::Permissions) {
        self.permissions = Some(permissions);
    }
}

// TODO: can this be done with traits instead so that people aren't having to setup the enum
// variants or are there some limitations that I'm not thinking of
// TODO: could store common stuff in here instead of having it duplicated in all the entries. This
// does make the api a bit weirder unless we do delegate methods or something like that though
#[derive(Debug, Clone)]
pub enum Entry {
    Dir(Dir),
    File(File),
    BrokenSymlink(BrokenSymlink),
}

impl Entry {
    pub fn try_build_at(
        self,
        at: &Path,
        symlinks: &Path,
        broken_symlinks: &Path,
    ) -> io::Result<()> {
        match self {
            Entry::Dir(entry) => entry.try_build_at(at, symlinks, broken_symlinks),
            Entry::File(entry) => entry.try_build_at(at),
            Entry::BrokenSymlink(entry) => entry.try_build_at(at, broken_symlinks),
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        let extract_name = |entry: &Self| {
            let common_prop: &CommonProp = match entry {
                Self::Dir(inner) => &inner.common_prop,
                Self::File(inner) => &inner.common_prop,
                Self::BrokenSymlink(inner) => &inner.common_prop,
            };

            // TODO: get rid of this clone by just holding a reference
            common_prop.name.clone()
        };

        extract_name(self).cmp(&extract_name(other))
    }
}
