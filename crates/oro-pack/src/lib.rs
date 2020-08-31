use ignore::{overrides::OverrideBuilder, WalkBuilder};
use oro_manifest::OroManifest;
use std::env;
use std::path::{Path, PathBuf};

const PKG_PATH: &str = "package.json";
const ALWAYS_IGNORED: [&str; 24] = [
    ".gitignore",
    ".npmignore",
    "**/.git",
    "**/.svn",
    "**/.hg",
    "**/CVS",
    "**/.git/**",
    "**/.svn/**",
    "**/.hg/**",
    "**/CVS/**",
    "/.lock-wscript",
    "/.wafpickle-*",
    "/build/config.gypi",
    "npm-debug.log",
    "**/.npmrc",
    ".*.swp",
    ".DS_Store",
    "**/.DS_Store/**",
    "._*",
    "**/._*/**",
    "*.orig",
    "/package-lock.json",
    "/yarn.lock",
    "/archived-packages/**",
];

fn read_package_json<P: AsRef<Path>>(pkg_path: P) -> OroManifest {
    match OroManifest::from_file(pkg_path) {
        Ok(pkg) => pkg,
        Err(e) => panic!("Problem loading package.json: {:?}", e),
    }
}

pub struct OroPack {
    pkg: Option<OroManifest>,
}

impl Default for OroPack {
    fn default() -> Self {
        Self::new()
    }
}

impl OroPack {
    pub fn new() -> Self {
        OroPack { pkg: None }
    }

    pub fn project_paths(&self) -> Vec<PathBuf> {
        let pkg_files = self.pkg_files();
        let cwd = env::current_dir().unwrap();

        let mut overd = OverrideBuilder::new(&cwd);

        if !pkg_files.is_empty() {
            for f in pkg_files {
                overd.add(f).unwrap();
            }
        }

        for ig in ALWAYS_IGNORED.iter() {
            let rev = format!("!{}", ig);
            overd.add(&rev).unwrap();
        }

        let mut paths = Vec::new();

        for path in WalkBuilder::new(&cwd)
            .overrides(overd.build().unwrap())
            .build()
        {
            if let Ok(entry) = path {
                paths.push(entry.path().to_owned());
            }
        }

        paths
            .iter()
            .filter(|f| !f.is_dir())
            .map(|p| p.strip_prefix(&cwd).unwrap().to_path_buf())
            .collect()
    }

    pub fn load(&mut self) {
        let mut path = env::current_dir().unwrap();

        path.push(PKG_PATH);

        self.pkg = Some(read_package_json(path));
    }

    fn pkg_files(&self) -> &Vec<String> {
        let pkg = self.pkg.as_ref().unwrap();

        &pkg.files
    }
}
