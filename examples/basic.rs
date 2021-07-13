use rand_dir::{Dir, File, RandDir};

fn main() -> std::io::Result<()> {
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
        .try_build()?;

    println!("Take a look at the directory: {:?}", rand_dir.at());
    println!("Press <ENTER> to delete the directory...");
    std::io::stdin().read_line(&mut String::new())?;

    Ok(())
}
