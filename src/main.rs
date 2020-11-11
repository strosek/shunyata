mod entity;
mod math;
mod universe;

use clap::{App, Arg, ArgMatches};
pub use universe::universe::Universe;

fn get_options() -> ArgMatches {
    let matches = App::new("Shunyata Simulator")
        .version("0.1.0")
        .author("Erick Corona. <edcoronag@gmail.com>")
        .about("Simulate the influence of entities, or beings, with others. Influence is targeted to solve something.")
        .arg(Arg::new("config_file")
            .short('c')
            .long("config-file")
            .value_name("FILE")
            .help_heading(Option::from("Set a custom config file"))
            .takes_value(true))
        .arg(Arg::new("output_csv")
            .short('o')
            .long("output-csv")
            .value_name("FILE")
            .help_heading(Option::from("Set a custom CSV output file"))
            .takes_value(true))
        .get_matches();

    matches
}

fn main() {
    let option_matches = get_options();

    println!("Shunyata: influence simulation");
    println!("==============================");

    let mut universe = Universe::from_config(
        option_matches
            .value_of("config_file")
            .unwrap_or("resources/universe_cfg.json"),
    );

    universe.spawn();

    println!("All done!");
}
