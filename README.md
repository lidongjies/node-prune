# node-prune
Remove unnecessary files from node_modules (.md .ts ...)

## guide

[cheat.rs](https://cheats.rs/)
[rust cargo book](https://doc.rust-lang.org/cargo/)
[rust document book](https://doc.rust-lang.org/stable/rustdoc/)
[rust command line book](https://rust-lang-nursery.github.io/cli-wg/);

## target

- read all files and directories
- delete all files and directories with mulit threads.

## Structures

### Prune

```rust
use std::spsc;
use std::sync::{ Arc, Mutex };
use std::collections::Map;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender,
}

struct Worker {
    id: usize,
    receiver: Arc<Mutex<mpsc::Receiver>
}

struct Stats {
    totalFiles: usize,
    totalSize: usize,
    filesRemoved: usize,
}

struct Config {
    dir: Path,
    verbose: bool,
    progress: bool,
}

struct Prune {
    dir: Path,
    log: String,
    dirs: Map<String>,
    exts: Map<String>,
    files: Map<String>,
}
```