//! Chord analysis algorithm (Fingering → Chord)
//!
//! This module contains the algorithm for identifying chords from
//! fingering patterns (reverse lookup).

use crate::chord::{Chord, ChordQuality};
use crate::fingering::Fingering;
use crate::instrument::Instrument;
use crate::interval::Interval;
use crate::note::PitchClass;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct ChordMatch {
	pub chord: Chord,
	pub score: u32,
	pub root_in_bass: bool,
	pub completeness: f32,
}

pub fn analyze_fingering<I: Instrument>(fingering: &Fingering, instrument: &I) -> Vec<ChordMatch> {
	let pitches = fingering.unique_pitch_classes(instrument);

	if pitches.is_empty() {
		return vec![];
	}

	let bass_note = fingering.bass_note(instrument).map(|n| n.pitch);

	let mut matches = Vec::new();

	for root in &pitches {
		let intervals = calculate_intervals_from_root(*root, &pitches);

		for quality in ChordQuality::iter() {
			if let Some(chord_match) = try_match_chord(*root, quality, &intervals, bass_note) {
				matches.push(chord_match);
			}
		}
	}

	matches.sort_by(|a, b| b.score.cmp(&a.score));
	deduplicate_matches(matches)
}

fn calculate_intervals_from_root(root: PitchClass, pitches: &[PitchClass]) -> Vec<Interval> {
	pitches
		.iter()
		.map(|pitch| {
			let semitones = root.semitone_distance_to(pitch);
			Interval::from_semitones(semitones)
		})
		.collect()
}

fn try_match_chord(
	root: PitchClass,
	quality: ChordQuality,
	intervals: &[Interval],
	bass_note: Option<PitchClass>,
) -> Option<ChordMatch> {
	let (required, optional) = quality.intervals();

	let required_present: Vec<_> = required
		.iter()
		.filter(|req| intervals.contains(req))
		.collect();

	if required_present.len() < 2 {
		return None;
	}

	let completeness = required_present.len() as f32 / required.len() as f32;
	let chord = Chord::new(root, quality);
	let root_in_bass = bass_note == Some(root);

	let mut score = 0u32;
	score += (completeness * 100.0) as u32;

	if root_in_bass {
		score += 20;
	}

	let optional_count = optional
		.iter()
		.filter(|opt| intervals.contains(opt))
		.count();
	score += (optional_count * 5) as u32;

	let all_chord_intervals: Vec<_> = required.iter().chain(optional.iter()).collect();
	let extra_count = intervals
		.iter()
		.filter(|interval| !all_chord_intervals.contains(interval))
		.count();
	score = score.saturating_sub((extra_count * 10) as u32);

	// Prefer more specific chords (G7 over G when 7th is present)
	score += (required.len() * 3) as u32;

	if completeness >= 1.0 {
		match quality {
			ChordQuality::Major | ChordQuality::Minor => score += 5,
			_ => {}
		}
	}

	Some(ChordMatch {
		chord,
		score,
		root_in_bass,
		completeness,
	})
}

fn deduplicate_matches(mut matches: Vec<ChordMatch>) -> Vec<ChordMatch> {
	let mut unique = Vec::new();

	for m in matches.drain(..) {
		let is_duplicate = unique.iter().any(|existing: &ChordMatch| {
			existing.chord.root == m.chord.root && existing.chord.quality == m.chord.quality
		});

		if !is_duplicate {
			unique.push(m);
		}
	}

	unique
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::instrument::Guitar;

	#[test]
	fn test_analyze_c_major() {
		let guitar = Guitar::default();
		let fingering = Fingering::parse("x32010").unwrap();

		let matches = analyze_fingering(&fingering, &guitar);

		assert!(!matches.is_empty(), "Should find at least one match");

		// First match should be C major
		let first = &matches[0];
		assert_eq!(first.chord.root, PitchClass::C);
		assert_eq!(first.chord.quality, ChordQuality::Major);
		assert!(first.root_in_bass);
	}

	#[test]
	fn test_analyze_am() {
		let guitar = Guitar::default();
		let fingering = Fingering::parse("x02210").unwrap();

		let matches = analyze_fingering(&fingering, &guitar);

		assert!(!matches.is_empty());

		// Should identify as A minor
		let first = &matches[0];
		assert_eq!(first.chord.root, PitchClass::A);
		assert_eq!(first.chord.quality, ChordQuality::Minor);
	}

	#[test]
	fn test_analyze_g7() {
		let guitar = Guitar::default();
		let fingering = Fingering::parse("320001").unwrap();

		let matches = analyze_fingering(&fingering, &guitar);

		assert!(!matches.is_empty());

		// Should identify as G7
		let first = &matches[0];
		assert_eq!(first.chord.root, PitchClass::G);
		assert_eq!(first.chord.quality, ChordQuality::Dominant7);
	}

	#[test]
	fn test_analyze_empty_fingering() {
		let guitar = Guitar::default();
		let fingering = Fingering::parse("xxxxxx").unwrap();

		let matches = analyze_fingering(&fingering, &guitar);

		assert!(matches.is_empty(), "No notes means no chord");
	}
}
