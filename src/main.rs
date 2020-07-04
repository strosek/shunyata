use Entity::Entity;

fn main() {
    let id = 0;
    let name = "Perro";
    let color = 0xFF0B10;

    let something: Entity::Entity(id, name, color);
    println!("Hello, world!");
}
