use std::fmt;

#[derive(Debug, Clone)]
pub enum WhiteNote { C, D, E, F, G, A, B }

#[derive(Debug, Clone)]
pub enum Note {
    WhiteNote(WhiteNote),
    Sharp(WhiteNote),
    Flat(WhiteNote),
}

#[derive(Debug)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

#[derive(Debug)]
pub struct Interval {
    quality: IntervalQuality,
    number: u8,
}

impl Interval {
    pub fn new(quality: IntervalQuality, number: u8) -> Interval {
        Interval { quality, number }
    }

    fn get_number_semitones(&self) -> u8 {
        match self.quality {
            IntervalQuality::Perfect => match self.number {
                1 => 0,
                4 => 5,
                5 => 7,
                8 => 12,
                _ => panic!("Invalid interval"),
            },
            IntervalQuality::Major => match self.number {
                2 => 2,
                3 => 4,
                6 => 9,
                7 => 11,
                _ => panic!("Invalid interval"),
            },
            IntervalQuality::Minor => Interval::new(IntervalQuality::Major, self.number).get_number_semitones() - 1,
            IntervalQuality::Augmented => Interval::new(IntervalQuality::Perfect, self.number).get_number_semitones() + 1,
            IntervalQuality::Diminished => Interval::new(IntervalQuality::Perfect, self.number).get_number_semitones() - 1,
        }
    }
}

impl WhiteNote {
    fn get_index(&self) -> u8 {
        match self {
            WhiteNote::C => 0,
            WhiteNote::D => 1,
            WhiteNote::E => 2,
            WhiteNote::F => 3,
            WhiteNote::G => 4,
            WhiteNote::A => 5,
            WhiteNote::B => 6,
        }
    }

    fn successor(&self) -> WhiteNote {
        match self {
            WhiteNote::C => WhiteNote::D,
            WhiteNote::D => WhiteNote::E,
            WhiteNote::E => WhiteNote::F,
            WhiteNote::F => WhiteNote::G,
            WhiteNote::G => WhiteNote::A,
            WhiteNote::A => WhiteNote::B,
            WhiteNote::B => WhiteNote::C,
        }
    }

    fn nth_successor(&self, n: u8) -> WhiteNote {
        let mut note = self.clone();
        for _ in 0..n {
            note = note.successor();
        }
        note
    }
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

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.get_index() == other.get_index()
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_index().partial_cmp(&other.get_index())
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

    fn up_semitones(&self, n: u8) -> Note {
        let mut note = self.clone();
        for _ in 0..n {
            note = note.up_semitone();
        }
        note
    }

    fn get_index(&self) -> u8 {
        match self {
            Note::WhiteNote(white_note) => match white_note {
                WhiteNote::C => 0,
                WhiteNote::D => 2,
                WhiteNote::E => 4,
                WhiteNote::F => 5,
                WhiteNote::G => 7,
                WhiteNote::A => 9,
                WhiteNote::B => 11,
            },
            Note::Sharp(white_note) => (1 + Note::WhiteNote(white_note.clone()).get_index()) % 12,
            Note::Flat(white_note) => Note::WhiteNote(white_note.clone()).get_index() - 1,
        }
    }

    fn get_white_note(&self) -> WhiteNote {
        match self {
            Note::WhiteNote(white_note) => white_note.clone(),
            Note::Sharp(white_note) => white_note.clone(),
            Note::Flat(white_note) => white_note.clone(),
        }
    }

    fn get_generic_interval(&self, other: &Note) -> u8 {
        let first = self.get_white_note();
        let second = other.get_white_note();
        let first_index = first.get_index();
        let second_index = second.get_index();
        (second_index + 7 - first_index) % 7 + 1
    }

    fn get_semitones(&self, other: &Note) -> u8 {
        let mut note = self.clone();
        let mut n_semitones = 0;
        while note != *other {
            note = note.up_semitone();
            n_semitones += 1;
        }
        n_semitones
    }

    fn add_accidentals(&self, other: WhiteNote) -> Note {
        let other_note = Note::WhiteNote(other.clone());
        if *self == other_note {
           self.clone()
        }
        else if *self < other_note {
            Note::Flat(other.clone())
        }
        else {
            Note::Sharp(other.clone())
        }
    }

    pub fn up_interval(&self, interval: Interval) -> Note {
        let white_note = self.get_white_note();
        let upper_white_note = white_note.nth_successor(interval.number - 1);
        let n_semitones = interval.get_number_semitones();
        let upper_note = self.up_semitones(n_semitones);
        upper_note.add_accidentals(upper_white_note)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_display() {
        assert_eq!(format!("{}", Note::WhiteNote(WhiteNote::C)), "C");
        assert_eq!(format!("{}", Note::Sharp(WhiteNote::C)), "C#");
        assert_eq!(format!("{}", Note::Flat(WhiteNote::C)), "B");
        assert_eq!(format!("{}", Note::WhiteNote(WhiteNote::D)), "D");
        assert_eq!(format!("{}", Note::Sharp(WhiteNote::D)), "D#");
        assert_eq!(format!("{}", Note::Flat(WhiteNote::D)), "Db");
        assert_eq!(format!("{}", Note::WhiteNote(WhiteNote::E)), "E");
        assert_eq!(format!("{}", Note::Sharp(WhiteNote::E)), "F");
        assert_eq!(format!("{}", Note::Flat(WhiteNote::E)), "Eb");
        assert_eq!(format!("{}", Note::WhiteNote(WhiteNote::F)), "F");
        assert_eq!(format!("{}", Note::Sharp(WhiteNote::F)), "F#");
        assert_eq!(format!("{}", Note::Flat(WhiteNote::F)), "E");
        assert_eq!(format!("{}", Note::WhiteNote(WhiteNote::G)), "G");
        assert_eq!(format!("{}", Note::Sharp(WhiteNote::G)), "G#");
        assert_eq!(format!("{}", Note::Flat(WhiteNote::G)), "Gb");
        assert_eq!(format!("{}", Note::WhiteNote(WhiteNote::A)), "A");
        assert_eq!(format!("{}", Note::Sharp(WhiteNote::A)), "A#");
        assert_eq!(format!("{}", Note::Flat(WhiteNote::A)), "Ab");
        assert_eq!(format!("{}", Note::WhiteNote(WhiteNote::B)), "B");
        assert_eq!(format!("{}", Note::Sharp(WhiteNote::B)), "C");
        assert_eq!(format!("{}", Note::Flat(WhiteNote::B)), "Bb");
    }

    #[test]
    fn test_note_up_semitone() {
        assert_eq!(Note::WhiteNote(WhiteNote::C).up_semitone(), Note::Sharp(WhiteNote::C));
        assert_eq!(Note::Sharp(WhiteNote::C).up_semitone(), Note::WhiteNote(WhiteNote::D));
        assert_eq!(Note::WhiteNote(WhiteNote::D).up_semitone(), Note::Sharp(WhiteNote::D));
        assert_eq!(Note::Sharp(WhiteNote::D).up_semitone(), Note::WhiteNote(WhiteNote::E));
        assert_eq!(Note::WhiteNote(WhiteNote::E).up_semitone(), Note::WhiteNote(WhiteNote::F));
        assert_eq!(Note::WhiteNote(WhiteNote::F).up_semitone(), Note::Sharp(WhiteNote::F));
        assert_eq!(Note::Sharp(WhiteNote::F).up_semitone(), Note::WhiteNote(WhiteNote::G));
        assert_eq!(Note::WhiteNote(WhiteNote::G).up_semitone(), Note::Sharp(WhiteNote::G));
        assert_eq!(Note::Sharp(WhiteNote::G).up_semitone(), Note::WhiteNote(WhiteNote::A));
        assert_eq!(Note::WhiteNote(WhiteNote::A).up_semitone(), Note::Sharp(WhiteNote::A));
        assert_eq!(Note::Sharp(WhiteNote::A).up_semitone(), Note::WhiteNote(WhiteNote::B));
        assert_eq!(Note::WhiteNote(WhiteNote::B).up_semitone(), Note::WhiteNote(WhiteNote::C));
    }

    #[test]
    fn test_note_generic_interval() {
        assert_eq!(Note::WhiteNote(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::C)), 1);
        assert_eq!(Note::WhiteNote(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::D)), 2);
        assert_eq!(Note::WhiteNote(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::E)), 3);
        assert_eq!(Note::WhiteNote(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::F)), 4);
        assert_eq!(Note::WhiteNote(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::G)), 5);
        assert_eq!(Note::WhiteNote(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::A)), 6);
        assert_eq!(Note::WhiteNote(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::B)), 7);
        assert_eq!(Note::Sharp(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::C)), 1);
        assert_eq!(Note::Sharp(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::D)), 2);
        assert_eq!(Note::Flat(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::C)), 1);
        assert_eq!(Note::Flat(WhiteNote::C).get_generic_interval(&Note::WhiteNote(WhiteNote::B)), 7);
        assert_eq!(Note::Sharp(WhiteNote::B).get_generic_interval(&Note::WhiteNote(WhiteNote::C)), 2);
    }

    #[test]
    fn test_note_up_interval() {
        assert_eq!(Note::WhiteNote(WhiteNote::C).up_interval(Interval::new(IntervalQuality::Major, 3)), Note::WhiteNote(WhiteNote::E));
        assert_eq!(Note::WhiteNote(WhiteNote::C).up_interval(Interval::new(IntervalQuality::Minor, 3)), Note::Flat(WhiteNote::E));
        assert_eq!(Note::WhiteNote(WhiteNote::C).up_interval(Interval::new(IntervalQuality::Perfect, 5)), Note::WhiteNote(WhiteNote::G));
        assert_eq!(Note::WhiteNote(WhiteNote::D).up_interval(Interval::new(IntervalQuality::Major, 3)), Note::Sharp(WhiteNote::F));
        assert_eq!(Note::WhiteNote(WhiteNote::D).up_interval(Interval::new(IntervalQuality::Minor, 3)), Note::WhiteNote(WhiteNote::F));
        assert_eq!(Note::WhiteNote(WhiteNote::D).up_interval(Interval::new(IntervalQuality::Perfect, 5)), Note::WhiteNote(WhiteNote::A));
        assert_eq!(Note::WhiteNote(WhiteNote::E).up_interval(Interval::new(IntervalQuality::Major, 3)), Note::Sharp(WhiteNote::G));
        assert_eq!(Note::WhiteNote(WhiteNote::E).up_interval(Interval::new(IntervalQuality::Minor, 3)), Note::WhiteNote(WhiteNote::G));
        assert_eq!(Note::WhiteNote(WhiteNote::E).up_interval(Interval::new(IntervalQuality::Perfect, 5)), Note::WhiteNote(WhiteNote::B));
        assert_eq!(Note::WhiteNote(WhiteNote::F).up_interval(Interval::new(IntervalQuality::Major, 3)), Note::WhiteNote(WhiteNote::A));
        assert_eq!(Note::WhiteNote(WhiteNote::F).up_interval(Interval::new(IntervalQuality::Minor, 3)), Note::Flat(WhiteNote::A));
        assert_eq!(Note::WhiteNote(WhiteNote::F).up_interval(Interval::new(IntervalQuality::Perfect, 5)), Note::WhiteNote(WhiteNote::C));
        assert_eq!(Note::WhiteNote(WhiteNote::G).up_interval(Interval::new(IntervalQuality::Major, 3)), Note::WhiteNote(WhiteNote::B));
        assert_eq!(Note::WhiteNote(WhiteNote::G).up_interval(Interval::new(IntervalQuality::Minor, 3)), Note::Flat(WhiteNote::B));
        assert_eq!(Note::WhiteNote(WhiteNote::G).up_interval(Interval::new(IntervalQuality::Perfect, 5)), Note::WhiteNote(WhiteNote::D));
        assert_eq!(Note::WhiteNote(WhiteNote::A).up_interval(Interval::new(IntervalQuality::Major, 3)), Note::Sharp(WhiteNote::C));
        assert_eq!(Note::WhiteNote(WhiteNote::A).up_interval(Interval::new(IntervalQuality::Minor, 3)), Note::WhiteNote(WhiteNote::C));
        assert_eq!(Note::WhiteNote(WhiteNote::A).up_interval(Interval::new(IntervalQuality::Perfect, 5)), Note::WhiteNote(WhiteNote::E));
        assert_eq!(Note::WhiteNote(WhiteNote::B).up_interval(Interval::new(IntervalQuality::Major, 3)), Note::Sharp(WhiteNote::D));
        assert_eq!(Note::WhiteNote(WhiteNote::B).up_interval(Interval::new(IntervalQuality::Minor, 3)), Note::WhiteNote(WhiteNote::D));
        assert_eq!(Note::WhiteNote(WhiteNote::B).up_interval(Interval::new(IntervalQuality::Perfect, 5)), Note::Sharp(WhiteNote::F));
    }
}
