/// Handles abstractions over filesystem entries
///
/// This whole module intends to contain as much internal complexity as possible from the user.
mod broken_symlink;
mod dir;
mod file;

use once_cell::sync::Lazy;

pub use self::{broken_symlink::BrokenSymlink, dir::Dir, file::File};
use crate::utils::next_global_counter;

use std::{
    cmp::Ordering,
    ffi::OsString,
    fs, io,
    path::Path,
    sync::{Arc, Mutex},
};

static GLOBAL_ENTRY_UNIQIFIER: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(0)));

/// This type is very similar to a plain `Option<PathBuf>` __except__ that the `None` variant holds
/// a `u64` that is used to ensure uniqueness. This allows for implementing `Ord` or `Eq` which is
/// required since a directories contents are represented with a `Set`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EntryName {
    Set(OsString),
    Generated(u64),
}

impl EntryName {
    fn unwrap_or_else(self, f: impl FnOnce() -> OsString) -> OsString {
        match self {
            Self::Set(s) => s,
            Self::Generated(_) => f(),
        }
    }
}

impl Default for EntryName {
    fn default() -> Self {
        let current_val = next_global_counter(&GLOBAL_ENTRY_UNIQIFIER);
        Self::Generated(current_val)
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct CommonProp {
    name: EntryName,
    permissions: Option<fs::Permissions>,
}

impl CommonProp {
    fn set_name(&mut self, name: impl Into<OsString>) {
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

fn extract_entrys_name(entry: &Entry) -> &EntryName {
    let common_prop = match entry {
        Entry::Dir(inner) => &inner.common_prop,
        Entry::File(inner) => &inner.common_prop,
        Entry::BrokenSymlink(inner) => &inner.common_prop,
    };

    &common_prop.name
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        extract_entrys_name(self).cmp(&extract_entrys_name(other))
    }
}
