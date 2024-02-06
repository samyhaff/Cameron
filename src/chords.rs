use std::fmt;
use crate::notes::*;

pub enum ChordQuality {
    Major,
    Minor,
    DominantSeventh,
    MajorSeventh,
    MinorSeventh,
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
