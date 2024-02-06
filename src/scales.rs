use std::fmt;
use regex::Regex;
use crate::notes::*;

#[derive(Debug, PartialEq)]
pub enum ScaleType {
    Major,
    Minor,
}

pub struct Scale {
    root: Note,
    scale_type: ScaleType,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.root, match self.scale_type {
            ScaleType::Major => "major scale",
            ScaleType::Minor => "minor scale",
        })
    }
}

impl Scale {
    pub fn new(root: Note, scale_type: ScaleType) -> Scale {
        Scale { root, scale_type, }
    }

    pub fn from_str(s: &str) -> Option<Scale> {
        let re = Regex::new(r"([A-Ga-g][#b]?)\s*((?:major|minor))").unwrap();
        let caps = re.captures(s)?;
        let root = Note::from_str(caps.get(1)?.as_str())?;
        let scale_type = match caps.get(2)?.as_str() {
            "major" => ScaleType::Major,
            "minor" => ScaleType::Minor,
            _ => return None,
        };
        Some(Scale::new(root, scale_type))
    }

    pub fn get_notes(&self) -> Vec<Note> {
        match self.scale_type {
            ScaleType::Major => vec![
                self.root.clone(),
                self.root.up_interval(Interval::new(IntervalQuality::Major, 2)),
                self.root.up_interval(Interval::new(IntervalQuality::Major, 3)),
                self.root.up_interval(Interval::new(IntervalQuality::Perfect, 4)),
                self.root.up_interval(Interval::new(IntervalQuality::Perfect, 5)),
                self.root.up_interval(Interval::new(IntervalQuality::Major, 6)),
                self.root.up_interval(Interval::new(IntervalQuality::Major, 7)),
            ],
            ScaleType::Minor => vec![
                self.root.clone(),
                self.root.up_interval(Interval::new(IntervalQuality::Major, 2)),
                self.root.up_interval(Interval::new(IntervalQuality::Minor, 3)),
                self.root.up_interval(Interval::new(IntervalQuality::Perfect, 4)),
                self.root.up_interval(Interval::new(IntervalQuality::Perfect, 5)),
                self.root.up_interval(Interval::new(IntervalQuality::Minor, 6)),
                self.root.up_interval(Interval::new(IntervalQuality::Minor, 7)),
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scale_from_str() {
        let scale = Scale::from_str("C major").unwrap();
        assert_eq!(scale.root, Note::WhiteNote(WhiteNote::C));
        assert_eq!(scale.scale_type, ScaleType::Major);

        let scale = Scale::from_str("C minor").unwrap();
        assert_eq!(scale.root, Note::WhiteNote(WhiteNote::C));
        assert_eq!(scale.scale_type, ScaleType::Minor);

        let scale = Scale::from_str("C# major").unwrap();
        assert_eq!(scale.root, Note::Sharp(WhiteNote::C));
        assert_eq!(scale.scale_type, ScaleType::Major);

        let scale = Scale::from_str("C# minor").unwrap();
        assert_eq!(scale.root, Note::Sharp(WhiteNote::C));
        assert_eq!(scale.scale_type, ScaleType::Minor);

        let scale = Scale::from_str("Cb major").unwrap();
        assert_eq!(scale.root, Note::Flat(WhiteNote::C));
        assert_eq!(scale.scale_type, ScaleType::Major);

        let scale = Scale::from_str("Cb minor").unwrap();
        assert_eq!(scale.root, Note::Flat(WhiteNote::C));
        assert_eq!(scale.scale_type, ScaleType::Minor);
    }

    #[test]
    fn test_scale_get_notes() {
        let scale = Scale::new(Note::WhiteNote(WhiteNote::C), ScaleType::Major);
        let notes = scale.get_notes();
        assert_eq!(notes.len(), 7);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::C));
        assert_eq!(notes[1], Note::WhiteNote(WhiteNote::D));
        assert_eq!(notes[2], Note::WhiteNote(WhiteNote::E));
        assert_eq!(notes[3], Note::WhiteNote(WhiteNote::F));
        assert_eq!(notes[4], Note::WhiteNote(WhiteNote::G));
        assert_eq!(notes[5], Note::WhiteNote(WhiteNote::A));
        assert_eq!(notes[6], Note::WhiteNote(WhiteNote::B));

        let scale = Scale::new(Note::WhiteNote(WhiteNote::C), ScaleType::Minor);
        let notes = scale.get_notes();
        assert_eq!(notes.len(), 7);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::C));
        assert_eq!(notes[1], Note::WhiteNote(WhiteNote::D));
        assert_eq!(notes[2], Note::Flat(WhiteNote::E));
        assert_eq!(notes[3], Note::WhiteNote(WhiteNote::F));
        assert_eq!(notes[4], Note::WhiteNote(WhiteNote::G));
        assert_eq!(notes[5], Note::Flat(WhiteNote::A));
        assert_eq!(notes[6], Note::Flat(WhiteNote::B));

        let scale = Scale::new(Note::WhiteNote(WhiteNote::A), ScaleType::Minor);
        let notes = scale.get_notes();
        assert_eq!(notes.len(), 7);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::A));
        assert_eq!(notes[1], Note::WhiteNote(WhiteNote::B));
        assert_eq!(notes[2], Note::WhiteNote(WhiteNote::C));
        assert_eq!(notes[3], Note::WhiteNote(WhiteNote::D));
        assert_eq!(notes[4], Note::WhiteNote(WhiteNote::E));
        assert_eq!(notes[5], Note::WhiteNote(WhiteNote::F));
        assert_eq!(notes[6], Note::WhiteNote(WhiteNote::G));

        let scale = Scale::new(Note::WhiteNote(WhiteNote::A), ScaleType::Major);
        let notes = scale.get_notes();
        assert_eq!(notes.len(), 7);
        assert_eq!(notes[0], Note::WhiteNote(WhiteNote::A));
        assert_eq!(notes[1], Note::WhiteNote(WhiteNote::B));
        assert_eq!(notes[2], Note::Sharp(WhiteNote::C));
        assert_eq!(notes[3], Note::WhiteNote(WhiteNote::D));
        assert_eq!(notes[4], Note::WhiteNote(WhiteNote::E));
        assert_eq!(notes[5], Note::Sharp(WhiteNote::F));
        assert_eq!(notes[6], Note::Sharp(WhiteNote::G));
    }
}
