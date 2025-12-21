//! Note representation and manipulation
//!
//! This module provides types for representing musical notes, including:
//! - Pitch classes (C, C#, D, etc.)
//! - Enharmonic equivalents (C# = Db)
//! - Octave-aware notes
//! - Conversions and calculations

use crate::error::{ChordCraftError, Result};
use std::fmt;

/// A pitch class representing one of the 12 notes in an octave
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PitchClass {
	C,
	CSharp, // C# / Db
	D,
	DSharp, // D# / Eb
	E,
	F,
	FSharp, // F# / Gb
	G,
	GSharp, // G# / Ab
	A,
	ASharp, // A# / Bb
	B,
}

impl PitchClass {
	pub fn to_semitone(&self) -> u8 {
		match self {
			PitchClass::C => 0,
			PitchClass::CSharp => 1,
			PitchClass::D => 2,
			PitchClass::DSharp => 3,
			PitchClass::E => 4,
			PitchClass::F => 5,
			PitchClass::FSharp => 6,
			PitchClass::G => 7,
			PitchClass::GSharp => 8,
			PitchClass::A => 9,
			PitchClass::ASharp => 10,
			PitchClass::B => 11,
		}
	}

	pub fn from_semitone(semitone: u8) -> Self {
		match semitone % 12 {
			0 => PitchClass::C,
			1 => PitchClass::CSharp,
			2 => PitchClass::D,
			3 => PitchClass::DSharp,
			4 => PitchClass::E,
			5 => PitchClass::F,
			6 => PitchClass::FSharp,
			7 => PitchClass::G,
			8 => PitchClass::GSharp,
			9 => PitchClass::A,
			10 => PitchClass::ASharp,
			11 => PitchClass::B,
			_ => unreachable!(),
		}
	}

	/// Parse a pitch class from a string (e.g., "C", "C#", "Db", "Ab")
	pub fn parse(s: &str) -> Result<Self> {
		let s = s.trim();
		match s.to_uppercase().as_str() {
			"C" => Ok(PitchClass::C),
			"C#" | "CS" | "DB" | "D♭" => Ok(PitchClass::CSharp),
			"D" => Ok(PitchClass::D),
			"D#" | "DS" | "EB" | "E♭" => Ok(PitchClass::DSharp),
			"E" => Ok(PitchClass::E),
			"F" => Ok(PitchClass::F),
			"F#" | "FS" | "GB" | "G♭" => Ok(PitchClass::FSharp),
			"G" => Ok(PitchClass::G),
			"G#" | "GS" | "AB" | "A♭" => Ok(PitchClass::GSharp),
			"A" => Ok(PitchClass::A),
			"A#" | "AS" | "BB" | "B♭" => Ok(PitchClass::ASharp),
			"B" => Ok(PitchClass::B),
			_ => Err(ChordCraftError::InvalidNote(s.to_string())),
		}
	}

	/// Get the sharp name (e.g., "C#" instead of "Db")
	pub fn sharp_name(&self) -> &'static str {
		match self {
			PitchClass::C => "C",
			PitchClass::CSharp => "C#",
			PitchClass::D => "D",
			PitchClass::DSharp => "D#",
			PitchClass::E => "E",
			PitchClass::F => "F",
			PitchClass::FSharp => "F#",
			PitchClass::G => "G",
			PitchClass::GSharp => "G#",
			PitchClass::A => "A",
			PitchClass::ASharp => "A#",
			PitchClass::B => "B",
		}
	}

	/// Get the flat name (e.g., "Db" instead of "C#")
	pub fn flat_name(&self) -> &'static str {
		match self {
			PitchClass::C => "C",
			PitchClass::CSharp => "Db",
			PitchClass::D => "D",
			PitchClass::DSharp => "Eb",
			PitchClass::E => "E",
			PitchClass::F => "F",
			PitchClass::FSharp => "Gb",
			PitchClass::G => "G",
			PitchClass::GSharp => "Ab",
			PitchClass::A => "A",
			PitchClass::ASharp => "Bb",
			PitchClass::B => "B",
		}
	}

	/// Wraps around octave boundaries using modular arithmetic.
	pub fn add_semitones(&self, semitones: i32) -> Self {
		let current = self.to_semitone() as i32;
		let new_semitone = (current + semitones).rem_euclid(12) as u8;
		Self::from_semitone(new_semitone)
	}

	/// Always returns ascending distance (0-11), useful for interval calculation.
	pub fn semitone_distance_to(&self, other: &PitchClass) -> u8 {
		let from = self.to_semitone() as i32;
		let to = other.to_semitone() as i32;
		((to - from + 12) % 12) as u8
	}
}

impl fmt::Display for PitchClass {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.sharp_name())
	}
}

/// An octave-aware note with pitch class and octave number
/// Octave 4 is the octave starting with middle C (C4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Note {
	pub pitch: PitchClass,
	pub octave: i8,
}

impl Note {
	pub fn new(pitch: PitchClass, octave: i8) -> Self {
		Note { pitch, octave }
	}

	/// Convert note to MIDI note number (C4 = 60)
	pub fn to_midi(&self) -> u8 {
		((self.octave + 1) * 12 + self.pitch.to_semitone() as i8) as u8
	}

	pub fn from_midi(midi: u8) -> Self {
		let octave = (midi as i8 / 12) - 1;
		let pitch = PitchClass::from_semitone(midi % 12);
		Note::new(pitch, octave)
	}

	/// Parse a note from string (e.g., "C4", "Ab3", "F#5")
	pub fn parse(s: &str) -> Result<Self> {
		let s = s.trim();

		// Find where the octave number starts
		let octave_start = s
			.chars()
			.position(|c| c.is_ascii_digit() || c == '-')
			.ok_or_else(|| ChordCraftError::InvalidNote(s.to_string()))?;

		let pitch_str = &s[..octave_start];
		let octave_str = &s[octave_start..];

		let pitch = PitchClass::parse(pitch_str)?;
		let octave = octave_str
			.parse::<i8>()
			.map_err(|_| ChordCraftError::InvalidNote(s.to_string()))?;

		Ok(Note::new(pitch, octave))
	}

	pub fn add_semitones(&self, semitones: i32) -> Self {
		let midi = self.to_midi() as i32 + semitones;
		Self::from_midi(midi.clamp(0, 127) as u8)
	}

	pub fn semitone_distance_to(&self, other: &Note) -> i32 {
		other.to_midi() as i32 - self.to_midi() as i32
	}

	/// Returns true if this note is in the bass register (below C3, ~131Hz).
	/// Notes below C3 are typically covered by bass guitar/piano left hand in a band context.
	/// C3 has MIDI note number 48.
	pub fn is_bass_register(&self) -> bool {
		self.to_midi() < 48 // C3 = MIDI 48
	}
}

impl fmt::Display for Note {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.pitch, self.octave)
	}
}

// Convenience constants for common pitch classes
pub const C: PitchClass = PitchClass::C;
pub const C_SHARP: PitchClass = PitchClass::CSharp;
pub const D: PitchClass = PitchClass::D;
pub const D_SHARP: PitchClass = PitchClass::DSharp;
pub const E: PitchClass = PitchClass::E;
pub const F: PitchClass = PitchClass::F;
pub const F_SHARP: PitchClass = PitchClass::FSharp;
pub const G: PitchClass = PitchClass::G;
pub const G_SHARP: PitchClass = PitchClass::GSharp;
pub const A: PitchClass = PitchClass::A;
pub const A_SHARP: PitchClass = PitchClass::ASharp;
pub const B: PitchClass = PitchClass::B;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_pitch_class_semitones() {
		assert_eq!(PitchClass::C.to_semitone(), 0);
		assert_eq!(PitchClass::CSharp.to_semitone(), 1);
		assert_eq!(PitchClass::G.to_semitone(), 7);
		assert_eq!(PitchClass::B.to_semitone(), 11);
	}

	#[test]
	fn test_pitch_class_from_semitone() {
		assert_eq!(PitchClass::from_semitone(0), PitchClass::C);
		assert_eq!(PitchClass::from_semitone(7), PitchClass::G);
		assert_eq!(PitchClass::from_semitone(12), PitchClass::C);
	}

	#[test]
	fn test_pitch_class_parse() {
		assert_eq!(PitchClass::parse("C").unwrap(), PitchClass::C);
		assert_eq!(PitchClass::parse("C#").unwrap(), PitchClass::CSharp);
		assert_eq!(PitchClass::parse("Db").unwrap(), PitchClass::CSharp);
		assert_eq!(PitchClass::parse("Ab").unwrap(), PitchClass::GSharp);
	}

	#[test]
	fn test_pitch_class_add_semitones() {
		assert_eq!(PitchClass::C.add_semitones(7), PitchClass::G);
		assert_eq!(PitchClass::C.add_semitones(12), PitchClass::C);
		assert_eq!(PitchClass::B.add_semitones(1), PitchClass::C);
	}

	#[test]
	fn test_note_midi() {
		let c4 = Note::new(PitchClass::C, 4);
		assert_eq!(c4.to_midi(), 60);
		assert_eq!(Note::from_midi(60), c4);
	}

	#[test]
	fn test_note_parse() {
		let note = Note::parse("C4").unwrap();
		assert_eq!(note.pitch, PitchClass::C);
		assert_eq!(note.octave, 4);

		let note2 = Note::parse("Ab3").unwrap();
		assert_eq!(note2.pitch, PitchClass::GSharp);
		assert_eq!(note2.octave, 3);
	}

	#[test]
	fn test_note_add_semitones() {
		let c4 = Note::new(PitchClass::C, 4);
		let g4 = c4.add_semitones(7);
		assert_eq!(g4.pitch, PitchClass::G);
		assert_eq!(g4.octave, 4);

		let c5 = c4.add_semitones(12);
		assert_eq!(c5.octave, 5);
	}

	#[test]
	fn test_is_bass_register() {
		// C3 is the threshold (MIDI 48) - notes below are bass
		let c3 = Note::new(PitchClass::C, 3);
		assert!(
			!c3.is_bass_register(),
			"C3 should NOT be bass (it's the threshold)"
		);

		let b2 = Note::new(PitchClass::B, 2);
		assert!(b2.is_bass_register(), "B2 should be bass");

		// Guitar low E and A strings
		let e2 = Note::new(PitchClass::E, 2);
		let a2 = Note::new(PitchClass::A, 2);
		assert!(e2.is_bass_register(), "E2 (guitar low E) should be bass");
		assert!(a2.is_bass_register(), "A2 (guitar A string) should be bass");

		// Guitar D string and above
		let d3 = Note::new(PitchClass::D, 3);
		assert!(!d3.is_bass_register(), "D3 should NOT be bass");

		// Ukulele strings (all in octave 4)
		let c4 = Note::new(PitchClass::C, 4);
		let g4 = Note::new(PitchClass::G, 4);
		assert!(!c4.is_bass_register(), "C4 (ukulele) should NOT be bass");
		assert!(!g4.is_bass_register(), "G4 (ukulele) should NOT be bass");

		// Bass guitar strings
		let e1 = Note::new(PitchClass::E, 1);
		let g2 = Note::new(PitchClass::G, 2);
		assert!(e1.is_bass_register(), "E1 (bass guitar) should be bass");
		assert!(g2.is_bass_register(), "G2 (bass guitar) should be bass");
	}
}
