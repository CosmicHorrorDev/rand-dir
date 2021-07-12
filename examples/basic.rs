use rand_dir::{Dir, Entry, File, FileSize, RandDir};

pub fn main() {
    let kibibyte_zeroed_file = Entry::File(File::zeroed().size(FileSize::Fixed(1_024)));

    let rand_dir = RandDir::builder()
        .entry(Entry::Dir(
            Dir::new()
                .entry(kibibyte_zeroed_file.clone())
                .entry(kibibyte_zeroed_file.clone()),
        ))
        .entry(Entry::Dir(
            Dir::symlink().entry(kibibyte_zeroed_file.clone()),
        ))
        .entry(kibibyte_zeroed_file)
        .try_build();

    std::thread::sleep_ms(30_000);

    println!("{:#?}", rand_dir);
}
