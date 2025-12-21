//! Instrument models and abstractions
//!
//! This module defines the Instrument trait and specific instrument implementations
//! like Guitar, Bass, Ukulele, etc.

use crate::error::{ChordCraftError, Result};
use crate::note::Note;

pub trait Instrument {
	fn tuning(&self) -> &[Note];
	fn fret_range(&self) -> (u8, u8);
	fn max_stretch(&self) -> u8;

	fn string_count(&self) -> usize {
		self.tuning().len()
	}

	fn max_fingers(&self) -> u8 {
		4
	}

	fn open_position_threshold(&self) -> u8 {
		4
	}

	/// Default: 50% of strings, minimum 2.
	fn main_barre_threshold(&self) -> usize {
		(self.string_count() / 2).max(2)
	}

	fn min_played_strings(&self) -> usize {
		(self.string_count() / 2).max(2)
	}

	fn max_capo_fret(&self) -> u8 {
		12.min(self.fret_range().1 / 2)
	}

	fn string_names(&self) -> Vec<String> {
		self.tuning()
			.iter()
			.map(|note| note.pitch.to_string())
			.collect()
	}

	/// For re-entrant tunings, returns the lowest-pitched string (not necessarily index 0).
	fn bass_string_index(&self) -> usize {
		0
	}

	/// Returns indices of strings whose open note is in the bass register (below C3).
	///
	/// This is used for band mode scoring - when playing with a bass player,
	/// fingerings that avoid these strings are preferred to not conflict.
	///
	/// Returns:
	/// - `None` if ALL strings are in bass register (e.g., bass guitar) - this IS a bass instrument
	/// - `Some(vec![])` if NO strings are in bass register (e.g., ukulele) - no avoidance needed
	/// - `Some(indices)` if SOME strings are in bass register (e.g., guitar) - avoid those in band mode
	fn bass_string_indices(&self) -> Option<Vec<usize>> {
		let bass_indices: Vec<usize> = self
			.tuning()
			.iter()
			.enumerate()
			.filter(|(_, note)| note.is_bass_register())
			.map(|(i, _)| i)
			.collect();

		if bass_indices.len() == self.string_count() {
			None // All strings are bass - this IS a bass instrument
		} else {
			Some(bass_indices)
		}
	}
}

/// Transposes tuning up and reduces fret range. Delegates other properties to inner instrument.
#[derive(Debug, Clone)]
pub struct CapoedInstrument<I: Instrument> {
	inner: I,
	tuning: Vec<Note>,
	fret_range: (u8, u8),
}

impl<I: Instrument> CapoedInstrument<I> {
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

	fn bass_string_index(&self) -> usize {
		self.inner.bass_string_index()
	}
}

/// A fully configurable instrument where all parameters can be set.
///
/// This allows creating any stringed instrument by specifying tuning and
/// physical characteristics. Use the builder pattern for ergonomic construction.
///
/// # Example
///
/// ```
/// use chordcraft_core::instrument::ConfigurableInstrument;
/// use chordcraft_core::note::{Note, PitchClass};
///
/// // Create a bass guitar
/// let bass = ConfigurableInstrument::builder()
///     .tuning(vec![
///         Note::new(PitchClass::E, 1),
///         Note::new(PitchClass::A, 1),
///         Note::new(PitchClass::D, 2),
///         Note::new(PitchClass::G, 2),
///     ])
///     .fret_range(0, 24)
///     .max_stretch(4)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct ConfigurableInstrument {
	name: String,
	tuning: Vec<Note>,
	fret_range: (u8, u8),
	max_stretch: u8,
	// Optional overrides (None = use default formula/value)
	max_fingers: Option<u8>,
	open_position_threshold: Option<u8>,
	main_barre_threshold: Option<usize>,
	min_played_strings: Option<usize>,
	bass_string_index: Option<usize>,
	string_names: Option<Vec<String>>,
}

impl ConfigurableInstrument {
	/// Create a new builder for ConfigurableInstrument
	pub fn builder() -> ConfigurableInstrumentBuilder {
		ConfigurableInstrumentBuilder::default()
	}

	/// Apply a capo at the specified fret
	pub fn with_capo(&self, fret: u8) -> Result<CapoedInstrument<ConfigurableInstrument>> {
		CapoedInstrument::new(self.clone(), fret)
	}

	/// Get the instrument name
	pub fn name(&self) -> &str {
		&self.name
	}

	// ==================== INSTRUMENT PRESETS ====================

	/// Standard 4-string bass guitar (E1-A1-D2-G2)
	pub fn bass() -> Self {
		use crate::note::PitchClass::*;
		ConfigurableInstrument {
			name: "Bass".to_string(),
			tuning: vec![
				Note::new(E, 1),
				Note::new(A, 1),
				Note::new(D, 2),
				Note::new(G, 2),
			],
			fret_range: (0, 24),
			max_stretch: 4,
			max_fingers: None,
			open_position_threshold: None,
			main_barre_threshold: None,
			min_played_strings: Some(1), // Bass often plays single notes
			bass_string_index: None,
			string_names: Some(vec![
				"E".to_string(),
				"A".to_string(),
				"D".to_string(),
				"G".to_string(),
			]),
		}
	}

	/// 5-string bass guitar (B0-E1-A1-D2-G2)
	pub fn bass_5_string() -> Self {
		use crate::note::PitchClass::*;
		ConfigurableInstrument {
			name: "Bass (5-string)".to_string(),
			tuning: vec![
				Note::new(B, 0),
				Note::new(E, 1),
				Note::new(A, 1),
				Note::new(D, 2),
				Note::new(G, 2),
			],
			fret_range: (0, 24),
			max_stretch: 4,
			max_fingers: None,
			open_position_threshold: None,
			main_barre_threshold: None,
			min_played_strings: Some(1),
			bass_string_index: None,
			string_names: Some(vec![
				"B".to_string(),
				"E".to_string(),
				"A".to_string(),
				"D".to_string(),
				"G".to_string(),
			]),
		}
	}

	/// Standard mandolin (G3-D4-A4-E5)
	pub fn mandolin() -> Self {
		use crate::note::PitchClass::*;
		ConfigurableInstrument {
			name: "Mandolin".to_string(),
			tuning: vec![
				Note::new(G, 3),
				Note::new(D, 4),
				Note::new(A, 4),
				Note::new(E, 5),
			],
			fret_range: (0, 17),
			max_stretch: 4,
			max_fingers: None,
			open_position_threshold: Some(5),
			main_barre_threshold: None,
			min_played_strings: Some(2),
			bass_string_index: None,
			string_names: Some(vec![
				"G".to_string(),
				"D".to_string(),
				"A".to_string(),
				"E".to_string(),
			]),
		}
	}

	/// Standard 5-string banjo with high G drone (gDGBD - G4-D3-G3-B3-D4)
	pub fn banjo() -> Self {
		use crate::note::PitchClass::*;
		ConfigurableInstrument {
			name: "Banjo".to_string(),
			tuning: vec![
				Note::new(G, 4), // High G drone (5th string, shorter)
				Note::new(D, 3),
				Note::new(G, 3),
				Note::new(B, 3),
				Note::new(D, 4),
			],
			fret_range: (0, 22),
			max_stretch: 4,
			max_fingers: None,
			open_position_threshold: Some(5),
			main_barre_threshold: None,
			min_played_strings: Some(2),
			bass_string_index: Some(1), // D3 is the actual bass, not the high G drone
			string_names: Some(vec![
				"g".to_string(), // lowercase for drone
				"D".to_string(),
				"G".to_string(),
				"B".to_string(),
				"d".to_string(),
			]),
		}
	}

	/// Baritone ukulele (DGBE - same as guitar's top 4 strings)
	pub fn baritone_ukulele() -> Self {
		use crate::note::PitchClass::*;
		ConfigurableInstrument {
			name: "Baritone Ukulele".to_string(),
			tuning: vec![
				Note::new(D, 3),
				Note::new(G, 3),
				Note::new(B, 3),
				Note::new(E, 4),
			],
			fret_range: (0, 18),
			max_stretch: 5,
			max_fingers: None,
			open_position_threshold: Some(5),
			main_barre_threshold: Some(2),
			min_played_strings: Some(1),
			bass_string_index: None,
			string_names: Some(vec![
				"D".to_string(),
				"G".to_string(),
				"B".to_string(),
				"E".to_string(),
			]),
		}
	}

	/// 7-string guitar with low B (B1-E2-A2-D3-G3-B3-E4)
	pub fn guitar_7_string() -> Self {
		use crate::note::PitchClass::*;
		ConfigurableInstrument {
			name: "Guitar (7-string)".to_string(),
			tuning: vec![
				Note::new(B, 1),
				Note::new(E, 2),
				Note::new(A, 2),
				Note::new(D, 3),
				Note::new(G, 3),
				Note::new(B, 3),
				Note::new(E, 4),
			],
			fret_range: (0, 24),
			max_stretch: 4,
			max_fingers: None,
			open_position_threshold: None,
			main_barre_threshold: None,
			min_played_strings: None,
			bass_string_index: None,
			string_names: Some(vec![
				"B".to_string(),
				"E".to_string(),
				"A".to_string(),
				"D".to_string(),
				"G".to_string(),
				"B".to_string(),
				"e".to_string(),
			]),
		}
	}

	/// Drop D guitar tuning (D2-A2-D3-G3-B3-E4)
	pub fn guitar_drop_d() -> Self {
		use crate::note::PitchClass::*;
		ConfigurableInstrument {
			name: "Guitar (Drop D)".to_string(),
			tuning: vec![
				Note::new(D, 2),
				Note::new(A, 2),
				Note::new(D, 3),
				Note::new(G, 3),
				Note::new(B, 3),
				Note::new(E, 4),
			],
			fret_range: (0, 24),
			max_stretch: 4,
			max_fingers: None,
			open_position_threshold: None,
			main_barre_threshold: None,
			min_played_strings: None,
			bass_string_index: None,
			string_names: Some(vec![
				"D".to_string(),
				"A".to_string(),
				"D".to_string(),
				"G".to_string(),
				"B".to_string(),
				"e".to_string(),
			]),
		}
	}

	/// Open G guitar tuning (D2-G2-D3-G3-B3-D4) - popular for slide guitar
	pub fn guitar_open_g() -> Self {
		use crate::note::PitchClass::*;
		ConfigurableInstrument {
			name: "Guitar (Open G)".to_string(),
			tuning: vec![
				Note::new(D, 2),
				Note::new(G, 2),
				Note::new(D, 3),
				Note::new(G, 3),
				Note::new(B, 3),
				Note::new(D, 4),
			],
			fret_range: (0, 24),
			max_stretch: 4,
			max_fingers: None,
			open_position_threshold: None,
			main_barre_threshold: None,
			min_played_strings: None,
			bass_string_index: None,
			string_names: Some(vec![
				"D".to_string(),
				"G".to_string(),
				"D".to_string(),
				"G".to_string(),
				"B".to_string(),
				"d".to_string(),
			]),
		}
	}

	/// DADGAD guitar tuning - popular for Celtic and folk music
	pub fn guitar_dadgad() -> Self {
		use crate::note::PitchClass::*;
		ConfigurableInstrument {
			name: "Guitar (DADGAD)".to_string(),
			tuning: vec![
				Note::new(D, 2),
				Note::new(A, 2),
				Note::new(D, 3),
				Note::new(G, 3),
				Note::new(A, 3),
				Note::new(D, 4),
			],
			fret_range: (0, 24),
			max_stretch: 4,
			max_fingers: None,
			open_position_threshold: None,
			main_barre_threshold: None,
			min_played_strings: None,
			bass_string_index: None,
			string_names: Some(vec![
				"D".to_string(),
				"A".to_string(),
				"D".to_string(),
				"G".to_string(),
				"A".to_string(),
				"d".to_string(),
			]),
		}
	}
}

impl Instrument for ConfigurableInstrument {
	fn tuning(&self) -> &[Note] {
		&self.tuning
	}

	fn fret_range(&self) -> (u8, u8) {
		self.fret_range
	}

	fn max_stretch(&self) -> u8 {
		self.max_stretch
	}

	fn max_fingers(&self) -> u8 {
		self.max_fingers.unwrap_or(4)
	}

	fn open_position_threshold(&self) -> u8 {
		self.open_position_threshold.unwrap_or(4)
	}

	fn main_barre_threshold(&self) -> usize {
		self.main_barre_threshold
			.unwrap_or_else(|| (self.string_count() / 2).max(2))
	}

	fn min_played_strings(&self) -> usize {
		self.min_played_strings
			.unwrap_or_else(|| (self.string_count() / 2).max(2))
	}

	fn bass_string_index(&self) -> usize {
		self.bass_string_index.unwrap_or(0)
	}

	fn string_names(&self) -> Vec<String> {
		self.string_names.clone().unwrap_or_else(|| {
			self.tuning
				.iter()
				.map(|note| note.pitch.to_string())
				.collect()
		})
	}
}

/// Builder for creating ConfigurableInstrument instances
#[derive(Debug, Default)]
pub struct ConfigurableInstrumentBuilder {
	name: Option<String>,
	tuning: Option<Vec<Note>>,
	fret_range: Option<(u8, u8)>,
	max_stretch: Option<u8>,
	max_fingers: Option<u8>,
	open_position_threshold: Option<u8>,
	main_barre_threshold: Option<usize>,
	min_played_strings: Option<usize>,
	bass_string_index: Option<usize>,
	string_names: Option<Vec<String>>,
}

impl ConfigurableInstrumentBuilder {
	/// Set the instrument name (optional, defaults to "Custom Instrument")
	pub fn name(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}

	/// Set the tuning (required)
	pub fn tuning(mut self, tuning: Vec<Note>) -> Self {
		self.tuning = Some(tuning);
		self
	}

	/// Set the fret range as (min, max) - typically (0, 24) for guitar
	pub fn fret_range(mut self, min: u8, max: u8) -> Self {
		self.fret_range = Some((min, max));
		self
	}

	/// Set the maximum stretch in frets (required)
	pub fn max_stretch(mut self, stretch: u8) -> Self {
		self.max_stretch = Some(stretch);
		self
	}

	/// Override the maximum number of fingers (default: 4)
	pub fn max_fingers(mut self, fingers: u8) -> Self {
		self.max_fingers = Some(fingers);
		self
	}

	/// Override the open position threshold (default: 4)
	pub fn open_position_threshold(mut self, threshold: u8) -> Self {
		self.open_position_threshold = Some(threshold);
		self
	}

	/// Override the main barre threshold (default: string_count / 2, min 2)
	pub fn main_barre_threshold(mut self, threshold: usize) -> Self {
		self.main_barre_threshold = Some(threshold);
		self
	}

	/// Override minimum played strings (default: string_count / 2, min 2)
	pub fn min_played_strings(mut self, min: usize) -> Self {
		self.min_played_strings = Some(min);
		self
	}

	/// Override bass string index for re-entrant tunings (default: 0)
	pub fn bass_string_index(mut self, index: usize) -> Self {
		self.bass_string_index = Some(index);
		self
	}

	/// Override string names for display (default: derived from pitch classes)
	pub fn string_names(mut self, names: Vec<String>) -> Self {
		self.string_names = Some(names);
		self
	}

	/// Build the ConfigurableInstrument, returning an error if required fields are missing
	pub fn build(self) -> Result<ConfigurableInstrument> {
		let tuning = self
			.tuning
			.ok_or_else(|| ChordCraftError::InvalidInstrument("tuning is required".to_string()))?;

		if tuning.is_empty() {
			return Err(ChordCraftError::InvalidInstrument(
				"tuning must have at least one string".to_string(),
			));
		}

		let fret_range = self.fret_range.ok_or_else(|| {
			ChordCraftError::InvalidInstrument("fret_range is required".to_string())
		})?;

		let max_stretch = self.max_stretch.ok_or_else(|| {
			ChordCraftError::InvalidInstrument("max_stretch is required".to_string())
		})?;

		// Validate string_names length if provided
		if let Some(ref names) = self.string_names
			&& names.len() != tuning.len()
		{
			return Err(ChordCraftError::InvalidInstrument(format!(
				"string_names length ({}) must match tuning length ({})",
				names.len(),
				tuning.len()
			)));
		}

		// Validate bass_string_index if provided
		if let Some(index) = self.bass_string_index
			&& index >= tuning.len()
		{
			return Err(ChordCraftError::InvalidInstrument(format!(
				"bass_string_index ({}) must be less than string count ({})",
				index,
				tuning.len()
			)));
		}

		Ok(ConfigurableInstrument {
			name: self.name.unwrap_or_else(|| "Custom Instrument".to_string()),
			tuning,
			fret_range,
			max_stretch,
			max_fingers: self.max_fingers,
			open_position_threshold: self.open_position_threshold,
			main_barre_threshold: self.main_barre_threshold,
			min_played_strings: self.min_played_strings,
			bass_string_index: self.bass_string_index,
			string_names: self.string_names,
		})
	}
}

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

#[derive(Debug, Clone)]
pub struct Ukulele {
	tuning: Vec<Note>,
	fret_range: (u8, u8),
	max_stretch: u8,
}

impl Default for Ukulele {
	fn default() -> Self {
		use crate::note::PitchClass::*;

		// Re-entrant tuning: G4 is higher than C4
		Ukulele {
			tuning: vec![
				Note::new(G, 4),
				Note::new(C, 4),
				Note::new(E, 4),
				Note::new(A, 4),
			],
			fret_range: (0, 15),
			max_stretch: 5,
		}
	}
}

impl Ukulele {
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

	fn open_position_threshold(&self) -> u8 {
		5
	}

	fn main_barre_threshold(&self) -> usize {
		2
	}

	fn min_played_strings(&self) -> usize {
		1
	}

	/// C string (index 1) is the lowest pitch due to re-entrant tuning.
	fn bass_string_index(&self) -> usize {
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

	#[test]
	fn test_guitar_bass_string_indices() {
		let guitar = Guitar::default();
		let bass_indices = guitar.bass_string_indices();

		// Guitar has some bass strings (E2, A2 are below C3)
		assert!(bass_indices.is_some());
		let indices = bass_indices.unwrap();

		// Low E (index 0) and A (index 1) are in bass register
		assert!(indices.contains(&0), "Low E should be bass");
		assert!(indices.contains(&1), "A string should be bass");

		// D3, G3, B3, E4 are NOT in bass register
		assert!(!indices.contains(&2), "D string should not be bass");
		assert!(!indices.contains(&3), "G string should not be bass");
		assert!(!indices.contains(&4), "B string should not be bass");
		assert!(!indices.contains(&5), "High E should not be bass");
	}

	#[test]
	fn test_ukulele_has_no_bass_strings() {
		let ukulele = Ukulele::default();
		let bass_indices = ukulele.bass_string_indices();

		// Ukulele (G4, C4, E4, A4) has no strings in bass register
		assert!(bass_indices.is_some());
		let indices = bass_indices.unwrap();
		assert!(indices.is_empty(), "Ukulele should have no bass strings");
	}

	#[test]
	fn test_capo_affects_bass_string_indices() {
		let guitar = Guitar::default();

		// With capo at 12, all strings are transposed up an octave
		// E2->E3, A2->A3, etc. - now all above C3 threshold
		let capo_guitar = guitar.with_capo(12).unwrap();
		let bass_indices = capo_guitar.bass_string_indices();

		assert!(bass_indices.is_some());
		let indices = bass_indices.unwrap();

		// With capo at 12, even the low E becomes E3 which is still below C3? Let's check:
		// E2 = MIDI 40, +12 = MIDI 52 = E3, and C3 = MIDI 48
		// So E3 (52) > C3 (48), meaning it's NOT in bass register anymore
		assert!(
			indices.is_empty() || indices.len() < 2,
			"High capo should reduce or eliminate bass strings"
		);
	}

	// ==================== ConfigurableInstrument Tests ====================

	#[test]
	fn test_configurable_instrument_builder() {
		use crate::note::PitchClass::*;

		let instrument = ConfigurableInstrument::builder()
			.tuning(vec![
				Note::new(E, 2),
				Note::new(A, 2),
				Note::new(D, 3),
				Note::new(G, 3),
			])
			.fret_range(0, 20)
			.max_stretch(4)
			.build()
			.unwrap();

		assert_eq!(instrument.string_count(), 4);
		assert_eq!(instrument.fret_range(), (0, 20));
		assert_eq!(instrument.max_stretch(), 4);
		// Default values
		assert_eq!(instrument.max_fingers(), 4);
		assert_eq!(instrument.open_position_threshold(), 4);
	}

	#[test]
	fn test_configurable_instrument_builder_with_overrides() {
		use crate::note::PitchClass::*;

		let instrument = ConfigurableInstrument::builder()
			.tuning(vec![Note::new(G, 4), Note::new(C, 4)])
			.fret_range(0, 15)
			.max_stretch(5)
			.max_fingers(3)
			.open_position_threshold(5)
			.min_played_strings(1)
			.string_names(vec!["G".to_string(), "C".to_string()])
			.build()
			.unwrap();

		assert_eq!(instrument.max_fingers(), 3);
		assert_eq!(instrument.open_position_threshold(), 5);
		assert_eq!(instrument.min_played_strings(), 1);
		assert_eq!(instrument.string_names(), vec!["G", "C"]);
	}

	#[test]
	fn test_configurable_instrument_builder_missing_tuning() {
		let result = ConfigurableInstrument::builder()
			.fret_range(0, 20)
			.max_stretch(4)
			.build();

		assert!(result.is_err());
	}

	#[test]
	fn test_configurable_instrument_builder_empty_tuning() {
		let result = ConfigurableInstrument::builder()
			.tuning(vec![])
			.fret_range(0, 20)
			.max_stretch(4)
			.build();

		assert!(result.is_err());
	}

	#[test]
	fn test_configurable_instrument_builder_invalid_string_names() {
		use crate::note::PitchClass::*;

		let result = ConfigurableInstrument::builder()
			.tuning(vec![Note::new(E, 2), Note::new(A, 2)])
			.fret_range(0, 20)
			.max_stretch(4)
			.string_names(vec!["E".to_string()]) // Wrong length!
			.build();

		assert!(result.is_err());
	}

	#[test]
	fn test_configurable_instrument_builder_invalid_bass_index() {
		use crate::note::PitchClass::*;

		let result = ConfigurableInstrument::builder()
			.tuning(vec![Note::new(E, 2), Note::new(A, 2)])
			.fret_range(0, 20)
			.max_stretch(4)
			.bass_string_index(5) // Out of bounds!
			.build();

		assert!(result.is_err());
	}

	#[test]
	fn test_bass_preset() {
		let bass = ConfigurableInstrument::bass();

		assert_eq!(bass.string_count(), 4);
		assert_eq!(bass.fret_range(), (0, 24));
		assert_eq!(bass.min_played_strings(), 1);

		// All bass strings should be in bass register
		let bass_indices = bass.bass_string_indices();
		assert!(
			bass_indices.is_none(),
			"Bass guitar should return None (all strings are bass)"
		);
	}

	#[test]
	fn test_mandolin_preset() {
		let mandolin = ConfigurableInstrument::mandolin();

		assert_eq!(mandolin.string_count(), 4);
		assert_eq!(mandolin.fret_range(), (0, 17));
		assert_eq!(mandolin.open_position_threshold(), 5);

		// Mandolin is high-pitched, no bass strings
		let bass_indices = mandolin.bass_string_indices();
		assert!(bass_indices.is_some());
		assert!(
			bass_indices.unwrap().is_empty(),
			"Mandolin should have no bass strings"
		);
	}

	#[test]
	fn test_banjo_preset() {
		let banjo = ConfigurableInstrument::banjo();

		assert_eq!(banjo.string_count(), 5);
		// Bass string is index 1 (D3), not 0 (high G drone)
		assert_eq!(banjo.bass_string_index(), 1);
	}

	#[test]
	fn test_7_string_guitar_preset() {
		let guitar7 = ConfigurableInstrument::guitar_7_string();

		assert_eq!(guitar7.string_count(), 7);

		// Should have 3 bass strings: B1, E2, A2
		let bass_indices = guitar7.bass_string_indices();
		assert!(bass_indices.is_some());
		let indices = bass_indices.unwrap();
		assert_eq!(
			indices.len(),
			3,
			"7-string should have 3 bass strings (B1, E2, A2)"
		);
	}

	#[test]
	fn test_drop_d_preset() {
		let drop_d = ConfigurableInstrument::guitar_drop_d();

		assert_eq!(drop_d.string_count(), 6);
		// First string is D2, which is in bass register
		assert_eq!(drop_d.tuning()[0].pitch, PitchClass::D);
		assert_eq!(drop_d.tuning()[0].octave, 2);
	}

	#[test]
	fn test_configurable_instrument_with_capo() {
		let bass = ConfigurableInstrument::bass();
		let capo_bass = bass.with_capo(5).unwrap();

		// Tuning should be transposed
		assert_eq!(capo_bass.tuning()[0].pitch, PitchClass::A); // E1 + 5 = A1

		// Fret range should be reduced
		assert_eq!(capo_bass.fret_range().1, 24 - 5);
	}
}
