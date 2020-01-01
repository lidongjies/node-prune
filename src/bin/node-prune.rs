use node_prune::Prune;

fn main() {
    let prune = Prune::init();
    let result = prune.run();
    if let Ok(stats) = result {
        println!("{:?}", stats);
    }
}
