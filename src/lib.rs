use std::fmt;

#[derive(Debug, Clone)]
pub enum WhiteNote { C, D, E, F, G, A, B }

#[derive(Clone)]
pub enum Note {
    WhiteNote(WhiteNote),
    Sharp(WhiteNote),
    Flat(WhiteNote),
}

pub enum ChordQuality { Major, Minor, }

pub struct Chord {
    root: Note,
    quality: ChordQuality,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Note::WhiteNote(white_note) => write!(f, "{:?}", white_note),
            Note::Sharp(white_note) =>
                match white_note {
                    WhiteNote::B => write!(f, "C"),
                    WhiteNote::E => write!(f, "F"),
                    _ => write!(f, "{:?}#", white_note),
                }
            Note::Flat(white_note) =>
                match white_note {
                    WhiteNote::C => write!(f, "B"),
                    WhiteNote::F => write!(f, "E"),
                    _ => write!(f, "{:?}b", white_note),
                }
        }
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let quality = match self.quality {
            ChordQuality::Major => "",
            ChordQuality::Minor => "m",
        };
        write!(f, "{}{}", self.root, quality)
    }
}

impl Note {
    fn up_semitone(&self) -> Note {
        match self {
            Note::WhiteNote(white_note) => Note::Sharp(white_note.clone()),
            Note::Sharp(white_note) =>
                match white_note {
                    WhiteNote::C => Note::WhiteNote(WhiteNote::D),
                    WhiteNote::D => Note::WhiteNote(WhiteNote::E),
                    WhiteNote::E => Note::Sharp(WhiteNote::F),
                    WhiteNote::F => Note::WhiteNote(WhiteNote::G),
                    WhiteNote::G => Note::WhiteNote(WhiteNote::A),
                    WhiteNote::A => Note::WhiteNote(WhiteNote::B),
                    WhiteNote::B => Note::Sharp(WhiteNote::C),
                },
            Note::Flat(white_note) => Note::WhiteNote(white_note.clone()),
        }
    }

    fn up_semitones(&self, n_semitones: u8) -> Note {
        let mut note = self.clone();
        for _ in 0..n_semitones {
            note = note.up_semitone();
        }
        note
    }

    pub fn from_str(s: &str) -> Option<Note> {
        let mut chars = s.chars();
        let white_note = match chars.next() {
            Some('C') => WhiteNote::C,
            Some('D') => WhiteNote::D,
            Some('E') => WhiteNote::E,
            Some('F') => WhiteNote::F,
            Some('G') => WhiteNote::G,
            Some('A') => WhiteNote::A,
            Some('B') => WhiteNote::B,
            _ => return None,
        };
        let accidental = match chars.next() {
            Some('#') => Note::Sharp(white_note),
            Some('b') => Note::Flat(white_note),
            _ => Note::WhiteNote(white_note),
        };
        Some(accidental)
    }
}

impl Chord {
    pub fn new(root: &Note, quality: ChordQuality) -> Chord {
        Chord { root: root.clone(), quality }
    }

    pub fn get_notes(&self) -> Vec<Note> {
        match self.quality {
            ChordQuality::Major => {
                let major_third = self.root.up_semitones(4);
                let perfect_fifth = self.root.up_semitones(7);
                vec![self.root.clone(), major_third, perfect_fifth]
            },
            ChordQuality::Minor => {
                let minor_third = self.root.up_semitones(3);
                let perfect_fifth = self.root.up_semitones(7);
                vec![self.root.clone(), minor_third, perfect_fifth]
            },
        }
    }
}
