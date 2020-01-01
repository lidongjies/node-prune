use node_prune::{Config, Prune};
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    let prune = Prune::init();
    let result = prune.run(&config);
    if let Ok(stats) = result {
        println!();
        println!("\t file removed: {}", stats.file_removed);
        println!("\t total_files: {}", stats.total_files);
        println!(
            "\t removed size: {:.4}M",
            (stats.file_size as f64) / 1024f64 / 1024f64
        );
        println!(
            "\t node_modules {:.4}M",
            (stats.module_size as f64) / 1024f64 / 1024f64
        );
        println!();
    }
}
