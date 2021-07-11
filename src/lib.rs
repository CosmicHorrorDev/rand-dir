use std::{fs, path::Path};

pub struct Root {
    entries: Vec<Entry>,
}

impl Root {
    pub fn new() -> Self {
        todo!()
    }

    pub fn entry(mut self, entry: Entry) -> Self {
        self.entries.push(entry);
        self
    }
}

// Move the name and permissions up here since it's common to all?
pub enum Entry {
    Dir(Dir),
    File(File),
    BrokenSymlink(BrokenSymlink),
}

pub struct Dir {
    kind: DirKind,
    prop: DirProp,
}

enum DirKind {
    Normal,
    Symlink,
}

#[derive(Default)]
struct DirProp {
    name: Option<String>,
    permissions: Option<fs::Permissions>,
    entries: Vec<Entry>,
}

impl Dir {
    fn _new(kind: DirKind) -> Self {
        Self {
            kind,
            prop: DirProp::default(),
        }
    }

    pub fn new() -> Self {
        Self::_new(DirKind::Normal)
    }

    pub fn symlink() -> Self {
        Self::_new(DirKind::Symlink)
    }

    pub fn name(mut self, name: String) -> Self {
        self.prop.name = Some(name);
        self
    }

    pub fn permissions(mut self, dir_permissions: fs::Permissions) -> Self {
        self.prop.permissions = Some(dir_permissions);
        self
    }

    pub fn entry(mut self, entry: Entry) -> Self {
        self.prop.entries.push(entry);
        self
    }

    fn try_build_at(self, at: &Path) -> Result<(), ()> {
        todo!()
    }
}

pub struct File {
    kind: FileKind,
    prop: FileProp,
}

enum FileKind {
    Zeroed,
    Oned,
    Random(Option<u64>),
    Custom(Box<dyn Iterator<Item = u8>>),
}

pub enum FileSize {
    Fixed(u64),
    Uniform(u64, u64),
}

#[derive(Default)]
struct FileProp {
    name: Option<String>,
    permissions: Option<fs::Permissions>,
    size: Option<FileSize>,
}

impl File {
    fn new(kind: FileKind) -> Self {
        Self {
            kind,
            prop: FileProp::default(),
        }
    }

    pub fn zeroed() -> Self {
        Self::new(FileKind::Zeroed)
    }

    pub fn oned() -> Self {
        Self::new(FileKind::Oned)
    }

    pub fn random() -> Self {
        Self::new(FileKind::Random(None))
    }

    pub fn random_with_size(seed: u64) -> Self {
        Self::new(FileKind::Random(Some(seed)))
    }

    pub fn custom(contents_iter: Box<dyn Iterator<Item = u8>>) -> Self {
        Self::new(FileKind::Custom(contents_iter))
    }

    pub fn permissions(mut self, file_permissions: fs::Permissions) -> Self {
        self.prop.permissions = Some(file_permissions);
        self
    }

    pub fn size(mut self, file_size: FileSize) -> Self {
        self.prop.size = Some(file_size);
        self
    }

    pub fn try_build_at(self, at: &Path) -> Result<(), ()> {
        todo!()
    }
}

pub struct BrokenSymlink {
    prop: BrokenSymlinkProp,
}

#[derive(Default)]
struct BrokenSymlinkProp {
    name: Option<String>,
    permissions: Option<fs::Permissions>,
}

impl BrokenSymlink {
    pub fn new() -> Self {
        Self {
            prop: BrokenSymlinkProp::default(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.prop.name = Some(name);
        self
    }

    pub fn permissions(mut self, broken_symlink_permissions: fs::Permissions) -> Self {
        self.prop.permissions = Some(broken_symlink_permissions);
        self
    }

    pub fn try_build_at(self, at: &Path) -> Result<(), ()> {
        todo!()
    }
}

fn goals() {
    let rand_dir = Root::new()
        .entry(Entry::Dir(
            Dir::new()
                .entry(Entry::File(File::random()))
                .entry(Entry::File(File::random())),
        ))
        .entry(Entry::File(File::random()));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
