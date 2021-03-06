# node-prune

Remove unnecessary files from node_modules (.md .ts ...)

## guide

- [cheat.rs](https://cheats.rs/)
- [rust cargo book](https://doc.rust-lang.org/cargo/)
- [rust document book](https://doc.rust-lang.org/stable/rustdoc/)
- [rust command line book](https://rust-lang-nursery.github.io/cli-wg/)
- [api-guidelines](https://rust-lang.github.io/api-guidelines/naming.html)

## Structures

### Prune

```rust
use std::fs;
use std::collections::HashSet;

struct Stats {
    total_files: i64,
    files_removed: i64,
    total_size: i64,
}

struct Config {
    verbose: bool,
    progress: bool,
}

struct Prune<'a> {
    dir: &<'a>Path,
    dirs: HashSet<String>,
    exts: HashSet<String>,
    files: HashSet<String>,
}
```

## Roadmap

### v0.0.1

- [x] first implementation
- [x] parsing command line arguments
- [x] add log
- [x] communicating with human
- [x] nicer error reporting
- [x] unit test TDD
- [x] add docs
- [x] cargo install
- [x] communicating with michines
- [x] intergation test
- [ ] bench mark

### v0.0.2

- [ ] async std
- [ ] bench mark
- [ ] release binary file
- [ ] system package manager

### v0.0.3

- [ ] threadpool
- [ ] signal handle
- [ ] exit code
