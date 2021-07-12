// TODO: use globally increasing counters for symlinks and broken symlinks

use std::{
    fmt, fs,
    io::{self, Write},
    os::unix::prelude::FileExt,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;
use petname::petname;
use tempdir::TempDir;

// Since the symlinks and broken symlinks all point to entries stored in the same directory so to
// prevent naming conflicts the entries are made unique with a globally incremented counter
static GLOBAL_SYMLINK_COUNTER: Lazy<Arc<Mutex<usize>>> = Lazy::new(|| Arc::new(Mutex::new(1)));
static GLOBAL_BROKEN_SYMLINK_COUNTER: usize = 1;

// TODO: setup properties that all entries would inherit from
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

// TODO: store an Rng here to propogate down to entries
#[derive(Default, Debug)]
pub struct RandDirBuilder {
    entries: Vec<Entry>,
}

// TODO: allow for user to specify the prefix
impl RandDirBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entry(mut self, entry: Entry) -> Self {
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

#[derive(Default, Debug, Clone)]
struct CommonProp {
    name: Option<String>,
    permissions: Option<fs::Permissions>,
}

// TODO: could store common stuff in here instead of having it duplicated in all the entries. This
// does make the api a bit weirder unless we do delegate methods or something like that though
#[derive(Debug, Clone)]
pub enum Entry {
    Dir(Dir),
    File(File),
    BrokenSymlink(BrokenSymlink),
}

impl Entry {
    fn try_build_at(self, at: &Path, symlinks: &Path, broken_symlinks: &Path) -> io::Result<()> {
        match self {
            Entry::Dir(entry) => entry.try_build_at(at, symlinks, broken_symlinks),
            Entry::File(entry) => entry.try_build_at(at),
            Entry::BrokenSymlink(entry) => entry.try_build_at(at, symlinks, broken_symlinks),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Dir {
    kind: DirKind,
    common_prop: CommonProp,
    prop: DirProp,
}

#[derive(Debug, Clone)]
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
    entries: Vec<Entry>,
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

    // TODO: actually set permissions if provided
    fn try_build_at(self, at: &Path, symlinks: &Path, broken_symlinks: &Path) -> io::Result<()> {
        let Self {
            common_prop: CommonProp { name, .. },
            kind,
            prop: DirProp { entries },
        } = self;

        // TODO: handle the case of a duplicate name
        let dir_name = name.unwrap_or_else(|| match kind {
            DirKind::Normal => format!("dir-{}", petname(3, "-")),
            DirKind::Symlink => {
                let mut symlink_counter = GLOBAL_SYMLINK_COUNTER.lock().unwrap();
                let current_val = *symlink_counter;
                *symlink_counter += 1;

                format!("symlink-dest-{}", current_val)
            }
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
            let link = at.join(&format!("symlink-{}", petname(3, "-")));
            std::os::unix::fs::symlink(original, link)?;
        }

        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
pub struct File {
    kind: FileKind,
    common_prop: CommonProp,
    prop: FileProp,
}

#[derive(Clone)]
enum FileKind {
    Zeroed,
    Oned,
    Random(Option<u64>),
    Custom(Vec<u8>),
}

// TODO: make this nicer
impl fmt::Debug for FileKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FileKind::Zeroed => "Zeroed",
            FileKind::Oned => "Oned",
            FileKind::Random(_) => "Random",
            FileKind::Custom(_) => "Custom",
        })
    }
}

impl Default for FileKind {
    fn default() -> Self {
        Self::Random(None)
    }
}

#[derive(Debug, Clone)]
pub enum FileSize {
    Fixed(usize),
    Uniform(usize, usize),
}

#[derive(Default, Debug, Clone)]
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

    // TODO: shortcut to setting the iter and size
    pub fn custom(contents: Vec<u8>) -> Self {
        todo!()
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

    fn try_build_at(self, at: &Path) -> io::Result<()> {
        let Self {
            kind,
            common_prop: CommonProp { name, .. },
            prop: FileProp { size: maybe_size },
        } = self;

        // Figure out the content size
        let size = maybe_size.unwrap_or_else(|| todo!());
        let contents_len = match size {
            FileSize::Fixed(len) => len,
            FileSize::Uniform(lower, upper) => todo!(),
        };

        // Create the file and write the contents
        let file_name = name.unwrap_or_else(|| {
            let prefix = "file-";
            let suffix = match &kind {
                FileKind::Zeroed => ".zeroed",
                FileKind::Oned => ".oned",
                FileKind::Random(_) => ".random",
                FileKind::Custom(_) => ".custom",
            };
            format!("{}{}{}", prefix, petname(3, "-"), suffix)
        });

        // TODO: this can be more efficient, just need to find a good way to handle streaming data
        // from the iterator to the file
        let contents_iter: Box<dyn Iterator<Item = u8>> = match kind {
            FileKind::Zeroed => Box::new(std::iter::repeat(0x00)),
            FileKind::Oned => Box::new(std::iter::repeat(0xFF)),
            // TODO: look into rand::sample_iter
            FileKind::Random(maybe_seed) => todo!(),
            FileKind::Custom(custom_contents) => Box::new(custom_contents.into_iter()),
        };
        let contents: Vec<_> = contents_iter.take(contents_len).collect();

        let file_loc = at.join(file_name);
        let mut file = fs::File::create(file_loc)?;
        file.write_all(&contents)?;

        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
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

    fn try_build_at(self, at: &Path, symlinks: &Path, broken_symlinks: &Path) -> io::Result<()> {
        todo!()
    }
}
