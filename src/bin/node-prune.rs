use node_prune::{Prune, Stats};

fn main() {
    let prune = Prune::init();
    // println!("{:?}", prune);

    let stats: Stats = prune.run();
    println!("{:?}", stats);
}
