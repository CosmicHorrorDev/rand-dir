use std::{
    ffi::OsString,
    fs, io,
    path::Path,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;

use crate::{
    entry::CommonProp,
    utils::{gen_petname, next_global_counter},
};

// Same deal as the `crate::entry::dir::GLOBAL_SYMLINK_COUNTER`. This keeps broken symlinks
// pointing to unique locations
static GLOBAL_BROKEN_SYMLINK_COUNTER: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(1)));

#[derive(Default, Debug, Clone)]
pub struct BrokenSymlink {
    pub(crate) common_prop: CommonProp,
}

impl BrokenSymlink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<OsString>) -> Self {
        self.common_prop.set_name(name);
        self
    }

    pub fn permissions(mut self, permissions: fs::Permissions) -> Self {
        self.common_prop.set_permissions(permissions);
        self
    }

    pub(super) fn try_build_at(self, at: &Path, broken_symlinks: &Path) -> io::Result<()> {
        let Self {
            common_prop: CommonProp { name, permissions },
        } = self;

        let name = name.unwrap_or_else(|| format!("broken-symlink-{}", gen_petname()).into());
        let link = at.join(name);
        let unique = next_global_counter(&GLOBAL_BROKEN_SYMLINK_COUNTER);
        let dest = broken_symlinks.join(format!("broken-symlink-dest-{}", unique));

        #[cfg(unix)]
        std::os::unix::fs::symlink(dest, link)?;
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(dest, link)?;

        Ok(())
    }
}
