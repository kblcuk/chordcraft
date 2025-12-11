//! Musical interval representation and manipulation
//!
//! This module provides types for representing intervals between notes,
//! including quality (perfect, major, minor, augmented, diminished) and
//! distance.

use crate::error::{ChordCraftError, Result};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntervalQuality {
	Perfect,
	Major,
	Minor,
	Augmented,
	Diminished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval {
	pub quality: IntervalQuality,
	pub distance: u8, // 1-based: 1=unison, 2=second, 3=third, etc.
}

impl Interval {
	pub fn new(quality: IntervalQuality, distance: u8) -> Self {
		Interval { quality, distance }
	}

	pub fn to_semitones(&self) -> u8 {
		use IntervalQuality::*;

		// Base semitones for perfect/major intervals
		let base_semitones = match self.distance {
			1 => 0,  // Unison
			2 => 2,  // Second
			3 => 4,  // Third
			4 => 5,  // Fourth
			5 => 7,  // Fifth
			6 => 9,  // Sixth
			7 => 11, // Seventh
			8 => 12, // Octave
			_ => {
				// For intervals larger than an octave
				let octaves = (self.distance - 1) / 7;
				let remainder = (self.distance - 1) % 7 + 1;
				return octaves * 12 + Interval::new(self.quality, remainder).to_semitones();
			}
		};

		// Adjust for quality
		match (self.quality, self.is_perfect_interval()) {
			(Perfect, true) | (Major, false) => base_semitones,
			(Minor, false) => base_semitones - 1,
			(Augmented, _) => base_semitones + 1,
			(Diminished, _) => base_semitones - 1,
			_ => base_semitones, // Invalid combination, return base
		}
	}

	/// Defaults to major/perfect intervals; ambiguous intervals like tritone become aug 4th.
	pub fn from_semitones(semitones: u8) -> Self {
		use IntervalQuality::*;

		match semitones % 12 {
			0 => Interval::new(Perfect, 1),   // Unison
			1 => Interval::new(Minor, 2),     // Minor 2nd
			2 => Interval::new(Major, 2),     // Major 2nd
			3 => Interval::new(Minor, 3),     // Minor 3rd
			4 => Interval::new(Major, 3),     // Major 3rd
			5 => Interval::new(Perfect, 4),   // Perfect 4th
			6 => Interval::new(Augmented, 4), // Augmented 4th (Tritone)
			7 => Interval::new(Perfect, 5),   // Perfect 5th
			8 => Interval::new(Minor, 6),     // Minor 6th
			9 => Interval::new(Major, 6),     // Major 6th
			10 => Interval::new(Minor, 7),    // Minor 7th
			11 => Interval::new(Major, 7),    // Major 7th
			_ => unreachable!(),
		}
	}

	fn is_perfect_interval(&self) -> bool {
		// Normalize to 1-7 range, then check if it's 1, 4, or 5
		let normalized = (self.distance - 1) % 7 + 1;
		matches!(normalized, 1 | 4 | 5)
	}

	/// Get the short name of this interval (e.g., "M3", "P5", "m7")
	pub fn short_name(&self) -> String {
		let quality_char = match self.quality {
			IntervalQuality::Perfect => "P",
			IntervalQuality::Major => "M",
			IntervalQuality::Minor => "m",
			IntervalQuality::Augmented => "A",
			IntervalQuality::Diminished => "d",
		};
		format!("{}{}", quality_char, self.distance)
	}

	/// Get the full name of this interval (e.g., "Major 3rd", "Perfect 5th")
	pub fn full_name(&self) -> String {
		let quality_name = match self.quality {
			IntervalQuality::Perfect => "Perfect",
			IntervalQuality::Major => "Major",
			IntervalQuality::Minor => "Minor",
			IntervalQuality::Augmented => "Augmented",
			IntervalQuality::Diminished => "Diminished",
		};

		let distance_name = match self.distance {
			1 => "Unison",
			2 => "2nd",
			3 => "3rd",
			4 => "4th",
			5 => "5th",
			6 => "6th",
			7 => "7th",
			8 => "Octave",
			9 => "9th",
			11 => "11th",
			13 => "13th",
			_ => return format!("{} {}", quality_name, self.distance),
		};

		format!("{quality_name} {distance_name}")
	}

	/// Parse an interval from short notation (e.g., "M3", "P5", "m7")
	pub fn parse(s: &str) -> Result<Self> {
		let s = s.trim();
		if s.is_empty() {
			return Err(ChordCraftError::InvalidInterval(s.to_string()));
		}

		// First char is quality
		let quality = match s.chars().next().unwrap() {
			'P' | 'p' => IntervalQuality::Perfect,
			'M' => IntervalQuality::Major,
			'm' => IntervalQuality::Minor,
			'A' | 'a' => IntervalQuality::Augmented,
			'd' | 'D' => IntervalQuality::Diminished,
			_ => return Err(ChordCraftError::InvalidInterval(s.to_string())),
		};

		// Rest is distance
		let distance_str = &s[1..];
		let distance = distance_str
			.parse::<u8>()
			.map_err(|_| ChordCraftError::InvalidInterval(s.to_string()))?;

		if distance == 0 {
			return Err(ChordCraftError::InvalidInterval(s.to_string()));
		}

		Ok(Interval::new(quality, distance))
	}
}

impl fmt::Display for Interval {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.short_name())
	}
}

// Convenience constants for common intervals
pub const UNISON: Interval = Interval {
	quality: IntervalQuality::Perfect,
	distance: 1,
};
pub const MINOR_SECOND: Interval = Interval {
	quality: IntervalQuality::Minor,
	distance: 2,
};
pub const MAJOR_SECOND: Interval = Interval {
	quality: IntervalQuality::Major,
	distance: 2,
};
pub const MINOR_THIRD: Interval = Interval {
	quality: IntervalQuality::Minor,
	distance: 3,
};
pub const MAJOR_THIRD: Interval = Interval {
	quality: IntervalQuality::Major,
	distance: 3,
};
pub const PERFECT_FOURTH: Interval = Interval {
	quality: IntervalQuality::Perfect,
	distance: 4,
};
pub const TRITONE: Interval = Interval {
	quality: IntervalQuality::Augmented,
	distance: 4,
};
pub const PERFECT_FIFTH: Interval = Interval {
	quality: IntervalQuality::Perfect,
	distance: 5,
};
pub const MINOR_SIXTH: Interval = Interval {
	quality: IntervalQuality::Minor,
	distance: 6,
};
pub const MAJOR_SIXTH: Interval = Interval {
	quality: IntervalQuality::Major,
	distance: 6,
};
pub const MINOR_SEVENTH: Interval = Interval {
	quality: IntervalQuality::Minor,
	distance: 7,
};
pub const MAJOR_SEVENTH: Interval = Interval {
	quality: IntervalQuality::Major,
	distance: 7,
};
pub const OCTAVE: Interval = Interval {
	quality: IntervalQuality::Perfect,
	distance: 8,
};

// Extended intervals
pub const MINOR_NINTH: Interval = Interval {
	quality: IntervalQuality::Minor,
	distance: 9,
};
pub const MAJOR_NINTH: Interval = Interval {
	quality: IntervalQuality::Major,
	distance: 9,
};
pub const PERFECT_ELEVENTH: Interval = Interval {
	quality: IntervalQuality::Perfect,
	distance: 11,
};
pub const MAJOR_THIRTEENTH: Interval = Interval {
	quality: IntervalQuality::Major,
	distance: 13,
};

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_interval_to_semitones() {
		assert_eq!(UNISON.to_semitones(), 0);
		assert_eq!(MAJOR_SECOND.to_semitones(), 2);
		assert_eq!(MAJOR_THIRD.to_semitones(), 4);
		assert_eq!(MINOR_THIRD.to_semitones(), 3);
		assert_eq!(PERFECT_FOURTH.to_semitones(), 5);
		assert_eq!(PERFECT_FIFTH.to_semitones(), 7);
		assert_eq!(MINOR_SEVENTH.to_semitones(), 10);
		assert_eq!(MAJOR_SEVENTH.to_semitones(), 11);
		assert_eq!(OCTAVE.to_semitones(), 12);
	}

	#[test]
	fn test_interval_from_semitones() {
		assert_eq!(Interval::from_semitones(0), UNISON);
		assert_eq!(Interval::from_semitones(4), MAJOR_THIRD);
		assert_eq!(Interval::from_semitones(7), PERFECT_FIFTH);
		assert_eq!(Interval::from_semitones(10), MINOR_SEVENTH);
	}

	#[test]
	fn test_interval_parse() {
		assert_eq!(Interval::parse("M3").unwrap(), MAJOR_THIRD);
		assert_eq!(Interval::parse("P5").unwrap(), PERFECT_FIFTH);
		assert_eq!(Interval::parse("m7").unwrap(), MINOR_SEVENTH);
		assert_eq!(Interval::parse("M9").unwrap(), MAJOR_NINTH);
	}

	#[test]
	fn test_interval_short_name() {
		assert_eq!(MAJOR_THIRD.short_name(), "M3");
		assert_eq!(PERFECT_FIFTH.short_name(), "P5");
		assert_eq!(MINOR_SEVENTH.short_name(), "m7");
	}

	#[test]
	fn test_interval_full_name() {
		assert_eq!(MAJOR_THIRD.full_name(), "Major 3rd");
		assert_eq!(PERFECT_FIFTH.full_name(), "Perfect 5th");
		assert_eq!(MINOR_SEVENTH.full_name(), "Minor 7th");
	}
}
