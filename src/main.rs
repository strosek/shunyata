mod entity;
mod universe;
pub use entity::entity::Entity;
use serde_json;
use std::string;
pub use universe::universe::Universe;

fn main() {
    println!("Shunyata: influence simulation");
    println!("==============================");

    let mut universe = Universe::new(string::String::from("somewhere"), 32usize);
    universe.spawn("resources/universe_cfg.json");
    let ser_universe = serde_json::to_string(&universe).unwrap();
    println!("{}", ser_universe);
}
