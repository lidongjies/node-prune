use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use log::debug;
use serde::Serialize;
use tokio::fs;
use walkdir::WalkDir;

/// default prune files
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

/// default prune directories
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

/// default prune extensions
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

#[derive(Parser)]
pub struct Config {
    #[clap(
        short = 'p',
        long = "path",
        parse(from_os_str),
        default_value = "node_modules"
    )]
    pub path: PathBuf,

    #[clap(short = 'v', long = "verbose")]
    pub verbose: bool,
}

#[derive(Debug, Serialize, Default)]
pub struct Stats {
    pub files_total: u64,
    pub files_removed: u64,
    pub removed_size: u64,
}

#[derive(Debug)]
pub struct Prune {
    pub dir: PathBuf,
    files: HashSet<String>,
    exts: HashSet<String>,
    dirs: HashSet<String>,
}

impl Prune {
    pub fn new() -> Self {
        Self {
            dir: PathBuf::from("node_modules"),
            files: split(DEFAULT_FILES),
            dirs: split(DEFAULT_DIRS),
            exts: split(DEFAULT_EXTS),
        }
    }

    pub async fn run(&self) -> Result<Stats> {
        let mut stats: Stats = Default::default();

        let mut walker = WalkDir::new(&self.dir).into_iter();
        loop {
            let entry = match walker.next() {
                Some(Ok(entry)) => entry,
                Some(Err(err)) => {
                    bail!("access {} error", err.path().unwrap().display())
                }
                None => break,
            };

            let filepath = entry.path();
            if !self.need_prune(filepath) {
                debug!("skip: {}", filepath.display());
                continue;
            }

            stats.files_total += 1;
            stats.removed_size += entry.metadata().unwrap().len();

            if filepath.is_dir() {
                let s = dir_stats(filepath)?;
                stats.files_total += s.files_total;
                stats.files_removed += s.files_removed;
                stats.removed_size += s.removed_size;

                fs::remove_dir_all(filepath)
                    .await
                    .with_context(|| format!("removing directory {}", filepath.display()))?;

                walker.skip_current_dir();
                continue;
            }

            fs::remove_file(filepath)
                .await
                .with_context(|| format!("removing file {}", filepath.display()))?;
        }

        Ok(stats)
    }

    /// is filepath need prune
    fn need_prune(&self, filepath: &Path) -> bool {
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

/// statistics file count, file size in given directory.
fn dir_stats(dir: &Path) -> Result<Stats, walkdir::Error> {
    let walker = WalkDir::new(dir).into_iter().filter_map(|e| e.ok());

    let mut stats: Stats = Default::default();

    for entry in walker {
        let metadata = entry.metadata()?;
        stats.files_total += 1;
        stats.files_removed += 1;
        stats.removed_size += metadata.len();
    }

    Ok(stats)
}

/// split string by comma
///
/// it will return `HashSet<String>`，items in HashSet has trimed
fn split(paths: &str) -> HashSet<String> {
    paths
        .split(",")
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::{dir_stats, split};
    use std::path::Path;

    #[test]
    fn dir_stats_happy_path() {
        let path = Path::new("src");
        let stats = dir_stats(path).unwrap();
        assert_eq!(stats.files_total, 4);
        assert_eq!(stats.files_removed, 4);
    }

    #[test]
    fn dir_not_exits() {
        let path = Path::new("not_exist");
        let stats = dir_stats(path).unwrap();
        assert_eq!(stats.files_removed, 0);
        assert_eq!(stats.files_total, 0);
        assert_eq!(stats.files_removed, 0);
    }

    #[test]
    fn split_happypath() {
        let paths = String::from("prettier,eslint,typescript,prettier");
        let files = split(&paths);
        assert_eq!(files.len(), 3);
    }

    #[test]
    fn split_string_with_consecutive_commas() {
        let paths = String::from("prettier,,");
        let files = split(&paths[..]);
        assert_eq!(files.len(), 1);
    }

    #[test]
    fn split_string_with_trim() {
        let paths = String::from(" prettier ,javascript es6");
        let files = split(&paths[..]);
        assert_eq!(files.len(), 2);
        assert!(files.contains("prettier"));
        assert!(files.contains("javascript es6"));
    }
}
