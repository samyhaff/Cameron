use cameron::*;

fn main() {
    let notes = [
        "C", "C#", "Db", "D", "D#", "Eb",
        "E", "F", "F#", "Gb", "G", "G#",
        "Ab", "A", "A#", "Bb", "B", "Cb",
    ];
    for note in notes {
        let root = Note::from_str(note).unwrap();
        let chord = Chord::new(&root, ChordQuality::Major);
        println!("{}: {}", chord, chord.get_notes().iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", "));
        let chord = Chord::new(&root, ChordQuality::Minor);
        println!("{}: {}", chord, chord.get_notes().iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", "));
    }
}
