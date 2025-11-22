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
}
