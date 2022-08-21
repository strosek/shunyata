mod entity;
mod math;
mod universe;

pub use universe::run_simulation;

fn main() {
    println!("Shunyata: influence simulator");
    println!("=============================\n");

    run_simulation();

    println!("\nAll done!")
}
