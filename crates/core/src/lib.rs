//! ChordCraft Core - Music theory library for chord-fingering conversion
//!
//! This crate provides the fundamental music theory types and algorithms for:
//! - Note and interval representation
//! - Chord construction and analysis
//! - Instrument modeling (guitar, bass, etc.)
//! - Fingering generation (chord → tabs)
//! - Chord identification (tabs → chord)
//!
//! # Examples
//!
//! ```
//! use chordcraft_core::note::{Note, PitchClass};
//! use chordcraft_core::chord::Chord;
//!
//! // Create a note
//! let c4 = Note::new(PitchClass::C, 4);
//!
//! // Parse a chord
//! let chord = Chord::parse("Cmaj7").unwrap();
//! ```

pub mod analyzer;
pub mod chord;
pub mod fingering;
pub mod generator;
pub mod instrument;
pub mod interval;
pub mod note;
pub mod progression;

// Re-export commonly used types
pub use analyzer::{ChordMatch, analyze_fingering};
pub use chord::{Chord, ChordQuality};
pub use fingering::Fingering;
pub use generator::PlayingContext;
pub use instrument::{CapoedInstrument, ConfigurableInstrument, Guitar, Instrument, Ukulele};
pub use interval::Interval;
pub use note::{Note, PitchClass};

/// Error types for the chordcraft-core library
pub mod error {
	use thiserror::Error;

	#[derive(Error, Debug)]
	pub enum ChordCraftError {
		#[error("Invalid chord name: {0}")]
		InvalidChordName(String),

		#[error("Invalid note name: {0}")]
		InvalidNote(String),

		#[error("Invalid interval: {0}")]
		InvalidInterval(String),

		#[error("Invalid fingering: {0}")]
		InvalidFingering(String),

		#[error("No fingerings found for chord: {0}")]
		NoFingeringsFound(String),

		#[error("Could not identify chord from fingering")]
		ChordNotIdentified,

		#[error("Invalid capo position: {0} (must be between {1} and {2})")]
		InvalidCapoPosition(u8, u8, u8),

		#[error("Invalid instrument configuration: {0}")]
		InvalidInstrument(String),
	}

	pub type Result<T> = std::result::Result<T, ChordCraftError>;
}

pub use error::{ChordCraftError, Result};
