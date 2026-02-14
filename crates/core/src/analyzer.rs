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
		.filter(|req| intervals.iter().any(|i| i.enharmonic_eq(req)))
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
		.filter(|opt| intervals.iter().any(|i| i.enharmonic_eq(opt)))
		.count();
	score += (optional_count * 5) as u32;

	let all_chord_intervals: Vec<_> = required.iter().chain(optional.iter()).collect();
	let extra_count = intervals
		.iter()
		.filter(|interval| {
			!all_chord_intervals
				.iter()
				.any(|ci| ci.enharmonic_eq(interval))
		})
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

	#[test]
	fn test_analyze_b_diminished() {
		let guitar = Guitar::default();
		// Bdim = B, D, F (x2x210 is B-D-F on guitar: A string fret 2 = B, G string fret 2 = A... let me use a known Bdim voicing)
		// x20101: A=B, D=open=D, G=1=G#... that's not right
		// Better: Bdim chord notes are B, D, F
		// xx0101 won't work. Let's use a fingering that produces B, D, F
		// E string: fret 7 = B; A string: fret 5 = D (wait, A+5 = D); D string: fret 3 = F
		// So: x53xxx won't include all strings...
		// Standard Bdim voicing: x2343x (A=B, D=D, G=F, B=D#... no)
		// Let me just construct one manually using known note positions
		// On guitar: B is on A string fret 2, D is on D string fret 0 (open), F is on e string fret 1
		// So x2x01x won't parse easily. Let me use Fingering::parse with a known shape
		// Bdim7 = x20101: A string fret 2 = B, D string fret 0 = D, G string fret 1 = G#, B string fret 0 = B, e string fret 1 = F
		// That gives B, D, G#, B, F -- which is Bdim7 (B, D, F, Ab)
		// Let's try Bdim (no 7th): just need B, D, F
		// Use xx0110: D string fret 0 = D, G string fret 1 = G#... no
		// Actually let me just use x2x010 for B, D, F: A fret 2 = B, G fret 0 = G (wrong)
		// Simplest approach: construct the fingering to give specific notes
		// Guitar tuning: E2 A2 D3 G3 B3 E4
		// B at A fret 2, D at D fret 0, F at E(high) fret 1 -> x20xx1
		let fingering = Fingering::parse("x20xx1").unwrap();
		let matches = analyze_fingering(&fingering, &guitar);

		assert!(
			!matches.is_empty(),
			"Should find matches for diminished chord notes"
		);

		// Should find Bdim somewhere in matches
		let has_bdim = matches.iter().any(|m| {
			m.chord.root == PitchClass::B
				&& matches!(
					m.chord.quality,
					ChordQuality::Diminished
						| ChordQuality::HalfDiminished7
						| ChordQuality::Diminished7
				)
		});
		assert!(
			has_bdim,
			"Should identify B diminished-family chord from B, D, F notes. Got: {:?}",
			matches
				.iter()
				.map(|m| format!("{}", m.chord))
				.collect::<Vec<_>>()
		);
	}

	#[test]
	fn test_analyze_tritone_interval() {
		// Verify the core bug is fixed: Augmented(4) and Diminished(5) are both 6 semitones
		use crate::interval::Interval;

		let aug4 = Interval::from_semitones(6); // Returns Augmented(4)
		let dim5 = Interval::new(crate::interval::IntervalQuality::Diminished, 5);

		// These are different by PartialEq (expected)
		assert_ne!(aug4, dim5, "Aug4 and Dim5 should differ by PartialEq");
		// But equal by semitone (the fix)
		assert!(
			aug4.enharmonic_eq(&dim5),
			"Aug4 and Dim5 should be enharmonically equal (both 6 semitones)"
		);
	}

	#[test]
	fn test_analyze_half_diminished() {
		let guitar = Guitar::default();
		// Bm7b5 (B half-diminished): B, D, F, A
		// x2020x: A fret 2 = B, D fret 0 = D, G fret 2 = A, B fret 0 = B... need F
		// Let's use x20101: A fret 2 = B, D fret 0 = D, G fret 1 = Ab, B fret 0 = B, e fret 1 = F
		// That's B, D, Ab, B, F = Bdim7 (B, D, F, Ab)
		// For half-dim we need B, D, F, A
		// x20210: A=B, D=D, G=A (fret 2), B=B (hmm, B open = B), e=fret 0... wait
		// G fret 2 = A, yes. B string open = B. High e open = E.
		// x2021x: B, D, A, B... no F
		// Try: x20201: A=B, D=D, G=A (fret 2=A), B=open=B, e=fret 1=F
		// Pitches: B, D, A, B, F -> unique: A, B, D, F -> that's Bm7b5!
		let fingering = Fingering::parse("x20201").unwrap();
		let matches = analyze_fingering(&fingering, &guitar);

		assert!(
			!matches.is_empty(),
			"Should find matches for half-diminished chord"
		);

		let has_half_dim = matches.iter().any(|m| {
			m.chord.root == PitchClass::B && m.chord.quality == ChordQuality::HalfDiminished7
		});
		assert!(
			has_half_dim,
			"Should identify Bm7b5 (half-diminished). Got: {:?}",
			matches
				.iter()
				.take(5)
				.map(|m| format!("{} (score: {})", m.chord, m.score))
				.collect::<Vec<_>>()
		);
	}
}
