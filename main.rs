// main.rs

mod embedding;
mod engine;
mod interpreter;

use engine::QuineEngine;

fn main() {
    println!("Initializing System `e`...");
    let mut engine = QuineEngine::new();
    engine.run();
    println!("System `e` has completed its cycle.");
}
