use log::debug;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use structopt::StructOpt;
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
testing,
benchmark,
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
    // node_modules path
    #[structopt(short, long, default_value = "node_modules")]
    pub path: String,

    // verbose model
    #[structopt(short, long)]
    pub verbose: bool,
}

///
///
#[derive(Debug)]
pub struct Stats {
    pub files_total: u64,
    pub files_removed: u64,
    pub removed_size: u64,
    pub module_size: u64,
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
        let mut stats = Stats {
            files_total: 0,
            removed_size: 0,
            files_removed: 0,
            module_size: 0,
        };

        let mut walker = WalkDir::new(&self.dir).into_iter();
        loop {
            let entry = match walker.next() {
                Some(Ok(entry)) => entry,
                Some(Err(_)) => continue,
                None => break,
            };

            let filepath = entry.path();
            if !self.need_prune(&filepath) {
                continue;
            }

            if filepath.is_dir() {
                let s = dir_state(&filepath)?;
                stats.files_total += s.files_total;
                stats.files_removed += s.files_removed;
                stats.removed_size += s.removed_size;

                match fs::remove_dir_all(&filepath) {
                    Ok(_) => debug!("removed {}", filepath.display()),
                    Err(err) => {
                        debug!("removed dir {} failed with err {}", filepath.display(), err)
                    }
                };
                walker.skip_current_dir();
                continue;
            }

            stats.files_total += 1;
            stats.removed_size += entry.metadata().unwrap().len();

            match fs::remove_file(&filepath) {
                Ok(_) => debug!("removed {}", filepath.display()),
                Err(err) => debug!(
                    "removed file {} failed with err {}",
                    filepath.display(),
                    err
                ),
            }
        }

        stats.module_size = Path::new(&self.dir[..]).metadata().unwrap().len();
        Ok(stats)
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
        let metadata = entry.metadata()?;
        stats.files_total += 1;
        stats.files_removed += 1;
        if entry.path().is_file() {
            stats.removed_size += metadata.len();
        }
    }

    Ok(stats)
}

///
///
fn to_map(paths: &str) -> HashSet<String> {
    paths.split(",").map(|x| x.trim().to_string()).collect()
}
