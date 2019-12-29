# node-prune

Remove unnecessary files from node_modules (.md .ts ...)

## guide

[cheat.rs](https://cheats.rs/)
[rust cargo book](https://doc.rust-lang.org/cargo/)
[rust document book](https://doc.rust-lang.org/stable/rustdoc/)
[rust command line book](https://rust-lang-nursery.github.io/cli-wg/)
[api-guidelines](https://rust-lang.github.io/api-guidelines/naming.html)

## target

delete all files and directories by async non blocking io.

1. 递归遍历 node_modules，如果文件夹或者文件需要删除，记录路径
2. 异步删除记录中的所有文件夹和文件，并记录文件数量和体积
3. 展示清除后 node_modules 大小，和第二步骤记录的信息

- tj 的 node-prune 没有展示 node_modules 前后的大小，我每次都要再去看当前 node_modules 的大小，所以应该实现第三步的功能
- 展示删除进度条，因为第一步已经记录了要删除的文件夹，所以可以默认展示进度条

## Structures

### Prune

```rust
use async_std::fs; // async-std filesystem module
use std::collections::Map;

struct Stats {
    total_files: usize,
    files_removed: usize,
    total_size: usize,
    duration: Duration,
}

struct Config {
    verbose: bool,
    progress: bool,
}

struct Prune {
    dir: PathBuf,
    log: None,
    dirs: Map<String, ()>,
    exts: Map<String, ()>,
    files: Map<String, ()>,
}
```
