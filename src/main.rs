mod strategies;
mod strategy;
mod game;

use strategy::Strategy;

fn main() {
    println!("🏁 Let the game begin!\n");

    println!("==============================");
    println!("          ROUND 1             ");
    println!("==============================\n");
    game::round1();
    println!("\n");

    // println!("==============================");
    // println!("          ROUND 2             ");
    // println!("==============================\n");
    // game::round2();
    // println!("\n");

    // println!("==============================");
    // println!("          ROUND 3             ");
    // println!("==============================\n");
    // game::round3();
    // println!("\n");

    println!("==============================");
}
