use std::{ffi::OsString, fs, io, path::Path};

use crate::entry::CommonProp;

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
        todo!()
    }
}
