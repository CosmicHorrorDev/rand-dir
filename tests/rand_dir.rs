use rand_dir::{Dir, File, RandDir};

#[test]
fn error_with_duplicate_entry_name() {
    let result = RandDir::builder()
        .file(File::random().name("duplicate"))
        .dir(Dir::real().name("duplicate"))
        .try_build();

    assert!(result.is_err());
}
