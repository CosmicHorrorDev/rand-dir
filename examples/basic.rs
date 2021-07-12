use rand_dir::{Dir, File, RandDir};

fn main() {
    #[rustfmt::skip]
    let rand_dir = RandDir::builder()
        .dir(
            Dir::real()
                .file(File::zeroed())
                .file(File::oned())
        )
        .dir(
            Dir::symlink()
                .file(File::random())
        )
        .file(File::custom(*b"Hello, World!"))
        .try_build()
        .unwrap();

    println!(
        "You can look at the generated directory in: {:?}",
        rand_dir.at()
    );

    println!("Press <ENTER> to delete the generated directory...");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
}
