//! Instrument models and abstractions
//!
//! This module defines the Instrument trait and specific instrument implementations
//! like Guitar, Bass, Ukulele, etc.

use crate::note::Note;

/// Trait for stringed instruments
pub trait Instrument {
    /// Get the tuning of the instrument (notes for each string from lowest to highest)
    fn tuning(&self) -> &[Note];

    /// Get the fret range (min, max)
    fn fret_range(&self) -> (u8, u8);

    /// Maximum finger stretch (in frets)
    fn max_stretch(&self) -> u8;

    /// Number of strings
    fn string_count(&self) -> usize {
        self.tuning().len()
    }

    /// Maximum number of fretting fingers available (default 4)
    fn max_fingers(&self) -> u8 {
        4
    }

    /// Maximum fret position considered "open position" (default 4)
    fn open_position_threshold(&self) -> u8 {
        4
    }

    /// Minimum consecutive strings to be considered a "main barre"
    /// Default is 50% of strings, minimum 2
    fn main_barre_threshold(&self) -> usize {
        (self.string_count() / 2).max(2)
    }

    /// Minimum number of strings that must be played for a valid chord
    /// Default is half the strings, with a minimum of 2
    fn min_played_strings(&self) -> usize {
        (self.string_count() / 2).max(2)
    }
}

/// Standard guitar in EADGBE tuning
#[derive(Debug, Clone)]
pub struct Guitar {
    tuning: Vec<Note>,
    fret_range: (u8, u8),
    max_stretch: u8,
}

impl Default for Guitar {
    fn default() -> Self {
        use crate::note::PitchClass::*;

        Guitar {
            // Standard tuning: E2, A2, D3, G3, B3, E4
            tuning: vec![
                Note::new(E, 2),
                Note::new(A, 2),
                Note::new(D, 3),
                Note::new(G, 3),
                Note::new(B, 3),
                Note::new(E, 4),
            ],
            fret_range: (0, 24),
            max_stretch: 4,
        }
    }
}

impl Guitar {
    /// Create a capoed guitar at the specified fret
    ///
    /// This returns a `CapoedInstrument<Guitar>` that transposes the tuning
    /// up by the capo position and reduces the available fret range accordingly.
    ///
    /// # Examples
    ///
    /// ```
    /// use chordcraft_core::instrument::{Guitar, Instrument};
    /// use chordcraft_core::note::PitchClass;
    ///
    /// let guitar = Guitar::default();
    /// let capo_guitar = guitar.with_capo(3);
    ///
    /// // Open strings are now 3 semitones higher
    /// assert_eq!(capo_guitar.tuning()[0].pitch, PitchClass::G);  // E + 3 = G
    /// ```
    pub fn with_capo(&self, fret: u8) -> Self {
        // Transpose tuning up by capo frets
        // Use the Note::add_semitones method which handles octave changes correctly
        let new_tuning: Vec<Note> = self.tuning
            .iter()
            .map(|note| note.add_semitones(fret as i32))
            .collect();

        Guitar {
            tuning: new_tuning,
            // Reduce available frets (can't play beyond the physical frets)
            fret_range: (0, self.fret_range.1.saturating_sub(fret)),
            max_stretch: self.max_stretch,
        }
    }
}

impl Instrument for Guitar {
    fn tuning(&self) -> &[Note] {
        &self.tuning
    }

    fn fret_range(&self) -> (u8, u8) {
        self.fret_range
    }

    fn max_stretch(&self) -> u8 {
        self.max_stretch
    }
}

/// Ukulele in standard GCEA tuning (soprano/concert/tenor)
#[derive(Debug, Clone)]
pub struct Ukulele {
    tuning: Vec<Note>,
    fret_range: (u8, u8),
    max_stretch: u8,
}

impl Default for Ukulele {
    fn default() -> Self {
        use crate::note::PitchClass::*;

        Ukulele {
            // Standard ukulele tuning: G4 (re-entrant), C4, E4, A4
            tuning: vec![
                Note::new(G, 4),
                Note::new(C, 4),
                Note::new(E, 4),
                Note::new(A, 4),
            ],
            fret_range: (0, 15),
            max_stretch: 5, // Easier to stretch on shorter scale
        }
    }
}

impl Ukulele {
    /// Create a new Ukulele with a capo at the specified fret
    ///
    /// This transposes the tuning up by the capo position and reduces
    /// the available fret range accordingly.
    pub fn with_capo(&self, fret: u8) -> Self {
        // Transpose tuning up by capo frets
        // Use the Note::add_semitones method which handles octave changes correctly
        let new_tuning: Vec<Note> = self
            .tuning
            .iter()
            .map(|note| note.add_semitones(fret as i32))
            .collect();

        Ukulele {
            tuning: new_tuning,
            // Reduce available frets (can't play beyond the physical frets)
            fret_range: (0, self.fret_range.1.saturating_sub(fret)),
            max_stretch: self.max_stretch,
        }
    }
}

impl Instrument for Ukulele {
    fn tuning(&self) -> &[Note] {
        &self.tuning
    }

    fn fret_range(&self) -> (u8, u8) {
        self.fret_range
    }

    fn max_stretch(&self) -> u8 {
        self.max_stretch
    }

    // Ukulele has shorter scale, so "open position" extends a bit further
    fn open_position_threshold(&self) -> u8 {
        5
    }

    // With only 4 strings, a 2-string barre is already 50%
    // So we use the default: string_count/2 = 2
    fn main_barre_threshold(&self) -> usize {
        2
    }

    // Ukulele can have very minimal fingerings (e.g., C major is often just "0003")
    // Allow single-note voicings
    fn min_played_strings(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::PitchClass;

    #[test]
    fn test_guitar_with_capo_transposes_tuning() {
        let guitar = Guitar::default();
        let capo_guitar = guitar.with_capo(2);

        // Open E string (index 0) should now be F# (E + 2 semitones)
        assert_eq!(capo_guitar.tuning()[0].pitch, PitchClass::FSharp);

        // Open A string (index 1) should now be B (A + 2 semitones)
        assert_eq!(capo_guitar.tuning()[1].pitch, PitchClass::B);

        // Open D string (index 2) should now be E (D + 2 semitones)
        assert_eq!(capo_guitar.tuning()[2].pitch, PitchClass::E);
    }

    #[test]
    fn test_guitar_with_capo_reduces_fret_range() {
        let guitar = Guitar::default();
        let capo_guitar = guitar.with_capo(3);

        // Max fret should be reduced by capo position
        assert_eq!(capo_guitar.fret_range().1, guitar.fret_range().1 - 3);
    }

    #[test]
    fn test_guitar_with_capo_preserves_max_stretch() {
        let guitar = Guitar::default();
        let capo_guitar = guitar.with_capo(5);

        // Max stretch should remain the same
        assert_eq!(capo_guitar.max_stretch(), guitar.max_stretch());
    }

    #[test]
    fn test_ukulele_with_capo() {
        let ukulele = Ukulele::default();
        let capo_ukulele = ukulele.with_capo(2);

        // Open G string (index 0) should now be A (G + 2 semitones)
        assert_eq!(capo_ukulele.tuning()[0].pitch, PitchClass::A);

        // Open C string (index 1) should now be D (C + 2 semitones)
        assert_eq!(capo_ukulele.tuning()[1].pitch, PitchClass::D);

        // Fret range should be reduced
        assert_eq!(capo_ukulele.fret_range().1, ukulele.fret_range().1 - 2);
    }

    #[test]
    fn test_capo_at_zero_is_identity() {
        let guitar = Guitar::default();
        let capo_guitar = guitar.with_capo(0);

        // Should be identical to original
        assert_eq!(guitar.tuning()[0].pitch, capo_guitar.tuning()[0].pitch);
        assert_eq!(guitar.fret_range(), capo_guitar.fret_range());
    }

    #[test]
    fn test_high_capo_position() {
        let guitar = Guitar::default();
        let capo_guitar = guitar.with_capo(12);

        // Open E string should now be E an octave higher
        assert_eq!(capo_guitar.tuning()[0].pitch, PitchClass::E);
        // But octave should have increased
        assert_eq!(
            capo_guitar.tuning()[0].octave,
            guitar.tuning()[0].octave + 1
        );

        // Fret range should be significantly reduced
        assert_eq!(capo_guitar.fret_range().1, guitar.fret_range().1 - 12);
    }
}
