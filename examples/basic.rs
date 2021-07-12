use std::io;

use rand_dir::{Dir, Entry, File, RandDir};

pub fn main() {
    let kibibyte = 1_024;

    let rand_dir = RandDir::builder()
        .entry(Entry::Dir(
            Dir::real()
                .entry(Entry::File(File::zeroed().size(kibibyte)))
                .entry(Entry::File(File::oned().size(kibibyte))),
        ))
        .entry(Entry::Dir(
            Dir::symlink().entry(Entry::File(File::random().size(kibibyte))),
        ))
        .entry(Entry::File(File::custom(*b"Hello, World!")))
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
