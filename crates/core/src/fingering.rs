//! Fingering representation for stringed instruments
//!
//! This module provides types for representing and working with chord fingerings
//! in tab notation format (e.g., "x32010" for C major on guitar).

use crate::error::{ChordCraftError, Result};
use crate::instrument::Instrument;
use crate::note::{Note, PitchClass};
use std::fmt;

/// Represents a single string's state in a fingering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringState {
    /// String is muted/not played
    Muted,
    /// String is played at a specific fret (0 = open)
    Fretted(u8),
}

impl StringState {
    /// Check if the string is played (not muted)
    pub fn is_played(&self) -> bool {
        matches!(self, StringState::Fretted(_))
    }

    /// Get the fret number if the string is played
    pub fn fret(&self) -> Option<u8> {
        match self {
            StringState::Muted => None,
            StringState::Fretted(f) => Some(*f),
        }
    }
}

/// A chord fingering on a stringed instrument
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fingering {
    /// State of each string, from lowest (bass) to highest (treble)
    strings: Vec<StringState>,
}

impl Fingering {
    /// Create a new fingering from string states
    pub fn new(strings: Vec<StringState>) -> Self {
        Fingering { strings }
    }

    /// Parse a fingering from tab notation
    ///
    /// Format: String of characters where each character represents a string
    /// - 'x' or 'X' = muted string
    /// - '0'-'9' = fret number 0-9
    /// - For frets 10+, use parentheses: "(10)" or "(12)"
    ///
    /// The notation is from lowest string to highest (E A D G B e for guitar)
    ///
    /// # Examples
    /// ```
    /// use chordcraft_core::fingering::Fingering;
    ///
    /// let c_major = Fingering::parse("x32010").unwrap();
    /// let barre_f = Fingering::parse("133211").unwrap();
    /// let high_fret = Fingering::parse("x(10)(10)9(10)x").unwrap();
    /// ```
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ChordCraftError::InvalidFingering("Empty fingering".to_string()));
        }

        let mut strings = Vec::new();
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            let state = match c {
                'x' | 'X' => StringState::Muted,
                '0'..='9' => StringState::Fretted(c.to_digit(10).unwrap() as u8),
                '(' => {
                    // Parse multi-digit fret number
                    let mut num_str = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == ')' {
                            chars.next(); // consume the ')'
                            break;
                        }
                        num_str.push(chars.next().unwrap());
                    }
                    let fret = num_str.parse::<u8>().map_err(|_| {
                        ChordCraftError::InvalidFingering(format!("Invalid fret number: {}", num_str))
                    })?;
                    StringState::Fretted(fret)
                }
                ' ' | '-' => continue, // Allow separators
                _ => {
                    return Err(ChordCraftError::InvalidFingering(format!(
                        "Invalid character in fingering: '{}'",
                        c
                    )))
                }
            };
            strings.push(state);
        }

        if strings.is_empty() {
            return Err(ChordCraftError::InvalidFingering("No strings found".to_string()));
        }

        Ok(Fingering { strings })
    }

    /// Get the string states
    pub fn strings(&self) -> &[StringState] {
        &self.strings
    }

    /// Get the number of strings
    pub fn string_count(&self) -> usize {
        self.strings.len()
    }

    /// Get the state of a specific string (0-indexed from lowest)
    pub fn get_string(&self, index: usize) -> Option<&StringState> {
        self.strings.get(index)
    }

    /// Get all fretted positions (excluding muted and open strings)
    pub fn fretted_positions(&self) -> Vec<(usize, u8)> {
        self.strings
            .iter()
            .enumerate()
            .filter_map(|(i, s)| match s {
                StringState::Fretted(f) if *f > 0 => Some((i, *f)),
                _ => None,
            })
            .collect()
    }

    /// Get the lowest fret position (excluding open strings)
    pub fn min_fret(&self) -> Option<u8> {
        self.strings
            .iter()
            .filter_map(|s| match s {
                StringState::Fretted(f) if *f > 0 => Some(*f),
                _ => None,
            })
            .min()
    }

    /// Get the highest fret position
    pub fn max_fret(&self) -> Option<u8> {
        self.strings
            .iter()
            .filter_map(|s| s.fret())
            .max()
    }

    /// Calculate the fret span (stretch required)
    pub fn fret_span(&self) -> u8 {
        let fretted: Vec<u8> = self.strings
            .iter()
            .filter_map(|s| match s {
                StringState::Fretted(f) if *f > 0 => Some(*f),
                _ => None,
            })
            .collect();

        if fretted.is_empty() {
            return 0;
        }

        let min = *fretted.iter().min().unwrap();
        let max = *fretted.iter().max().unwrap();
        max - min
    }

    /// Check if this is an open position chord (includes open strings, low frets)
    pub fn is_open_position(&self) -> bool {
        self.strings.iter().any(|s| matches!(s, StringState::Fretted(0)))
            && self.max_fret().unwrap_or(0) <= 4
    }

    /// Check if this fingering requires a barre
    pub fn requires_barre(&self) -> bool {
        if let Some(min) = self.min_fret() {
            let count_at_min = self.strings
                .iter()
                .filter(|s| matches!(s, StringState::Fretted(f) if *f == min))
                .count();
            count_at_min >= 2
        } else {
            false
        }
    }

    /// Check if the fingering is physically playable given max stretch
    pub fn is_playable(&self, max_stretch: u8) -> bool {
        self.fret_span() <= max_stretch
    }

    /// Get the notes produced by this fingering on a given instrument
    pub fn notes<I: Instrument>(&self, instrument: &I) -> Vec<Note> {
        let tuning = instrument.tuning();

        self.strings
            .iter()
            .enumerate()
            .filter_map(|(i, state)| {
                if i >= tuning.len() {
                    return None;
                }
                match state {
                    StringState::Muted => None,
                    StringState::Fretted(fret) => {
                        Some(tuning[i].add_semitones(*fret as i32))
                    }
                }
            })
            .collect()
    }

    /// Get the pitch classes (notes without octave) produced by this fingering
    pub fn pitch_classes<I: Instrument>(&self, instrument: &I) -> Vec<PitchClass> {
        self.notes(instrument)
            .into_iter()
            .map(|n| n.pitch)
            .collect()
    }

    /// Get unique pitch classes (deduplicated)
    pub fn unique_pitch_classes<I: Instrument>(&self, instrument: &I) -> Vec<PitchClass> {
        let mut pitches = self.pitch_classes(instrument);
        pitches.sort_by_key(|p| p.to_semitone());
        pitches.dedup();
        pitches
    }

    /// Calculate a playability score (0-100, higher is easier to play)
    pub fn playability_score(&self, max_stretch: u8) -> u8 {
        let mut score: i32 = 100;

        // Penalize for stretch
        let span = self.fret_span();
        if span > max_stretch {
            return 0; // Unplayable
        }
        score -= (span as i32) * 10;

        // Penalize for barre chords
        if self.requires_barre() {
            score -= 15;
        }

        // Bonus for open position
        if self.is_open_position() {
            score += 10;
        }

        // Penalize for high fret positions (harder to reach)
        if let Some(min) = self.min_fret()
            && min > 7
        {
            score -= ((min - 7) as i32) * 2;
        }

        // Penalize for many muted strings in the middle
        let muted_count = self.strings.iter().filter(|s| !s.is_played()).count();
        if muted_count > 1 {
            score -= ((muted_count - 1) as i32) * 5;
        }

        score.clamp(0, 100) as u8
    }

    /// Get the bass note (lowest pitched note that's played)
    pub fn bass_note<I: Instrument>(&self, instrument: &I) -> Option<Note> {
        let tuning = instrument.tuning();

        for (i, state) in self.strings.iter().enumerate() {
            if i >= tuning.len() {
                break;
            }
            if let StringState::Fretted(fret) = state {
                return Some(tuning[i].add_semitones(*fret as i32));
            }
        }
        None
    }
}

impl fmt::Display for Fingering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for state in &self.strings {
            match state {
                StringState::Muted => write!(f, "x")?,
                StringState::Fretted(fret) if *fret < 10 => write!(f, "{}", fret)?,
                StringState::Fretted(fret) => write!(f, "({})", fret)?,
            }
        }
        Ok(())
    }
}

/// Builder for creating fingerings programmatically
pub struct FingeringBuilder {
    strings: Vec<StringState>,
}

impl FingeringBuilder {
    /// Create a new builder for an instrument with the given number of strings
    pub fn new(string_count: usize) -> Self {
        FingeringBuilder {
            strings: vec![StringState::Muted; string_count],
        }
    }

    /// Set a string to a specific fret
    pub fn fret(mut self, string: usize, fret: u8) -> Self {
        if string < self.strings.len() {
            self.strings[string] = StringState::Fretted(fret);
        }
        self
    }

    /// Set a string to muted
    pub fn mute(mut self, string: usize) -> Self {
        if string < self.strings.len() {
            self.strings[string] = StringState::Muted;
        }
        self
    }

    /// Build the fingering
    pub fn build(self) -> Fingering {
        Fingering::new(self.strings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instrument::Guitar;

    #[test]
    fn test_parse_simple() {
        let f = Fingering::parse("x32010").unwrap();
        assert_eq!(f.string_count(), 6);
        assert_eq!(f.strings[0], StringState::Muted);
        assert_eq!(f.strings[1], StringState::Fretted(3));
        assert_eq!(f.strings[2], StringState::Fretted(2));
        assert_eq!(f.strings[3], StringState::Fretted(0));
        assert_eq!(f.strings[4], StringState::Fretted(1));
        assert_eq!(f.strings[5], StringState::Fretted(0));
    }

    #[test]
    fn test_parse_barre() {
        let f = Fingering::parse("133211").unwrap();
        assert_eq!(f.string_count(), 6);
        assert!(f.requires_barre());
    }

    #[test]
    fn test_parse_high_frets() {
        let f = Fingering::parse("x(10)(10)9(10)x").unwrap();
        assert_eq!(f.string_count(), 6);
        assert_eq!(f.strings[1], StringState::Fretted(10));
        assert_eq!(f.strings[3], StringState::Fretted(9));
    }

    #[test]
    fn test_display() {
        let f = Fingering::parse("x32010").unwrap();
        assert_eq!(f.to_string(), "x32010");

        let f2 = Fingering::parse("x(10)(10)9(10)x").unwrap();
        assert_eq!(f2.to_string(), "x(10)(10)9(10)x");
    }

    #[test]
    fn test_fret_span() {
        let f = Fingering::parse("x32010").unwrap();
        assert_eq!(f.fret_span(), 2); // frets 1, 2, 3 -> span is 3-1=2

        let open = Fingering::parse("022100").unwrap();
        assert_eq!(open.fret_span(), 1); // frets 1, 2 -> span is 2-1=1
    }

    #[test]
    fn test_is_open_position() {
        let c = Fingering::parse("x32010").unwrap();
        assert!(c.is_open_position());

        let barre_f = Fingering::parse("133211").unwrap();
        assert!(!barre_f.is_open_position());
    }

    #[test]
    fn test_notes_on_guitar() {
        let guitar = Guitar::default();
        let c_major = Fingering::parse("x32010").unwrap();
        let notes = c_major.notes(&guitar);

        // Should have 5 notes (one string muted)
        assert_eq!(notes.len(), 5);

        // Check pitch classes
        let pitches = c_major.unique_pitch_classes(&guitar);
        assert!(pitches.contains(&PitchClass::C));
        assert!(pitches.contains(&PitchClass::E));
        assert!(pitches.contains(&PitchClass::G));
    }

    #[test]
    fn test_playability() {
        let easy = Fingering::parse("x32010").unwrap();
        assert!(easy.is_playable(4));
        assert!(easy.playability_score(4) > 50);

        let hard = Fingering::parse("x24442").unwrap();
        assert!(hard.playability_score(4) < easy.playability_score(4));
    }

    #[test]
    fn test_builder() {
        let f = FingeringBuilder::new(6)
            .mute(0)
            .fret(1, 3)
            .fret(2, 2)
            .fret(3, 0)
            .fret(4, 1)
            .fret(5, 0)
            .build();

        assert_eq!(f.to_string(), "x32010");
    }

    #[test]
    fn test_bass_note() {
        let guitar = Guitar::default();
        let c_major = Fingering::parse("x32010").unwrap();
        let bass = c_major.bass_note(&guitar).unwrap();
        assert_eq!(bass.pitch, PitchClass::C); // C on 3rd fret of A string
    }
}
