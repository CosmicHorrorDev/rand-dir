mod broken_symlink;
mod dir;
mod file;

pub use self::{broken_symlink::BrokenSymlink, dir::Dir, file::File};

use std::{
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Default, Debug, Clone)]
struct CommonProp {
    name: Option<PathBuf>,
    permissions: Option<fs::Permissions>,
}

impl CommonProp {
    fn set_name(&mut self, name: impl Into<PathBuf>) {
        self.name = Some(name.into());
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
