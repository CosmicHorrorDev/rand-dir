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

/// Global counter that's bumped for added entries.
///
/// This allows for keeping ordering and comparisons of different entries even when the names are
/// being generated
static GLOBAL_ENTRY_UNIQIFIER: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(0)));

/// This type is very similar to a plain `Option<PathBuf>` __except__ that the `None` variant holds
/// a `u64` that is used to ensure uniqueness. This allows for implementing `Ord` or `Eq` which is
/// required since a directories contents are represented with a `Set`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EntryName {
    Set(OsString),
    Generated(u64),
}

// `Set` can be cloned normally, but `Generated` must use a new UUID to keep it unique
impl Clone for EntryName {
    fn clone(&self) -> Self {
        match self {
            Self::Set(name) => Self::Set(name.clone()),
            Self::Generated(_) => Self::default(),
        }
    }
}

impl Default for EntryName {
    fn default() -> Self {
        let current_val = next_global_counter(&GLOBAL_ENTRY_UNIQIFIER);
        Self::Generated(current_val)
    }
}

impl EntryName {
    fn unwrap_or_else(self, f: impl FnOnce() -> OsString) -> OsString {
        match self {
            Self::Set(s) => s,
            Self::Generated(_) => f(),
        }
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

    fn get_common_prop(&self) -> &CommonProp {
        match self {
            Entry::Dir(inner) => &inner.common_prop,
            Entry::File(inner) => &inner.common_prop,
            Entry::BrokenSymlink(inner) => &inner.common_prop,
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
        self.get_common_prop()
            .name
            .cmp(&other.get_common_prop().name)
    }
}
