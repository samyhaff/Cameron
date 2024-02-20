use clap::{arg, Command};
use cameron::notes::*;
use cameron::chords::*;
use cameron::scales::*;

fn main() {
    let matches = Command::new("cameron")
        .about("A multi-purpose cli music theory tool")
        .subcommand(
            Command::new("scale")
                .about("Displays he notes of a scale")
                .arg(arg!([SCALE]))
        )
        .subcommand(
            Command::new("chord")
                .about("Displays the notes of a chord")
                .arg(arg!([CHORD]))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("scale", scale_matches)) => {
            if let Some(scale) = scale_matches.get_one::<String>("SCALE") {
                if let Some(scale) = Scale::from_str(scale) {
                    let notes = scale.get_notes();
                    println!("{}", notes.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" "));
                }
                else {
                    println!("Invalid scale provided.");
                }
            }
            else {
                println!("No scale provided.");
            }
        }
        Some(("chord", chord_matches)) => {
            if let Some(chord) = chord_matches.get_one::<String>("CHORD") {
                if let Some(chord) = Chord::from_str(chord) {
                    let notes = chord.get_notes();
                    println!("{}", notes.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" "));
                }
                else {
                    println!("Invalid chord provided.");
                }
            }
            else {
                println!("No chord provided.");
            }
        }
        _ => {
            println!("No command provided.");
        }
    }
}
