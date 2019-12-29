use async_std::fs;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

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
.markdown,
.md,
.mkd,
.ts,
.jst,
.coffee,
.tgz,
.swp,
";

///
///
#[derive(Debug)]
pub struct Stats {
    total_files: usize,
    file_removed: usize,
    file_size: usize,
    duration: Duration,
}

///
///
#[derive(Debug)]
pub struct Prune {
    dir: PathBuf,
    files: HashSet<String>,
    exts: HashSet<String>,
    dirs: HashSet<String>,
}

///
///
impl Prune {
    pub fn init() -> Self {
        Self {
            dir: PathBuf::from("node_modules"),
            files: to_map(DEFAULT_FILES),
            dirs: to_map(DEFAULT_DIRS),
            exts: to_map(DEFAULT_EXTS),
        }
    }

    pub fn run(&self) -> Stats {
        Stats {
            total_files: 10,
            file_size: 10,
            file_removed: 9,
            duration: Duration::from_secs(1),
        }
    }

    fn prune(&self, filepath: &Path) -> bool {
        prune(filepath)
    }
}

///
///
fn prune(filepath: &Path) -> bool {
    true
}

///
///
fn to_map(paths: &str) -> HashSet<String> {
    paths.split(",").map(|x| x.trim().to_string()).collect()
}
