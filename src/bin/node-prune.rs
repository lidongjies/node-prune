use log::{set_max_level, LevelFilter};
use node_prune::{Config, Prune};
use std::time::Instant;
use structopt::StructOpt;

const MB: f64 = 1024f64 * 1024f64;

// fn init() {
//     set_max_level(LevelFilter::Warn);
// }

fn main() {
    let now = Instant::now();
    let config = Config::from_args();
    if config.verbose {
        set_max_level(LevelFilter::Debug);
    }

    let prune = Prune::init();

    if let Ok(stats) = prune.run() {
        println!();
        println!("\t files total: {}", stats.files_total);
        println!("\t files removed: {}", stats.files_removed);
        println!("\t removed size: {:.2}M", (stats.removed_size as f64) / MB);
        println!("\t node_modules {:.2}M", (stats.module_size as f64) / MB);
        println!("\t duration: {}ms", now.elapsed().as_millis());
        println!();
    }
}
