mod universe;
mod attribute;
mod entity;
pub use entity::entity::Entity;

fn main() {
    let id = 0;
    let name = "something".to_string();
    let color = 0xFF0B10;

    let something = Entity::new(id, name, 0.5, 0.5, color);
    println!("Shunyata: influence simulation");
    println!("==============================");

    println!("Entity: {}", something);
}
