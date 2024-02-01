use std::fmt;

#[derive(Debug, Clone)]
pub enum WhiteNote { C, D, E, F, G, A, B }

#[derive(Clone)]
pub enum Note {
    WhiteNote(WhiteNote),
    Sharp(WhiteNote),
    Flat(WhiteNote),
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

    fn down_semitone(&self) -> Note {
        match self {
            Note::WhiteNote(white_note) => Note::Flat(white_note.clone()),
            Note::Flat(white_note) =>
                match white_note {
                    WhiteNote::C => Note::Flat(WhiteNote::B),
                    WhiteNote::D => Note::WhiteNote(WhiteNote::C),
                    WhiteNote::E => Note::WhiteNote(WhiteNote::D),
                    WhiteNote::F => Note::Flat(WhiteNote::E),
                    WhiteNote::G => Note::WhiteNote(WhiteNote::F),
                    WhiteNote::A => Note::WhiteNote(WhiteNote::G),
                    WhiteNote::B => Note::Flat(WhiteNote::A),
                },
            Note::Sharp(white_note) => Note::WhiteNote(white_note.clone()),
        }
    }

    fn up_semitones(&self, n_semitones: u8) -> Note {
        let mut note = self.clone();
        for _ in 0..n_semitones {
            note = note.up_semitone();
        }
        note
    }

    pub fn major_third(&self) -> Note {
        self.up_semitones(4)
    }

    pub fn minor_third(&self) -> Note {
        let note = self.up_semitones(4);
        note.down_semitone()
    }

    pub fn perfect_fifth(&self) -> Note {
        self.up_semitones(7)
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
