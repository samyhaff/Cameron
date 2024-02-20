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
                .arg(arg!([NOTE]))
        )
        .subcommand(
            Command::new("chord")
                .about("Displays the notes of a chord")
                .arg(arg!([CHORD]))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("scale", scale_matches)) => {
            if let Some(note) = scale_matches.get_one::<String>("NOTE") {
                if let Some(note) = Note::from_str(note) {
                    let notes = Scale::new(note, ScaleType::Major).get_notes();
                    println!("{}", notes.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" "));
                }
                else {
                    println!("Invalid note provided.");
                }
            }
            else {
                println!("No note provided.");
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
