use std::fmt;
use crate::notes::*;

pub enum ChordQuality { Major, Minor, }

pub struct Chord {
    root: Note,
    quality: ChordQuality,
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

impl Chord {
    pub fn new(root: &Note, quality: ChordQuality) -> Chord {
        Chord { root: root.clone(), quality }
    }

    pub fn get_notes(&self) -> Vec<Note> {
        match self.quality {
            ChordQuality::Major => {
                let major_third = self.root.major_third();
                let perfect_fifth = self.root.perfect_fifth();
                vec![self.root.clone(), major_third, perfect_fifth]
            },
            ChordQuality::Minor => {
                let minor_third = self.root.minor_third();
                let perfect_fifth = self.root.perfect_fifth();
                vec![self.root.clone(), minor_third, perfect_fifth]
            },
        }
    }
}
