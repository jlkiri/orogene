use oro_pack::*;
use std::env;
use std::path::Path;

#[test]
fn paths_no_files_field() -> std::io::Result<()> {
    let mut cwd = env::current_dir()?;
    cwd.push("fixtures/implicit_files");
    env::set_current_dir(cwd)?;

    let mut pack = OroPack::new();

    let mut expected_paths = vec![
        Path::new("README.md"),
        Path::new("package.json"),
        Path::new("src/index.js"),
        Path::new("src/module.js"),
    ];

    pack.load();

    let mut files = pack.project_paths();

    expected_paths.sort();
    files.sort();

    assert_eq!(expected_paths, files);

    Ok(())
}
