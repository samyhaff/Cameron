use std::collections::HashSet;
use std::fmt;
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::notes::*;

#[derive(Debug, Clone, PartialEq, Hash, Eq, EnumIter)]
pub enum ChordQuality {
    Major,
    Minor,
    DominantSeventh,
    MajorSeventh,
    MinorSeventh,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Chord {
    root: Note,
    quality: ChordQuality,
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let quality = match self.quality {
            ChordQuality::Major => "",
            ChordQuality::Minor => "m",
            ChordQuality::DominantSeventh => "7",
            ChordQuality::MajorSeventh => "maj7",
            ChordQuality::MinorSeventh => "m7",
        };
        write!(f, "{}{}", self.root, quality)
    }
}

impl Chord {
    pub fn new(root: Note, quality: ChordQuality) -> Chord {
        Chord { root, quality }
    }

    pub fn from_str(s: &str) -> Option<Chord> {
        let re = Regex::new(r"([A-Ga-g][#b]?)((?:maj7|m7|7|m)?)").unwrap();
        let caps = re.captures(s)?;
        let root = Note::from_str(caps.get(1)?.as_str())?;
        let quality = match caps.get(2)?.as_str() {
            "maj7" => ChordQuality::MajorSeventh,
            "m7" => ChordQuality::MinorSeventh,
            "7" => ChordQuality::DominantSeventh,
            "m" => ChordQuality::Minor,
            _ => ChordQuality::Major,
        };
        Some(Chord::new(root, quality))
    }

    pub fn get_notes(&self) -> Vec<Note> {
        match self.quality {
            ChordQuality::Major => {
                let major_third = self.root.up_interval(Interval::new(IntervalQuality::Major, 3));
                let perfect_fifth = self.root.up_interval(Interval::new(IntervalQuality::Perfect, 5));
                vec![self.root.clone(), major_third, perfect_fifth]
            },
            ChordQuality::Minor => {
                let minor_third = self.root.up_interval(Interval::new(IntervalQuality::Minor, 3));
                let perfect_fifth = self.root.up_interval(Interval::new(IntervalQuality::Perfect, 5));
                vec![self.root.clone(), minor_third, perfect_fifth]
            },
            ChordQuality::DominantSeventh => {
                let major_third = self.root.up_interval(Interval::new(IntervalQuality::Major, 3));
                let perfect_fifth = self.root.up_interval(Interval::new(IntervalQuality::Perfect, 5));
                let minor_seventh = self.root.up_interval(Interval::new(IntervalQuality::Minor, 7));
                vec![self.root.clone(), major_third, perfect_fifth, minor_seventh]
            },
            ChordQuality::MajorSeventh => {
                let major_third = self.root.up_interval(Interval::new(IntervalQuality::Major, 3));
                let perfect_fifth = self.root.up_interval(Interval::new(IntervalQuality::Perfect, 5));
                let major_seventh = self.root.up_interval(Interval::new(IntervalQuality::Major, 7));
                vec![self.root.clone(), major_third, perfect_fifth, major_seventh]
            },
            ChordQuality::MinorSeventh => {
                let minor_third = self.root.up_interval(Interval::new(IntervalQuality::Minor, 3));
                let perfect_fifth = self.root.up_interval(Interval::new(IntervalQuality::Perfect, 5));
                let minor_seventh = self.root.up_interval(Interval::new(IntervalQuality::Minor, 7));
                vec![self.root.clone(), minor_third, perfect_fifth, minor_seventh]
            },
        }
    }

    pub fn reverse_lookup(notes: &Vec<Note>) -> HashSet<Chord> {
        let mut possible_chords = HashSet::new();
        for white_note in WhiteNote::iter() {
            for root in [Note::WhiteNote(white_note.clone()), Note::Sharp(white_note.clone()), Note::Flat(white_note)].iter() {
                for quality in ChordQuality::iter() {
                    let chord = Chord::new(root.clone(), quality);
                    let chord_notes = chord.get_notes();
                    if notes.iter().all(|note| chord_notes.contains(note)) {
                        possible_chords.insert(chord);
                    }
                }
            }
        }
        possible_chords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_from_str() {
        let chord = Chord::from_str("C").unwrap();
        assert_eq!(chord.root, Note::WhiteNote(WhiteNote::C));
        assert_eq!(chord.quality, ChordQuality::Major);

        let chord = Chord::from_str("Cm").unwrap();
        assert_eq!(chord.root, Note::WhiteNote(WhiteNote::C));
        assert_eq!(chord.quality, ChordQuality::Minor);

        let chord = Chord::from_str("C7").unwrap();
        assert_eq!(chord.root, Note::WhiteNote(WhiteNote::C));
        assert_eq!(chord.quality, ChordQuality::DominantSeventh);

        let chord = Chord::from_str("Cmaj7").unwrap();
        assert_eq!(chord.root, Note::WhiteNote(WhiteNote::C));
        assert_eq!(chord.quality, ChordQuality::MajorSeventh);

        let chord = Chord::from_str("Cm7").unwrap();
        assert_eq!(chord.root, Note::WhiteNote(WhiteNote::C));
        assert_eq!(chord.quality, ChordQuality::MinorSeventh);
    }

    #[test]
    fn test_chord_get_notes() {
        let chord = Chord::new(Note::WhiteNote(WhiteNote::C), ChordQuality::Major);
        let notes = chord.get_notes();
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::C));
        assert_eq!(notes[1], Note::WhiteNote(WhiteNote::E));
        assert_eq!(notes[2], Note::WhiteNote(WhiteNote::G));

        let chord = Chord::new(Note::WhiteNote(WhiteNote::C), ChordQuality::Minor);
        let notes = chord.get_notes();
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::C));
        assert_eq!(notes[1], Note::Flat(WhiteNote::E));
        assert_eq!(notes[2], Note::WhiteNote(WhiteNote::G));

        let chord = Chord::new(Note::WhiteNote(WhiteNote::D), ChordQuality::Major);
        let notes = chord.get_notes();
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::D));
        assert_eq!(notes[1], Note::Sharp(WhiteNote::F));
        assert_eq!(notes[2], Note::WhiteNote(WhiteNote::A));

        let chord = Chord::new(Note::WhiteNote(WhiteNote::D), ChordQuality::Minor);
        let notes = chord.get_notes();
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::D));
        assert_eq!(notes[1], Note::WhiteNote(WhiteNote::F));
        assert_eq!(notes[2], Note::WhiteNote(WhiteNote::A));

        let chord = Chord::new(Note::WhiteNote(WhiteNote::C), ChordQuality::DominantSeventh);
        let notes = chord.get_notes();
        assert_eq!(notes.len(), 4);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::C));
        assert_eq!(notes[1], Note::WhiteNote(WhiteNote::E));
        assert_eq!(notes[2], Note::WhiteNote(WhiteNote::G));
        assert_eq!(notes[3], Note::Flat(WhiteNote::B));

        let chord = Chord::new(Note::WhiteNote(WhiteNote::C), ChordQuality::MajorSeventh);
        let notes = chord.get_notes();
        assert_eq!(notes.len(), 4);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::C));
        assert_eq!(notes[1], Note::WhiteNote(WhiteNote::E));
        assert_eq!(notes[2], Note::WhiteNote(WhiteNote::G));
        assert_eq!(notes[3], Note::WhiteNote(WhiteNote::B));

        let chord = Chord::new(Note::WhiteNote(WhiteNote::C), ChordQuality::MinorSeventh);
        let notes = chord.get_notes();
        assert_eq!(notes.len(), 4);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::C));
        assert_eq!(notes[1], Note::Flat(WhiteNote::E));
        assert_eq!(notes[2], Note::WhiteNote(WhiteNote::G));
        assert_eq!(notes[3], Note::Flat(WhiteNote::B));
    }

    #[test]
    fn test_chord_reverse_lookup() {
        let notes = vec![
            Note::WhiteNote(WhiteNote::C),
            Note::WhiteNote(WhiteNote::E),
            Note::WhiteNote(WhiteNote::G),
        ];
        let chords = Chord::reverse_lookup(&notes);
        assert!(chords.contains(&Chord::new(Note::WhiteNote(WhiteNote::C), ChordQuality::Major)));

        let notes = vec![
            Note::WhiteNote(WhiteNote::A),
            Note::WhiteNote(WhiteNote::C),
            Note::WhiteNote(WhiteNote::E),
        ];
        let chords = Chord::reverse_lookup(&notes);
        assert!(chords.contains(&Chord::new(Note::WhiteNote(WhiteNote::A), ChordQuality::Minor)));
    }
}
