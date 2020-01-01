// use async_std::fs;
// use std::time::Duration;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use structopt::StructOpt;
use walkdir::WalkDir;

#[macro_use]
extern crate log;

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
#[derive(Debug, StructOpt)]
#[structopt(name = "node-prune")]
pub struct Config {
    // verbose model
    #[structopt(short, long)]
    verbose: bool,

    // show progress
    #[structopt(short, long)]
    progress: bool,
}

///
///
#[derive(Debug)]
pub struct Stats {
    pub files_total: i64,
    pub files_removed: i64,
    pub removed_size: i64,
    pub module_size: i64,
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
        env_logger::init();
        Self {
            dir: String::from("node_modules"),
            files: to_map(DEFAULT_FILES),
            dirs: to_map(DEFAULT_DIRS),
            exts: to_map(DEFAULT_EXTS),
        }
    }

    pub fn run(&self, _config: &Config) -> Result<Stats, std::io::Error> {
        let mut state = Stats {
            files_total: 0,
            removed_size: 0,
            files_removed: 0,
            module_size: 0,
        };
        for entry in WalkDir::new(&self.dir).into_iter().filter_map(|e| e.ok()) {
            let filepath = entry.path();
            if !self.need_prune(&filepath) {
                info!("skip {:?}", filepath.display());
            } else {
                state.files_total += 1;
                state.removed_size += 1;

                if filepath.is_dir() {
                    let s = dir_state(&filepath)?;
                    state.files_total += s.files_total;
                    state.files_removed += s.files_removed;
                    state.removed_size += s.removed_size;

                    match fs::remove_dir_all(&filepath) {
                        Ok(_) => info!("removed {}", filepath.display()),
                        Err(err) => {
                            println!("removed dir {} failed with err {}", filepath.display(), err)
                        }
                    };
                } else {
                    match fs::remove_file(&filepath) {
                        Ok(_) => info!("removed {}", filepath.display()),
                        Err(err) => println!(
                            "removed file {} failed with err {}",
                            filepath.display(),
                            err
                        ),
                    }
                }
            }
        }
        state.module_size = Path::new(&self.dir[..]).metadata().unwrap().len() as i64;
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
        files_total: 0,
        files_removed: 0,
        removed_size: 0,
        module_size: 0,
    };
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let filepath = entry.path();
        let metadata = std::fs::metadata(&filepath)?;
        let file_size = metadata.len() as i64;
        stats.files_removed += 1;
        stats.files_total += 1;
        stats.removed_size += file_size;
    }
    Ok(stats)
}

///
///
fn to_map(paths: &str) -> HashSet<String> {
    paths.split(",").map(|x| x.trim().to_string()).collect()
}
