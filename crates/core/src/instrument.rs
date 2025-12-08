//! Instrument models and abstractions
//!
//! This module defines the Instrument trait and specific instrument implementations
//! like Guitar, Bass, Ukulele, etc.

use crate::error::{ChordCraftError, Result};
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

	/// Maximum reasonable capo position for this instrument
	/// Default is 12 frets (one octave), or half the fret range, whichever is smaller
	fn max_capo_fret(&self) -> u8 {
		12.min(self.fret_range().1 / 2)
	}

	/// Get display names for strings (used in diagrams)
	/// Returns names from lowest to highest pitch string
	/// Default implementation uses the pitch class name of each open string
	fn string_names(&self) -> Vec<String> {
		self.tuning()
			.iter()
			.map(|note| note.pitch.to_string())
			.collect()
	}
}

/// Generic wrapper for an instrument with a capo
///
/// This transposes the tuning up by the capo position and reduces
/// the available fret range accordingly, while delegating all other
/// instrument properties to the wrapped instrument.
///
/// # Examples
///
/// ```
/// use chordcraft_core::instrument::{Guitar, CapoedInstrument, Instrument};
/// use chordcraft_core::note::PitchClass;
///
/// let guitar = Guitar::default();
/// let capo_guitar = CapoedInstrument::new(guitar, 3).unwrap();
///
/// // Open strings are now 3 semitones higher
/// assert_eq!(capo_guitar.tuning()[0].pitch, PitchClass::G);  // E + 3 = G
/// ```
///
/// # Errors
///
/// Returns an error if the capo position exceeds the instrument's maximum capo fret.
#[derive(Debug, Clone)]
pub struct CapoedInstrument<I: Instrument> {
	inner: I,
	tuning: Vec<Note>,
	fret_range: (u8, u8),
}

impl<I: Instrument> CapoedInstrument<I> {
	/// Create a capoed instrument at the specified fret
	///
	/// # Arguments
	///
	/// * `instrument` - The base instrument to apply the capo to
	/// * `fret` - The fret position of the capo (0 means no capo)
	///
	/// # Errors
	///
	/// Returns `ChordCraftError::InvalidCapoPosition` if the fret position is invalid.
	/// Valid range is 0 to the instrument's `max_capo_fret()`.
	///
	/// # Examples
	///
	/// ```
	/// use chordcraft_core::instrument::{Guitar, CapoedInstrument};
	///
	/// let guitar = Guitar::default();
	///
	/// // Valid capo position
	/// let capo_guitar = CapoedInstrument::new(guitar.clone(), 5).unwrap();
	///
	/// // Invalid capo position (too high)
	/// let result = CapoedInstrument::new(guitar, 20);
	/// assert!(result.is_err());
	/// ```
	pub fn new(instrument: I, fret: u8) -> Result<Self> {
		let max_capo = instrument.max_capo_fret();

		if fret > max_capo {
			return Err(ChordCraftError::InvalidCapoPosition(fret, 0, max_capo));
		}

		let tuning: Vec<Note> = instrument
			.tuning()
			.iter()
			.map(|note| note.add_semitones(fret as i32))
			.collect();

		let fret_range = (0, instrument.fret_range().1.saturating_sub(fret));

		Ok(CapoedInstrument {
			inner: instrument,
			tuning,
			fret_range,
		})
	}

	/// Get the underlying instrument
	pub fn inner(&self) -> &I {
		&self.inner
	}
}

impl<I: Instrument> Instrument for CapoedInstrument<I> {
	fn tuning(&self) -> &[Note] {
		&self.tuning
	}

	fn fret_range(&self) -> (u8, u8) {
		self.fret_range
	}

	fn max_stretch(&self) -> u8 {
		self.inner.max_stretch()
	}

	fn string_count(&self) -> usize {
		self.inner.string_count()
	}

	fn max_fingers(&self) -> u8 {
		self.inner.max_fingers()
	}

	fn open_position_threshold(&self) -> u8 {
		self.inner.open_position_threshold()
	}

	fn main_barre_threshold(&self) -> usize {
		self.inner.main_barre_threshold()
	}

	fn min_played_strings(&self) -> usize {
		self.inner.min_played_strings()
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
	/// let capo_guitar = guitar.with_capo(3).unwrap();
	///
	/// // Open strings are now 3 semitones higher
	/// assert_eq!(capo_guitar.tuning()[0].pitch, PitchClass::G);  // E + 3 = G
	/// ```
	///
	/// # Errors
	///
	/// Returns an error if the capo position exceeds the maximum capo fret.
	pub fn with_capo(&self, fret: u8) -> Result<CapoedInstrument<Guitar>> {
		CapoedInstrument::new(self.clone(), fret)
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

	/// Guitar string names use lowercase 'e' for high E (convention)
	fn string_names(&self) -> Vec<String> {
		vec![
			"E".to_string(), // Low E
			"A".to_string(),
			"D".to_string(),
			"G".to_string(),
			"B".to_string(),
			"e".to_string(), // High e
		]
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
	/// Create a capoed ukulele at the specified fret
	///
	/// This returns a `CapoedInstrument<Ukulele>` that transposes the tuning
	/// up by the capo position and reduces the available fret range accordingly.
	///
	/// # Errors
	///
	/// Returns an error if the capo position exceeds the maximum capo fret.
	pub fn with_capo(&self, fret: u8) -> Result<CapoedInstrument<Ukulele>> {
		CapoedInstrument::new(self.clone(), fret)
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
		let capo_guitar = guitar.with_capo(2).unwrap();

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
		let capo_guitar = guitar.with_capo(3).unwrap();

		// Max fret should be reduced by capo position
		assert_eq!(capo_guitar.fret_range().1, guitar.fret_range().1 - 3);
	}

	#[test]
	fn test_guitar_with_capo_preserves_max_stretch() {
		let guitar = Guitar::default();
		let capo_guitar = guitar.with_capo(5).unwrap();

		// Max stretch should remain the same
		assert_eq!(capo_guitar.max_stretch(), guitar.max_stretch());
	}

	#[test]
	fn test_ukulele_with_capo() {
		let ukulele = Ukulele::default();
		let capo_ukulele = ukulele.with_capo(2).unwrap();

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
		let capo_guitar = guitar.with_capo(0).unwrap();

		// Should be identical to original
		assert_eq!(guitar.tuning()[0].pitch, capo_guitar.tuning()[0].pitch);
		assert_eq!(guitar.fret_range(), capo_guitar.fret_range());
	}

	#[test]
	fn test_high_capo_position() {
		let guitar = Guitar::default();
		let capo_guitar = guitar.with_capo(12).unwrap();

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

	#[test]
	fn test_invalid_capo_position() {
		let guitar = Guitar::default();

		// Capo beyond max should fail
		let result = guitar.with_capo(20);
		assert!(result.is_err());

		// Error should be InvalidCapoPosition
		if let Err(ChordCraftError::InvalidCapoPosition(fret, min, max)) = result {
			assert_eq!(fret, 20);
			assert_eq!(min, 0);
			assert_eq!(max, 12); // Guitar max_capo_fret is 12
		} else {
			panic!("Expected InvalidCapoPosition error");
		}
	}

	#[test]
	fn test_max_capo_fret() {
		let guitar = Guitar::default();
		assert_eq!(guitar.max_capo_fret(), 12); // 24 frets / 2 = 12

		let ukulele = Ukulele::default();
		assert_eq!(ukulele.max_capo_fret(), 7); // 15 frets / 2 = 7
	}
}
