//! Chord theory and chord name parsing
//!
//! This module provides types and functions for working with chords:
//! - Chord types and their interval formulas
//! - Chord quality (major, minor, etc.)
//! - Chord name parsing (e.g., "Abm7", "Cmaj9")
//! - Voicing classification (core, full, jazzy)

use crate::error::{ChordCraftError, Result};
use crate::interval::*;
use crate::note::PitchClass;
use std::fmt;

/// Chord quality/type
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum ChordQuality {
    // Triads
    Major,
    Minor,
    Diminished,
    Augmented,

    // Suspended
    Sus2,
    Sus4,

    // 7th chords
    Dominant7,          // X7
    Major7,             // Xmaj7
    Minor7,             // Xm7
    MinorMajor7,        // Xm(maj7)
    Diminished7,        // Xdim7
    HalfDiminished7,    // Xm7b5

    // Extended chords
    Dominant9,          // X9
    Major9,             // Xmaj9
    Minor9,             // Xm9
    Dominant11,         // X11
    Minor11,            // Xm11
    Dominant13,         // X13
    Major13,            // Xmaj13
    Minor13,            // Xm13

    // Altered dominants
    Dominant7b9,        // X7b9
    Dominant7sharp9,    // X7#9
    Dominant7b5,        // X7b5
    Dominant7sharp5,    // X7#5 (aug7)

    // Add chords
    Add9,               // Xadd9
    MinorAdd9,          // Xmadd9
    Add11,              // Xadd11

    // 6th chords
    Major6,             // X6
    Minor6,             // Xm6
}

impl ChordQuality {
    /// Get the interval formula for this chord type
    /// Returns (required_intervals, optional_intervals)
    pub fn intervals(&self) -> (Vec<Interval>, Vec<Interval>) {
        use ChordQuality::*;

        match self {
            // Triads
            Major => (vec![UNISON, MAJOR_THIRD, PERFECT_FIFTH], vec![]),
            Minor => (vec![UNISON, MINOR_THIRD, PERFECT_FIFTH], vec![]),
            Diminished => (vec![UNISON, MINOR_THIRD, Interval::new(IntervalQuality::Diminished, 5)], vec![]),
            Augmented => (vec![UNISON, MAJOR_THIRD, Interval::new(IntervalQuality::Augmented, 5)], vec![]),

            // Suspended
            Sus2 => (vec![UNISON, MAJOR_SECOND, PERFECT_FIFTH], vec![]),
            Sus4 => (vec![UNISON, PERFECT_FOURTH, PERFECT_FIFTH], vec![]),

            // 7th chords
            Dominant7 => (vec![UNISON, MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH], vec![]),
            Major7 => (vec![UNISON, MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH], vec![]),
            Minor7 => (vec![UNISON, MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH], vec![]),
            MinorMajor7 => (vec![UNISON, MINOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH], vec![]),
            Diminished7 => (
                vec![UNISON, MINOR_THIRD, Interval::new(IntervalQuality::Diminished, 5), Interval::new(IntervalQuality::Diminished, 7)],
                vec![],
            ),
            HalfDiminished7 => (
                vec![UNISON, MINOR_THIRD, Interval::new(IntervalQuality::Diminished, 5), MINOR_SEVENTH],
                vec![],
            ),

            // Extended chords (9ths)
            Dominant9 => (
                vec![UNISON, MAJOR_THIRD, MINOR_SEVENTH, MAJOR_NINTH],
                vec![PERFECT_FIFTH],  // 5th often omitted in jazz voicings
            ),
            Major9 => (
                vec![UNISON, MAJOR_THIRD, MAJOR_SEVENTH, MAJOR_NINTH],
                vec![PERFECT_FIFTH],
            ),
            Minor9 => (
                vec![UNISON, MINOR_THIRD, MINOR_SEVENTH, MAJOR_NINTH],
                vec![PERFECT_FIFTH],
            ),

            // Extended chords (11ths)
            Dominant11 => (
                vec![UNISON, MAJOR_THIRD, MINOR_SEVENTH, MAJOR_NINTH, PERFECT_ELEVENTH],
                vec![PERFECT_FIFTH],
            ),
            Minor11 => (
                vec![UNISON, MINOR_THIRD, MINOR_SEVENTH, MAJOR_NINTH, PERFECT_ELEVENTH],
                vec![PERFECT_FIFTH],
            ),

            // Extended chords (13ths)
            Dominant13 => (
                vec![UNISON, MAJOR_THIRD, MINOR_SEVENTH, MAJOR_NINTH, MAJOR_THIRTEENTH],
                vec![PERFECT_FIFTH, PERFECT_ELEVENTH],
            ),
            Major13 => (
                vec![UNISON, MAJOR_THIRD, MAJOR_SEVENTH, MAJOR_NINTH, MAJOR_THIRTEENTH],
                vec![PERFECT_FIFTH, PERFECT_ELEVENTH],
            ),
            Minor13 => (
                vec![UNISON, MINOR_THIRD, MINOR_SEVENTH, MAJOR_NINTH, MAJOR_THIRTEENTH],
                vec![PERFECT_FIFTH, PERFECT_ELEVENTH],
            ),

            // Altered dominants
            Dominant7b9 => (
                vec![UNISON, MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH, MINOR_NINTH],
                vec![],
            ),
            Dominant7sharp9 => (
                vec![UNISON, MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH, Interval::new(IntervalQuality::Augmented, 9)],
                vec![],
            ),
            Dominant7b5 => (
                vec![UNISON, MAJOR_THIRD, Interval::new(IntervalQuality::Diminished, 5), MINOR_SEVENTH],
                vec![],
            ),
            Dominant7sharp5 => (
                vec![UNISON, MAJOR_THIRD, Interval::new(IntervalQuality::Augmented, 5), MINOR_SEVENTH],
                vec![],
            ),

            // Add chords
            Add9 => (vec![UNISON, MAJOR_THIRD, PERFECT_FIFTH, MAJOR_NINTH], vec![]),
            MinorAdd9 => (vec![UNISON, MINOR_THIRD, PERFECT_FIFTH, MAJOR_NINTH], vec![]),
            Add11 => (vec![UNISON, MAJOR_THIRD, PERFECT_FIFTH, PERFECT_ELEVENTH], vec![]),

            // 6th chords
            Major6 => (vec![UNISON, MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH], vec![]),
            Minor6 => (vec![UNISON, MINOR_THIRD, PERFECT_FIFTH, MAJOR_SIXTH], vec![]),
        }
    }

    /// Check if the 5th can be omitted in voicings
    /// Typically true for 7th chords and extended chords where the 7th is present
    pub fn can_omit_fifth(&self) -> bool {
        use ChordQuality::*;
        matches!(
            self,
            Dominant7 | Major7 | Minor7 | MinorMajor7 |
            Dominant9 | Major9 | Minor9 |
            Dominant11 | Minor11 |
            Dominant13 | Major13 | Minor13 |
            Dominant7b9 | Dominant7sharp9 | Dominant7b5 | Dominant7sharp5
        )
    }

    /// Get a display name for this chord quality
    pub fn display_name(&self) -> &'static str {
        use ChordQuality::*;
        match self {
            Major => "",
            Minor => "m",
            Diminished => "dim",
            Augmented => "aug",
            Sus2 => "sus2",
            Sus4 => "sus4",
            Dominant7 => "7",
            Major7 => "maj7",
            Minor7 => "m7",
            MinorMajor7 => "m(maj7)",
            Diminished7 => "dim7",
            HalfDiminished7 => "m7b5",
            Dominant9 => "9",
            Major9 => "maj9",
            Minor9 => "m9",
            Dominant11 => "11",
            Minor11 => "m11",
            Dominant13 => "13",
            Major13 => "maj13",
            Minor13 => "m13",
            Dominant7b9 => "7b9",
            Dominant7sharp9 => "7#9",
            Dominant7b5 => "7b5",
            Dominant7sharp5 => "7#5",
            Add9 => "add9",
            MinorAdd9 => "madd9",
            Add11 => "add11",
            Major6 => "6",
            Minor6 => "m6",
        }
    }
}

/// Voicing type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoicingType {
    /// Core notes only (root, 3rd, 7th for 7th chords)
    Core,
    /// All chord tones present
    Full,
    /// Jazz voicings with possible omissions
    Jazzy,
}

/// A chord with root note and quality
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub root: PitchClass,
    pub quality: ChordQuality,
    pub bass: Option<PitchClass>,  // For slash chords (e.g., C/G)
}

impl Chord {
    /// Create a new chord
    pub fn new(root: PitchClass, quality: ChordQuality) -> Self {
        Chord {
            root,
            quality,
            bass: None,
        }
    }

    /// Create a slash chord (e.g., C/G)
    pub fn with_bass(root: PitchClass, quality: ChordQuality, bass: PitchClass) -> Self {
        Chord {
            root,
            quality,
            bass: Some(bass),
        }
    }

    /// Get all notes in this chord (pitch classes)
    pub fn notes(&self) -> Vec<PitchClass> {
        let (required, optional) = self.quality.intervals();
        let all_intervals: Vec<_> = required.into_iter().chain(optional).collect();

        all_intervals
            .iter()
            .map(|interval| {
                self.root.add_semitones(interval.to_semitones() as i32)
            })
            .collect()
    }

    /// Get required notes (for core voicings)
    pub fn required_notes(&self) -> Vec<PitchClass> {
        let (required, _) = self.quality.intervals();
        required
            .iter()
            .map(|interval| {
                self.root.add_semitones(interval.to_semitones() as i32)
            })
            .collect()
    }

    /// Get core notes (essential for chord identity)
    /// For triads: root, 3rd, 5th
    /// For 7th chords: root, 3rd, 7th (5th can be omitted)
    /// For extended: root, 3rd, 7th, extension
    pub fn core_notes(&self) -> Vec<PitchClass> {
        let (required, _) = self.quality.intervals();

        // For most 7th and extended chords, the 5th is not essential
        let skip_fifth = self.quality.can_omit_fifth();

        required
            .iter()
            .filter(|interval| {
                if skip_fifth {
                    interval.distance != 5 || interval.quality != IntervalQuality::Perfect
                } else {
                    true
                }
            })
            .map(|interval| {
                self.root.add_semitones(interval.to_semitones() as i32)
            })
            .collect()
    }

    /// Parse a chord from a string (e.g., "Cmaj7", "Abm", "G7/B")
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ChordCraftError::InvalidChordName(s.to_string()));
        }

        // Check for slash chord (e.g., "C/G")
        if let Some(slash_pos) = s.find('/') {
            let chord_part = &s[..slash_pos];
            let bass_part = &s[slash_pos + 1..];

            let mut chord = Self::parse(chord_part)?;
            let bass = PitchClass::parse(bass_part)?;
            chord.bass = Some(bass);
            return Ok(chord);
        }

        // Parse root note (1-2 characters)
        let root_end = if s.len() > 1 && (s.as_bytes()[1] == b'#' || s.as_bytes()[1] == b'b') {
            2
        } else {
            1
        };

        let root = PitchClass::parse(&s[..root_end])?;
        let quality_str = &s[root_end..];

        // Parse quality
        let quality = Self::parse_quality(quality_str)?;

        Ok(Chord::new(root, quality))
    }

    /// Parse chord quality from string
    fn parse_quality(s: &str) -> Result<ChordQuality> {
        use ChordQuality::*;

        // Empty means major
        if s.is_empty() {
            return Ok(Major);
        }

        // Normalize the string
        let s = s.replace(['♭', '♯'], "");
        let s_lower = s.to_lowercase();

        // Try to match chord quality
        // Order matters - check longer patterns first!
        match s_lower.as_str() {
            // Minor variations
            "m(maj7)" | "mmaj7" | "mM7" | "minmaj7" => Ok(MinorMajor7),
            "m7b5" | "m7♭5" | "ø" | "half-dim" | "halfdim" => Ok(HalfDiminished7),
            "madd9" | "m(add9)" => Ok(MinorAdd9),
            "m13" | "min13" => Ok(Minor13),
            "m11" | "min11" => Ok(Minor11),
            "m9" | "min9" => Ok(Minor9),
            "m7" | "min7" => Ok(Minor7),
            "m6" | "min6" => Ok(Minor6),
            "m" | "min" | "-" => Ok(Minor),

            // Major 7th variations
            "maj13" | "M13" | "Δ13" => Ok(Major13),
            "maj9" | "M9" | "Δ9" => Ok(Major9),
            "maj7" | "M7" | "Δ7" | "Δ" => Ok(Major7),
            "maj" | "M" => Ok(Major),

            // Dominant variations
            "13" => Ok(Dominant13),
            "11" => Ok(Dominant11),
            "9" => Ok(Dominant9),
            "7#9" | "7♯9" => Ok(Dominant7sharp9),
            "7b9" | "7♭9" => Ok(Dominant7b9),
            "7#5" | "7♯5" | "7aug" | "+7" => Ok(Dominant7sharp5),
            "7b5" | "7♭5" => Ok(Dominant7b5),
            "7" => Ok(Dominant7),

            // Diminished
            "dim7" | "°7" | "o7" => Ok(Diminished7),
            "dim" | "°" | "o" => Ok(Diminished),

            // Augmented
            "aug" | "+" => Ok(Augmented),

            // Suspended
            "sus4" | "sus" => Ok(Sus4),
            "sus2" => Ok(Sus2),

            // Add chords
            "add11" => Ok(Add11),
            "add9" => Ok(Add9),

            // 6th chords
            "6" => Ok(Major6),

            _ => Err(ChordCraftError::InvalidChordName(format!(
                "Unknown chord quality: {s}"
            ))),
        }
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.root, self.quality.display_name())?;
        if let Some(bass) = self.bass {
            write!(f, "/{bass}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::PitchClass;

    #[test]
    fn test_chord_parse_basic() {
        let c_major = Chord::parse("C").unwrap();
        assert_eq!(c_major.root, PitchClass::C);
        assert_eq!(c_major.quality, ChordQuality::Major);

        let a_minor = Chord::parse("Am").unwrap();
        assert_eq!(a_minor.root, PitchClass::A);
        assert_eq!(a_minor.quality, ChordQuality::Minor);
    }

    #[test]
    fn test_chord_parse_seventh() {
        let cmaj7 = Chord::parse("Cmaj7").unwrap();
        assert_eq!(cmaj7.quality, ChordQuality::Major7);

        let g7 = Chord::parse("G7").unwrap();
        assert_eq!(g7.quality, ChordQuality::Dominant7);

        let dm7 = Chord::parse("Dm7").unwrap();
        assert_eq!(dm7.quality, ChordQuality::Minor7);
    }

    #[test]
    fn test_chord_parse_extended() {
        let cmaj9 = Chord::parse("Cmaj9").unwrap();
        assert_eq!(cmaj9.quality, ChordQuality::Major9);

        let g13 = Chord::parse("G13").unwrap();
        assert_eq!(g13.quality, ChordQuality::Dominant13);
    }

    #[test]
    fn test_chord_parse_accidentals() {
        let ab_minor = Chord::parse("Abm").unwrap();
        assert_eq!(ab_minor.root, PitchClass::GSharp);
        assert_eq!(ab_minor.quality, ChordQuality::Minor);

        let f_sharp_maj7 = Chord::parse("F#maj7").unwrap();
        assert_eq!(f_sharp_maj7.root, PitchClass::FSharp);
    }

    #[test]
    fn test_chord_parse_slash() {
        let c_over_g = Chord::parse("C/G").unwrap();
        assert_eq!(c_over_g.root, PitchClass::C);
        assert_eq!(c_over_g.bass, Some(PitchClass::G));
    }

    #[test]
    fn test_chord_notes() {
        let c_major = Chord::parse("C").unwrap();
        let notes = c_major.notes();
        assert!(notes.contains(&PitchClass::C));
        assert!(notes.contains(&PitchClass::E));
        assert!(notes.contains(&PitchClass::G));
    }

    #[test]
    fn test_chord_display() {
        assert_eq!(Chord::parse("Cmaj7").unwrap().to_string(), "Cmaj7");
        assert_eq!(Chord::parse("Am").unwrap().to_string(), "Am");
        assert_eq!(Chord::parse("G7").unwrap().to_string(), "G7");
    }
}
