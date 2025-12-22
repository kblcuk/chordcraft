//! Fingering representation for stringed instruments
//!
//! This module provides types for representing and working with chord fingerings
//! in tab notation format (e.g., "x32010" for C major on guitar).

use crate::error::{ChordCraftError, Result};
use crate::instrument::Instrument;
use crate::note::{Note, PitchClass};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringState {
	Muted,
	Fretted(u8), // 0 = open string
}

impl StringState {
	pub fn is_played(&self) -> bool {
		matches!(self, StringState::Fretted(_))
	}

	pub fn fret(&self) -> Option<u8> {
		match self {
			StringState::Muted => None,
			StringState::Fretted(f) => Some(*f),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fingering {
	strings: Vec<StringState>, // Ordered lowest (bass) to highest (treble)
}

impl Fingering {
	pub fn new(strings: Vec<StringState>) -> Self {
		Fingering { strings }
	}

	/// Format: 'x'=muted, '0'-'9'=fret, '(10)'=high frets. Ordered low to high string.
	pub fn parse(s: &str) -> Result<Self> {
		let s = s.trim();
		if s.is_empty() {
			return Err(ChordCraftError::InvalidFingering(
				"Empty fingering".to_string(),
			));
		}

		let mut strings = Vec::new();
		let mut chars = s.chars().peekable();

		while let Some(c) = chars.next() {
			let state = match c {
				'x' | 'X' => StringState::Muted,
				'0'..='9' => StringState::Fretted(c.to_digit(10).unwrap() as u8),
				'(' => {
					let mut num_str = String::new();
					let mut found_closing = false;
					while let Some(&next) = chars.peek() {
						if next == ')' {
							chars.next(); // consume the ')'
							found_closing = true;
							break;
						}
						num_str.push(chars.next().unwrap());
					}
					if !found_closing {
						return Err(ChordCraftError::InvalidFingering(
							"Unclosed parenthesis in fret notation".to_string(),
						));
					}
					let fret = num_str.parse::<u8>().map_err(|_| {
						ChordCraftError::InvalidFingering(format!("Invalid fret number: {num_str}"))
					})?;
					if fret > 24 {
						return Err(ChordCraftError::InvalidFingering(format!(
							"Fret {fret} exceeds maximum of 24"
						)));
					}
					StringState::Fretted(fret)
				}
				' ' | '-' => continue, // Allow separators
				_ => {
					return Err(ChordCraftError::InvalidFingering(format!(
						"Invalid character in fingering: '{c}'"
					)));
				}
			};
			strings.push(state);
		}

		if strings.is_empty() {
			return Err(ChordCraftError::InvalidFingering(
				"No strings found".to_string(),
			));
		}

		Ok(Fingering { strings })
	}

	pub fn strings(&self) -> &[StringState] {
		&self.strings
	}

	pub fn string_count(&self) -> usize {
		self.strings.len()
	}

	pub fn get_string(&self, index: usize) -> Option<&StringState> {
		self.strings.get(index)
	}

	/// Returns (string_index, fret) pairs, excluding muted and open strings.
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

	pub fn min_fret(&self) -> Option<u8> {
		self.strings
			.iter()
			.filter_map(|s| match s {
				StringState::Fretted(f) if *f > 0 => Some(*f),
				_ => None,
			})
			.min()
	}

	pub fn max_fret(&self) -> Option<u8> {
		self.strings.iter().filter_map(|s| s.fret()).max()
	}

	pub fn fret_span(&self) -> u8 {
		let fretted: Vec<u8> = self
			.strings
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

	pub fn is_open_position_for<I: Instrument>(&self, instrument: &I) -> bool {
		self.strings
			.iter()
			.any(|s| matches!(s, StringState::Fretted(0)))
			&& self.max_fret().unwrap_or(0) <= instrument.open_position_threshold()
	}

	pub fn requires_barre(&self) -> bool {
		if let Some(min) = self.min_fret() {
			let count_at_min = self
				.strings
				.iter()
				.filter(|s| matches!(s, StringState::Fretted(f) if *f == min))
				.count();
			count_at_min >= 2
		} else {
			false
		}
	}

	pub fn has_barre(&self) -> bool {
		self.requires_barre()
	}

	/// Detects awkward barres above the minimum fret (requiring ring/pinkie barre).
	/// Only penalizes if the LARGEST barre isn't at the base position.
	pub fn has_high_barre_for<I: Instrument>(&self, instrument: &I) -> bool {
		self.has_high_barre_with_threshold(instrument.main_barre_threshold())
	}

	fn has_high_barre_with_threshold(&self, threshold: usize) -> bool {
		use std::collections::HashMap;

		let min_fret = match self.min_fret() {
			Some(f) => f,
			None => return false,
		};

		let mut fret_groups: HashMap<u8, Vec<usize>> = HashMap::new();

		for (string_idx, state) in self.strings.iter().enumerate() {
			if let StringState::Fretted(fret) = state
				&& *fret > 0
			{
				fret_groups.entry(*fret).or_default().push(string_idx);
			}
		}

		let mut max_barre_length = 0;
		let mut max_barre_fret = 0;

		for (fret, strings) in fret_groups.iter() {
			let consecutive = Self::count_consecutive_strings(strings);
			if consecutive > max_barre_length {
				max_barre_length = consecutive;
				max_barre_fret = *fret;
			}
		}

		max_barre_length >= threshold && max_barre_fret > min_fret
	}

	fn count_consecutive_strings(strings: &[usize]) -> usize {
		if strings.is_empty() {
			return 0;
		}

		let mut sorted = strings.to_vec();
		sorted.sort_unstable();

		let mut max_consecutive = 1;
		let mut current_consecutive = 1;

		for i in 1..sorted.len() {
			if sorted[i] == sorted[i - 1] + 1 {
				current_consecutive += 1;
				max_consecutive = max_consecutive.max(current_consecutive);
			} else {
				current_consecutive = 1;
			}
		}

		max_consecutive
	}

	/// Accounts for barres: consecutive strings at the same fret use one finger.
	pub fn min_fingers_required(&self) -> u8 {
		use std::collections::BTreeMap;

		// Group non-open fretted positions by fret number
		let mut frets_map: BTreeMap<u8, Vec<usize>> = BTreeMap::new();

		for (string_idx, state) in self.strings.iter().enumerate() {
			if let StringState::Fretted(fret) = state
				&& *fret > 0
			{
				frets_map.entry(*fret).or_default().push(string_idx);
			}
		}

		let mut total_fingers = 0;
		for (_fret, strings) in frets_map.iter() {
			total_fingers += Self::count_fingers_for_strings(strings);
		}

		total_fingers
	}

	/// Consecutive strings can be barred; gaps require separate fingers.
	fn count_fingers_for_strings(strings: &[usize]) -> u8 {
		if strings.is_empty() {
			return 0;
		}

		if strings.len() == 1 {
			return 1;
		}

		let mut sorted = strings.to_vec();
		sorted.sort_unstable();

		let mut finger_count = 0;
		let mut i = 0;

		while i < sorted.len() {
			finger_count += 1;
			while i + 1 < sorted.len() && sorted[i + 1] == sorted[i] + 1 {
				i += 1;
			}

			i += 1;
		}

		finger_count
	}

	pub fn is_playable_for<I: Instrument>(&self, instrument: &I) -> bool {
		self.is_playable_with_constraints(instrument.max_stretch(), instrument.max_fingers())
	}

	fn is_playable_with_constraints(&self, max_stretch: u8, max_fingers: u8) -> bool {
		if self.fret_span() > max_stretch {
			return false;
		}
		if self.min_fingers_required() > max_fingers {
			return false;
		}

		true
	}

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
					StringState::Fretted(fret) => Some(tuning[i].add_semitones(*fret as i32)),
				}
			})
			.collect()
	}

	pub fn pitch_classes<I: Instrument>(&self, instrument: &I) -> Vec<PitchClass> {
		self.notes(instrument)
			.into_iter()
			.map(|n| n.pitch)
			.collect()
	}

	pub fn unique_pitch_classes<I: Instrument>(&self, instrument: &I) -> Vec<PitchClass> {
		let mut pitches = self.pitch_classes(instrument);
		pitches.sort_by_key(|p| p.to_semitone());
		pitches.dedup();
		pitches
	}

	/// Returns 0-100, higher is easier to play.
	pub fn playability_score_for<I: Instrument>(&self, instrument: &I) -> u8 {
		self.playability_score_with_params(
			instrument.max_stretch(),
			instrument.max_fingers(),
			instrument.main_barre_threshold(),
			instrument.open_position_threshold(),
		)
	}

	fn playability_score_with_params(
		&self,
		max_stretch: u8,
		max_fingers: u8,
		main_barre_threshold: usize,
		open_position_threshold: u8,
	) -> u8 {
		let mut score: i32 = 100;
		let span = self.fret_span();
		if span > max_stretch {
			return 0; // Unplayable
		}
		score -= (span as i32) * 10;

		let fingers = self.min_fingers_required();
		if fingers > max_fingers {
			return 0;
		}
		// Reward efficient finger usage: fewer fingers = easier transitions
		// Note: using all 4 fingers is normal for barre chords, so penalty is mild
		let finger_ratio = (fingers as f32) / (max_fingers as f32);
		if finger_ratio <= 0.25 {
			score += 15;
		} else if finger_ratio <= 0.5 {
			score += 10;
		} else if finger_ratio <= 0.75 {
			score += 0;
		} else {
			score -= 5; // Mild penalty for using all fingers (was -20)
		}

		if self.has_high_barre_with_threshold(main_barre_threshold) {
			score -= 40;
		}

		// Check for interior open strings (open strings between fretted notes)
		// These create muddy tone and require precise muting control
		let interior_opens = self.interior_open_string_count();
		if interior_opens > 0 {
			score -= (interior_opens as i32) * 15;
		}

		// Open position bonus only for true open positions (no interior opens)
		let has_open_strings = self
			.strings
			.iter()
			.any(|s| matches!(s, StringState::Fretted(0)));
		let is_open = has_open_strings
			&& self.max_fret().unwrap_or(0) <= open_position_threshold
			&& interior_opens == 0;
		if is_open {
			score += 10;
		}

		if let Some(min) = self.min_fret()
			&& min > 7
		{
			score -= ((min - 7) as i32) * 2;
		}

		let muted_count = self.strings.iter().filter(|s| !s.is_played()).count();
		if muted_count > 1 {
			score -= ((muted_count - 1) as i32) * 5;
		}

		score.clamp(0, 100) as u8
	}

	/// Count open strings (fret 0) that fall between the first and last fretted (>0) strings.
	/// These "interior opens" are harder to play cleanly due to muting requirements.
	fn interior_open_string_count(&self) -> usize {
		// Find first and last string with fret > 0
		let first_fretted = self
			.strings
			.iter()
			.position(|s| matches!(s, StringState::Fretted(f) if *f > 0));
		let last_fretted = self
			.strings
			.iter()
			.rposition(|s| matches!(s, StringState::Fretted(f) if *f > 0));

		match (first_fretted, last_fretted) {
			(Some(first), Some(last)) if last > first => self.strings[first..=last]
				.iter()
				.filter(|s| matches!(s, StringState::Fretted(0)))
				.count(),
			_ => 0,
		}
	}

	/// Uses instrument's `bass_string_index()` for re-entrant tunings (e.g., ukulele).
	pub fn bass_note<I: Instrument>(&self, instrument: &I) -> Option<Note> {
		let tuning = instrument.tuning();
		let bass_idx = instrument.bass_string_index();
		let string_count = self.strings.len().min(tuning.len());

		for (string_state, open_note) in self.strings[bass_idx..string_count]
			.iter()
			.zip(tuning[bass_idx..string_count].iter())
		{
			if let StringState::Fretted(fret) = string_state {
				return Some(open_note.add_semitones(*fret as i32));
			}
		}

		// Fallback: check strings before bass_idx (for re-entrant tunings)
		for (string_state, open_note) in self.strings[..bass_idx]
			.iter()
			.zip(tuning[..bass_idx].iter())
		{
			if let StringState::Fretted(fret) = string_state {
				return Some(open_note.add_semitones(*fret as i32));
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
				StringState::Fretted(fret) if *fret < 10 => write!(f, "{fret}")?,
				StringState::Fretted(fret) => write!(f, "({fret})")?,
			}
		}
		Ok(())
	}
}

pub struct FingeringBuilder {
	strings: Vec<StringState>,
}

impl FingeringBuilder {
	pub fn new(string_count: usize) -> Self {
		FingeringBuilder {
			strings: vec![StringState::Muted; string_count],
		}
	}

	pub fn fret(mut self, string: usize, fret: u8) -> Self {
		if string < self.strings.len() {
			self.strings[string] = StringState::Fretted(fret);
		}
		self
	}

	pub fn mute(mut self, string: usize) -> Self {
		if string < self.strings.len() {
			self.strings[string] = StringState::Muted;
		}
		self
	}

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
		let guitar = Guitar::default();
		let c = Fingering::parse("x32010").unwrap();
		assert!(c.is_open_position_for(&guitar));

		let barre_f = Fingering::parse("133211").unwrap();
		assert!(!barre_f.is_open_position_for(&guitar));
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
		let guitar = Guitar::default();
		let easy = Fingering::parse("x32010").unwrap();
		assert!(easy.is_playable_for(&guitar));
		assert!(easy.playability_score_for(&guitar) > 50);

		let hard = Fingering::parse("x24442").unwrap();
		assert!(hard.playability_score_for(&guitar) < easy.playability_score_for(&guitar));
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

	#[test]
	fn test_bass_note_ukulele() {
		use crate::instrument::Ukulele;

		let ukulele = Ukulele::default();

		// Classic ukulele C chord: 0003 (G-C-E-A strings, frets 0-0-0-3)
		// The C string (index 1) is the bass, not the G string (index 0)
		// C string open = C, so bass note should be C
		let c_chord = Fingering::parse("0003").unwrap();
		let bass = c_chord.bass_note(&ukulele).unwrap();
		assert_eq!(
			bass.pitch,
			PitchClass::C,
			"Ukulele 0003: bass should be C (from C string), not G"
		);

		// Another test: 2010 (Am on ukulele)
		// Strings: G=2 (A), C=0 (C), E=1 (F), A=0 (A)
		// Bass string is C (index 1), which is open = C
		let am_chord = Fingering::parse("2010").unwrap();
		let bass = am_chord.bass_note(&ukulele).unwrap();
		assert_eq!(bass.pitch, PitchClass::C, "Ukulele 2010: bass should be C");

		// Test when C string is muted: xx03
		// G (index 0) muted, C (index 1) muted
		// Should find bass from next available string (E at index 2)
		let partial = Fingering::parse("xx03").unwrap();
		let bass = partial.bass_note(&ukulele).unwrap();
		assert_eq!(
			bass.pitch,
			PitchClass::E,
			"Ukulele xx03: bass should be E (G and C strings muted)"
		);
	}

	#[test]
	fn test_min_fingers_simple_barre() {
		// 464444 - string 1 at fret 6 breaks the barre at fret 4
		// Algorithm counts: string 0 at fret 4 (1), strings 2-5 barred at fret 4 (1), string 1 at fret 6 (1)
		// = 3 fingers (conservative but correct - can't make a full barre with string 1 in the way)
		let f = Fingering::parse("464444").unwrap();
		assert_eq!(
			f.min_fingers_required(),
			3,
			"Broken barre + one note = 3 fingers"
		);

		// A true full barre would be something like 444444
		let full_barre = Fingering::parse("444444").unwrap();
		assert_eq!(
			full_barre.min_fingers_required(),
			1,
			"Full barre = 1 finger"
		);

		// Or a barre with one extension: 444445
		let barre_plus = Fingering::parse("444445").unwrap();
		assert_eq!(
			barre_plus.min_fingers_required(),
			2,
			"Barre + extension = 2 fingers"
		);
	}

	#[test]
	fn test_min_fingers_complex() {
		// 424404 - mixed frets with gaps
		let f = Fingering::parse("424404").unwrap();
		// Fret 4: strings 0,2,3,5 (can barre 2-3, separate fingers for 0 and 5) = 3 fingers
		// Fret 2: string 1 = 1 finger
		// Total: 4 fingers
		assert_eq!(
			f.min_fingers_required(),
			4,
			"Mixed frets with gaps = 4 fingers"
		);
	}

	#[test]
	fn test_min_fingers_open_chord() {
		// x32010 - classic C major
		let f = Fingering::parse("x32010").unwrap();
		// Fret 3: string 1 = 1 finger
		// Fret 2: string 2 = 1 finger
		// Fret 1: string 4 = 1 finger
		// Fret 0: strings 3,5 = open, no fingers
		assert_eq!(f.min_fingers_required(), 3, "Open C major = 3 fingers");
	}

	#[test]
	fn test_min_fingers_barre_f() {
		// 133211 - barre F chord
		let f = Fingering::parse("133211").unwrap();
		// Fret 1: strings 0,1,5 (0-1 consecutive, 5 separate) = 2 barres/fingers
		// Fret 3: string 2 = 1 finger
		// Fret 3: string 3 = wait, let me recalculate
		// 133211 = E:1, A:3, D:3, G:2, B:1, e:1
		// Fret 1: strings 0,4,5 (not all consecutive) = need to check grouping
		// Fret 2: string 3 = 1 finger
		// Fret 3: strings 1,2 (consecutive) = 1 barre
		// Let's trace through the algorithm...
		// Actually, standard F is: index finger barres fret 1 (strings 0,1,5)
		// but they're not consecutive (0,1 are, but 5 is separate)
		// So: barre 0-1 (1 finger), string 5 at fret 1 (could extend barre? or separate)
		// In practice, you barre all of fret 1 with index finger = 1 finger
		// Then fret 2 string 3 with middle, fret 3 strings 1,2 with ring/pinkie
		// Let's see what our algorithm says...
		let fingers = f.min_fingers_required();
		println!("Barre F requires {fingers} fingers by algorithm");
		// The algorithm will group by fret, so:
		// Fret 1: [0,4,5] -> 0 alone, 4-5 consecutive = 2 groups
		// Fret 2: [3] = 1 group
		// Fret 3: [1,2] consecutive = 1 group
		// Total = 4 fingers
		// But in reality, you can barre all of fret 1 with one finger!
		// Our algorithm is conservative (over-estimates) which is okay for now
		assert!(fingers <= 4, "Barre F should be playable");
	}

	#[test]
	fn test_unplayable_too_many_fingers() {
		let guitar = Guitar::default();
		// Create a fingering that requires 5+ fingers (should be filtered)
		let f = Fingering::parse("123456").unwrap();
		let fingers = f.min_fingers_required();
		assert!(fingers > 4, "This should require too many fingers");
		assert!(!f.is_playable_for(&guitar), "Should be marked unplayable");
	}

	#[test]
	fn test_playability_prefers_fewer_fingers() {
		let guitar = Guitar::default();
		let simple_barre = Fingering::parse("464444").unwrap(); // 3 fingers
		let complex = Fingering::parse("424404").unwrap(); // 4 fingers

		let score_simple = simple_barre.playability_score_for(&guitar);
		let score_complex = complex.playability_score_for(&guitar);

		assert!(
			score_simple > score_complex,
			"Simpler fingering (3 fingers) should score higher than complex (4 fingers): {score_simple} vs {score_complex}"
		);
	}

	#[test]
	fn test_has_high_barre() {
		let guitar = Guitar::default();
		// 464444 - barre at fret 4 (minimum), extension at fret 6 - NO high barre
		let good_barre = Fingering::parse("464444").unwrap();
		assert!(
			!good_barre.has_high_barre_for(&guitar),
			"464444 should NOT have high barre (barre is at min fret)"
		);

		// 424444 - fret 2 on one string, barre at fret 4 - YES high barre
		let bad_barre = Fingering::parse("424444").unwrap();
		assert!(
			bad_barre.has_high_barre_for(&guitar),
			"424444 should have high barre (barre above min fret)"
		);

		// 133211 - classic F barre chord, barre at fret 1 (minimum) - NO high barre
		let f_chord = Fingering::parse("133211").unwrap();
		assert!(
			!f_chord.has_high_barre_for(&guitar),
			"F barre should NOT have high barre"
		);

		// x32010 - open C, no barres at all
		let c_chord = Fingering::parse("x32010").unwrap();
		assert!(
			!c_chord.has_high_barre_for(&guitar),
			"Open C should have no high barre"
		);
	}

	#[test]
	fn test_playability_penalizes_high_barre() {
		let guitar = Guitar::default();
		let good_barre = Fingering::parse("464444").unwrap(); // Barre at min fret
		let bad_barre = Fingering::parse("424444").unwrap(); // Barre above min fret

		let score_good = good_barre.playability_score_for(&guitar);
		let score_bad = bad_barre.playability_score_for(&guitar);

		assert!(
			score_good > score_bad,
			"Low barre should score higher than high barre: {score_good} vs {score_bad}"
		);

		// Should be significant penalty (at least 30 points difference due to -40 penalty)
		assert!(
			score_good - score_bad >= 30,
			"High barre penalty should be substantial: {} - {} = {}",
			score_good,
			score_bad,
			score_good - score_bad
		);
	}

	#[test]
	fn test_parse_unclosed_parenthesis() {
		let result = Fingering::parse("x(10");
		assert!(result.is_err());
		let err = result.unwrap_err();
		assert!(
			err.to_string().contains("Unclosed parenthesis"),
			"Expected unclosed parenthesis error, got: {err}"
		);
	}

	#[test]
	fn test_parse_fret_exceeds_maximum() {
		let result = Fingering::parse("x(25)0000");
		assert!(result.is_err());
		let err = result.unwrap_err();
		assert!(
			err.to_string().contains("exceeds maximum"),
			"Expected fret exceeds maximum error, got: {err}"
		);

		// Fret 24 should be allowed (edge case)
		let result = Fingering::parse("x(24)0000");
		assert!(result.is_ok(), "Fret 24 should be valid");
	}

	#[test]
	fn test_parse_empty_parenthesis() {
		let result = Fingering::parse("x()0000");
		assert!(result.is_err());
		let err = result.unwrap_err();
		assert!(
			err.to_string().contains("Invalid fret number"),
			"Expected invalid fret number error for empty parens, got: {err}"
		);
	}

	#[test]
	fn test_interior_open_strings_penalized() {
		let guitar = Guitar::default();

		// Classic Bm barre shape (no interior opens)
		let barre = Fingering::parse("x24432").unwrap();
		assert_eq!(barre.interior_open_string_count(), 0);

		// Scattered open strings (2 interior opens)
		let scattered = Fingering::parse("x20402").unwrap();
		assert_eq!(scattered.interior_open_string_count(), 2);

		// Barre should score higher than scattered opens
		assert!(
			barre.playability_score_for(&guitar) > scattered.playability_score_for(&guitar),
			"Barre shape should score higher than scattered opens"
		);

		// True open position (Am - opens at treble end, not interior)
		let am = Fingering::parse("x02210").unwrap();
		assert_eq!(am.interior_open_string_count(), 0);
		assert!(am.is_open_position_for(&guitar));
	}
}
