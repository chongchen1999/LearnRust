use clap::{Arg, Command};
use multi_toolkit::math::geometry;
use multi_toolkit::string_utils::parser;

fn main() {
    let matches = Command::new("Multi Toolkit CLI")
        .version("0.1.0")
        .author("Your Name")
        .about("Command-line interface for the toolkit")
        .arg(
            Arg::new("circle")
                .long("circle")
                .value_name("RADIUS")
                .help("Calculates the area of a circle")
                .num_args(1),
        )
        .arg(
            Arg::new("words")
                .long("words")
                .value_name("TEXT")
                .help("Counts the words in a text")
                .num_args(1),
        )
        .get_matches();

    if let Some(radius) = matches.get_one::<String>("circle") {
        let radius: f64 = radius.parse().unwrap_or(0.0);
        println!("Area of circle: {}", geometry::area_of_circle(radius));
    }

    if let Some(text) = matches.get_one::<String>("words") {
        println!("Word count: {}", parser::count_words(text));
    }
}
