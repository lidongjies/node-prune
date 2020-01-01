// use async_std::fs;
// use std::time::Duration;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const DEFAULT_FILES: &'static str = r"
Jenkinsfile,
Makefile,
Gulpfile.js,
Gruntfile.js,
gulpfile.js,
.DS_Store,
.tern-project,
.gitattributes,
.editorconfig,
.eslintrc,
eslint,
.eslintrc.js,
.eslintrc.json,
.eslintrc.yml,
.eslintignore,
.stylelintrc,
stylelint.config.js,
.stylelintrc.json,
.stylelintrc.yaml,
.stylelintrc.yml,
.stylelintrc.js,
.htmllintrc,
htmllint.js,
.lint,
.npmrc,
.npmignore,
.jshintrc,
.flowconfig,
.documentup.json,
.yarn-metadata.json,
.travis.yml,
appveyor.yml,
.gitlab-ci.yml,
circle.yml,
.coveralls.yml,
CHANGES,
changelog,
License,
LICENSE.txt,
LICENSE,
LICENSE-MIT,
LICENSE.BSD,
license,
LICENCE.txt,
LICENCE,
LICENCE-MIT,
LICENCE.BSD,
licence,
AUTHORS,
CONTRIBUTORS,
.yarn-integrity,
.yarnclean,
_config.yml,
.babelrc,
.yo-rc.json,
jest.config.js,
karma.conf.js,
wallaby.js,
wallaby.conf.js,
.prettierrc,
.prettierrc.yml,
.prettierrc.toml,
.prettierrc.js,
.prettierrc.json,
prettier.config.js,
.appveyor.yml,
tsconfig.json,
tslint.json,
";

const DEFAULT_DIRS: &'static str = r"
__tests__,
test,
tests,
powered-test,
docs,
doc,
.idea,
.vscode,
website,
images,
assets,
example,
examples,
coverage,
.nyc_output,
.circleci,
.github,
";

const DEFAULT_EXTS: &'static str = r"
markdown,
md,
mkd,
ts,
jst,
coffee,
tgz,
swp,
";

///
///
#[derive(Debug)]
pub struct Stats {
    total_files: u32,
    file_removed: u32,
    file_size: u64,
}

///
///
#[derive(Debug)]
pub struct Prune {
    dir: String,
    files: HashSet<String>,
    exts: HashSet<String>,
    dirs: HashSet<String>,
}

///
///
impl Prune {
    pub fn init() -> Self {
        Self {
            dir: String::from("node_modules"),
            files: to_map(DEFAULT_FILES),
            dirs: to_map(DEFAULT_DIRS),
            exts: to_map(DEFAULT_EXTS),
        }
    }

    pub fn run(&self) -> Result<Stats, std::io::Error> {
        let mut state = Stats {
            total_files: 0,
            file_size: 0,
            file_removed: 0,
        };
        for entry in WalkDir::new(&self.dir).into_iter().filter_map(|e| e.ok()) {
            let filepath = entry.path();
            if !self.need_prune(&filepath) {
                println!("skip {:?}", filepath.display());
            } else {
                state.total_files += 1;
                state.file_size += 1;

                if filepath.is_dir() {
                    let s = dir_state(&filepath)?;
                    state.total_files += s.total_files;
                    state.file_removed += s.file_removed;
                    state.file_size += s.file_size;

                    match fs::remove_dir_all(&filepath) {
                        Ok(_) => println!("removed {}", filepath.display()),
                        Err(err) => {
                            println!("removed dir {} failed with err {}", filepath.display(), err)
                        }
                    };
                } else {
                    match fs::remove_file(&filepath) {
                        Ok(_) => println!("removed {}", filepath.display()),
                        Err(err) => println!(
                            "removed file {} failed with err {}",
                            filepath.display(),
                            err
                        ),
                    }
                }
            }
        }
        Ok(state)
    }

    pub fn need_prune(&self, filepath: &Path) -> bool {
        let filename = filepath.file_name().unwrap().to_str().unwrap();

        if filepath.is_dir() {
            return self.dirs.contains(filename);
        }
        if self.files.contains(filename) {
            return true;
        }

        if let Some(extension) = filepath.extension() {
            let ext = extension.to_str().unwrap();
            if self.exts.contains(ext) {
                return true;
            }
        }

        false
    }
}

///
///
fn dir_state(dir: &Path) -> Result<Stats, std::io::Error> {
    let mut stats = Stats {
        file_removed: 0,
        file_size: 0,
        total_files: 0,
    };
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let filepath = entry.path();
        let metadata = std::fs::metadata(&filepath)?;
        let file_size = metadata.len() as u64;
        stats.file_removed += 1;
        stats.file_size += file_size;
        stats.total_files += 1;
    }
    Ok(stats)
}

///
///
fn to_map(paths: &str) -> HashSet<String> {
    paths.split(",").map(|x| x.trim().to_string()).collect()
}
