# node-prune

Remove unnecessary files from node_modules (.md .ts ...)

## guide

[cheat.rs](https://cheats.rs/)
[rust cargo book](https://doc.rust-lang.org/cargo/)
[rust document book](https://doc.rust-lang.org/stable/rustdoc/)
[rust command line book](https://rust-lang-nursery.github.io/cli-wg/)
[api-guidelines](https://rust-lang.github.io/api-guidelines/naming.html)

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

struct Prune {
    dir: String,
    dirs: HashSet<String>,
    exts: HashSet<String>,
    files: HashSet<String>,
}
```

## Roadmap

- [x] first implementation
- [x] parsing command line arguments
- [x] add log
- [x] communicating with human
- [ ] better error handling
- [ ] unit test TDD
- [ ] bench mark
- [ ] async std
- [ ] threadpool + async std
- [ ] signal handle
- [ ] communicating with michines
- [ ] rending document
- [ ] cargo install
- [ ] release binary file
- [ ] system package manager
- [ ] other in command line book
- [ ] intergation test
- [ ] add process bar
