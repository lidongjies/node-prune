use atty::Stream;
use clap::Parser;
use log::{set_max_level, LevelFilter};
use node_prune::{Config, Prune, Stats};
use serde_json::json;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    set_max_level(LevelFilter::Warn);
    let now = Instant::now();

    let config = Config::parse();
    if config.verbose {
        set_max_level(LevelFilter::Debug);
    }

    let mut prune = Prune::new();
    if config.path.exists() {
        prune.dir = config.path;
    }

    let mut stats: Stats = Default::default();
    if prune.dir.exists() {
        stats = prune.run().await?;
    }

    if atty::is(Stream::Stdout) {
        println!();
        println!("\t files total: {}", stats.files_total);
        println!("\t files removed: {}", stats.files_removed);
        println!(
            "\t removed size: {:.1}M",
            (stats.removed_size as f64) / 1024f64 / 1024f64
        );
        println!("\t duration: {}ms", now.elapsed().as_millis());
        println!();
    } else {
        let json_stats = json!(&stats);
        println!("{}", json_stats);
    }

    Ok(())
}
