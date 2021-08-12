use rand_dir::{Dir, File, RandDir};

use std::path::Path;

use dir_diff::is_different;

// Snapshot test that compares the directory to a correct one stored under `tests/assets`
#[test]
fn it_works() {
    let ideal_dir = Path::new("tests")
        .join("assets")
        .join("it_works_ideal")
        .join("base");

    let rand_dir = RandDir::builder()
        .dir(Dir::real().name("Outer dir"))
        .file(File::zeroed().name("Outer file").size(128))
        .try_build()
        .unwrap();

    assert!(!is_different(rand_dir.at(), ideal_dir).unwrap());
}

#[test]
fn error_with_duplicate_entry_name() {
    let result = RandDir::builder()
        .file(File::random().name("duplicate"))
        .dir(Dir::real().name("duplicate"))
        .try_build();

    assert!(result.is_err());
}
