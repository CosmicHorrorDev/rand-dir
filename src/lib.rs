use std::{fs, path::Path};

// TODO: setup traits that all entries would inherit from
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

#[derive(Default)]
struct CommonProp {
    name: Option<String>,
    permissions: Option<fs::Permissions>,
}

pub enum Entry {
    Dir(Dir),
    File(File),
    BrokenSymlink(BrokenSymlink),
}

#[derive(Default)]
pub struct Dir {
    kind: DirKind,
    common_prop: CommonProp,
    prop: DirProp,
}

enum DirKind {
    Normal,
    Symlink,
}

impl Default for DirKind {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
struct DirProp {
    entries: Vec<Entry>,
}

impl Dir {
    fn _new(kind: DirKind) -> Self {
        Self {
            kind,
            ..Self::default()
        }
    }

    pub fn new() -> Self {
        Self::_new(DirKind::Normal)
    }

    pub fn symlink() -> Self {
        Self::_new(DirKind::Symlink)
    }

    pub fn name(mut self, name: String) -> Self {
        self.common_prop.name = Some(name);
        self
    }

    pub fn permissions(mut self, dir_permissions: fs::Permissions) -> Self {
        self.common_prop.permissions = Some(dir_permissions);
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

#[derive(Default)]
pub struct File {
    kind: FileKind,
    common_prop: CommonProp,
    prop: FileProp,
}

enum FileKind {
    Zeroed,
    Oned,
    Random(Option<u64>),
    Custom(Box<dyn Iterator<Item = u8>>),
}

impl Default for FileKind {
    fn default() -> Self {
        Self::Random(None)
    }
}

pub enum FileSize {
    Fixed(u64),
    Uniform(u64, u64),
}

#[derive(Default)]
struct FileProp {
    size: Option<FileSize>,
}

impl File {
    fn new(kind: FileKind) -> Self {
        Self {
            kind,
            ..Self::default()
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

    pub fn name(mut self, name: String) -> Self {
        self.common_prop.name = Some(name);
        self
    }

    pub fn permissions(mut self, file_permissions: fs::Permissions) -> Self {
        self.common_prop.permissions = Some(file_permissions);
        self
    }

    pub fn size(mut self, file_size: FileSize) -> Self {
        self.prop.size = Some(file_size);
        self
    }

    fn try_build_at(self, at: &Path) -> Result<(), ()> {
        todo!()
    }
}

#[derive(Default)]
pub struct BrokenSymlink {
    common_prop: CommonProp,
}

impl BrokenSymlink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.common_prop.name = Some(name);
        self
    }

    pub fn permissions(mut self, broken_symlink_permissions: fs::Permissions) -> Self {
        self.common_prop.permissions = Some(broken_symlink_permissions);
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
