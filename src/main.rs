mod universe;
mod entity;
pub use entity::entity::Entity;

fn main() {
    let id = 0;
    let name = "something".to_string();
    let color = 0xFAFBFCu32;

    let something = Entity::new(id, name, 0.5, 0.5, color);
    println!("Shunyata: influence simulation");
    println!("==============================");

    println!("Entity: {}", something);
}
