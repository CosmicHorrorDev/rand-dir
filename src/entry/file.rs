use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use crate::entry::CommonProp;

use petname::petname;
use rand::{random, thread_rng, Rng};

#[derive(Default, Debug, Clone)]
pub struct File {
    kind: FileKind,
    common_prop: CommonProp,
    prop: FileProp,
}

#[derive(Debug, Clone)]
enum FileKind {
    Zeroed,
    Oned,
    Random(Option<u64>),
    Custom(Vec<u8>),
}

impl Default for FileKind {
    fn default() -> Self {
        Self::Random(None)
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct FileProp {
    size: Option<usize>,
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

    pub fn random_with_seed(seed: u64) -> Self {
        Self::new(FileKind::Random(Some(seed)))
    }

    pub fn custom(contents: impl Into<Vec<u8>>) -> Self {
        let contents = contents.into();
        let contents_len = contents.len();
        Self::new(FileKind::Custom(contents.to_owned())).size(contents_len)
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.common_prop.set_name(name);
        self
    }

    pub fn permissions(mut self, permissions: fs::Permissions) -> Self {
        self.common_prop.set_permissions(permissions);
        self
    }

    pub fn size(mut self, file_size: usize) -> Self {
        self.prop.size = Some(file_size);
        self
    }

    pub(super) fn try_build_at(self, at: &Path) -> io::Result<()> {
        let Self {
            kind,
            common_prop: CommonProp { name, .. },
            prop: FileProp { size: maybe_size },
        } = self;

        // Defaults the size to 4 KiB
        let size = maybe_size.unwrap_or(4_096);

        // Create the file and write the contents
        let file_name = name.unwrap_or_else(|| {
            let prefix = "file-";
            let suffix = match &kind {
                FileKind::Zeroed => ".zeroed",
                FileKind::Oned => ".oned",
                FileKind::Random(_) => ".random",
                FileKind::Custom(_) => ".custom",
            };
            format!("{}{}{}", prefix, petname(2, "-"), suffix)
        });

        // TODO: this can be more efficient, just need to find a good way to handle streaming data
        // from the iterator to the file
        let contents_iter: Box<dyn Iterator<Item = u8>> = match kind {
            FileKind::Zeroed => Box::new(std::iter::repeat(0x00)),
            FileKind::Oned => Box::new(std::iter::repeat(0xFF)),
            // TODO: look into rand::sample_iter
            FileKind::Random(maybe_seed) => {
                let seed = maybe_seed.unwrap_or_else(|| random());
                // TODO: use a seedable rng here so that we can actually seed this
                // TODO: look into accepting a mutable rng for generating the seed?
                Box::new(thread_rng().sample_iter(rand::distributions::Standard))
            }
            FileKind::Custom(custom_contents) => Box::new(custom_contents.into_iter()),
        };
        let contents: Vec<_> = contents_iter.take(size).collect();

        let file_loc = at.join(file_name);
        let mut file = fs::File::create(file_loc)?;
        file.write_all(&contents)?;

        Ok(())
    }
}
