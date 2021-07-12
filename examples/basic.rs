use std::io;

use rand_dir::{Dir, Entry, File, FileSize, RandDir};

pub fn main() {
    let kibibyte = FileSize::Fixed(1_024);

    let rand_dir = RandDir::builder()
        .entry(Entry::Dir(
            Dir::real()
                .entry(Entry::File(File::zeroed().size(kibibyte.clone())))
                .entry(Entry::File(File::oned().size(kibibyte.clone()))),
        ))
        .entry(Entry::Dir(
            Dir::symlink().entry(Entry::File(File::random().size(kibibyte.clone()))),
        ))
        .entry(Entry::File(File::custom(b"Hello, World!".to_vec())))
        .try_build()
        .unwrap();

    println!(
        "You can look at the generated directory in: {:?}",
        rand_dir.at()
    );

    println!("Press <ENTER> to delete the generated directory...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}
