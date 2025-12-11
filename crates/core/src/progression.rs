//! Chord progression optimization
//!
//! This module provides algorithms for finding optimal fingering sequences
//! for chord progressions, minimizing finger movement and maximizing smooth transitions.

use crate::chord::Chord;
use crate::fingering::Fingering;
use crate::generator::{GeneratorOptions, PlayingContext, ScoredFingering, generate_fingerings};
use crate::instrument::Instrument;

const BASE_SCORE: i32 = 100;
const MOVEMENT_WEIGHT: i32 = 30;
const ANCHOR_BONUS: i32 = 20;
const BARRE_SIMILARITY_BONUS: i32 = 15;
const OPEN_POSITION_BONUS: i32 = 10;
const STRING_COUNT_SIMILARITY_BONUS: i32 = 5;
const DISTANCE_PENALTY: i32 = 5;
const BAND_MOVEMENT_WEIGHT: i32 = 40;
const BAND_DISTANCE_PENALTY: i32 = 8;

#[derive(Debug, Clone)]
pub struct ProgressionOptions {
	pub limit: usize,
	pub max_fret_distance: u8,
	pub candidates_per_chord: usize,
	pub generator_options: GeneratorOptions,
}

impl Default for ProgressionOptions {
	fn default() -> Self {
		ProgressionOptions {
			limit: 3,
			max_fret_distance: 3,
			candidates_per_chord: 20,
			generator_options: GeneratorOptions::default(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct ChordTransition {
	pub from_chord: String,
	pub to_chord: String,
	pub from_fingering: ScoredFingering,
	pub to_fingering: ScoredFingering,
	pub score: i32,
	pub finger_movements: usize,
	pub common_anchors: usize,
	pub position_distance: u8,
}

#[derive(Debug, Clone)]
pub struct ProgressionSequence {
	pub chords: Vec<String>,
	pub fingerings: Vec<ScoredFingering>,
	pub transitions: Vec<ChordTransition>,
	pub total_score: i32,
	pub avg_transition_score: f32,
}

/// Generate optimized fingering progressions for a sequence of chords
///
/// # Examples
///
/// ```
/// use chordcraft_core::progression::{generate_progression, ProgressionOptions};
/// use chordcraft_core::instrument::Guitar;
///
/// let guitar = Guitar::default();
/// let chords = vec!["C", "Am", "F", "G"];
/// let options = ProgressionOptions::default();
///
/// let progressions = generate_progression(&chords, &guitar, &options);
/// assert!(!progressions.is_empty());
/// ```
pub fn generate_progression<I: Instrument>(
	chord_names: &[&str],
	instrument: &I,
	options: &ProgressionOptions,
) -> Vec<ProgressionSequence> {
	let chords: Vec<Chord> = chord_names
		.iter()
		.filter_map(|name| Chord::parse(name).ok())
		.collect();

	if chords.is_empty() {
		return vec![];
	}

	let mut candidates: Vec<Vec<ScoredFingering>> = Vec::new();
	for chord in &chords {
		let mut opts = options.generator_options.clone();
		opts.limit = options.candidates_per_chord;
		let fingerings = generate_fingerings(chord, instrument, &opts);
		candidates.push(fingerings);
	}

	if candidates.iter().any(|c| c.is_empty()) {
		return vec![];
	}

	let mut sequences = Vec::new();
	let start_limit = options.limit.min(candidates[0].len());

	for start_idx in 0..start_limit {
		if let Some(sequence) = build_progression_sequence(
			&chords,
			chord_names,
			&candidates,
			start_idx,
			instrument,
			options,
		) {
			sequences.push(sequence);
		}
	}

	sequences.sort_by(|a, b| b.total_score.cmp(&a.total_score));
	sequences.truncate(options.limit);
	sequences
}

fn build_progression_sequence<I: Instrument>(
	chords: &[Chord],
	chord_names: &[&str],
	candidates: &[Vec<ScoredFingering>],
	start_idx: usize,
	instrument: &I,
	options: &ProgressionOptions,
) -> Option<ProgressionSequence> {
	let mut selected_fingerings = Vec::new();
	let mut transitions = Vec::new();

	selected_fingerings.push(candidates[0][start_idx].clone());

	for i in 1..chords.len() {
		let from = &selected_fingerings[i - 1];
		let from_chord_name = chord_names[i - 1].to_string();
		let to_chord_name = chord_names[i].to_string();
		let mut best_transition: Option<(ChordTransition, ScoredFingering)> = None;

		for to in &candidates[i] {
			let transition = score_transition(
				from_chord_name.clone(),
				to_chord_name.clone(),
				from,
				to,
				instrument,
				options.generator_options.playing_context,
			);

			if transition.position_distance > options.max_fret_distance {
				continue;
			}

			if best_transition.is_none()
				|| transition.score > best_transition.as_ref().unwrap().0.score
			{
				best_transition = Some((transition, to.clone()));
			}
		}

		let (transition, to_fingering) = best_transition?;

		transitions.push(transition);
		selected_fingerings.push(to_fingering);
	}

	let total_score: i32 = transitions.iter().map(|t| t.score).sum();
	let avg_transition_score = if transitions.is_empty() {
		0.0
	} else {
		total_score as f32 / transitions.len() as f32
	};

	Some(ProgressionSequence {
		chords: chord_names.iter().map(|s| s.to_string()).collect(),
		fingerings: selected_fingerings,
		transitions,
		total_score,
		avg_transition_score,
	})
}

fn score_transition<I: Instrument>(
	from_chord: String,
	to_chord: String,
	from_scored: &ScoredFingering,
	to_scored: &ScoredFingering,
	instrument: &I,
	playing_context: PlayingContext,
) -> ChordTransition {
	let from = &from_scored.fingering;
	let to = &to_scored.fingering;
	let from_pos = from_scored.position;
	let to_pos = to_scored.position;

	let mut score = BASE_SCORE;

	let (movement_weight, distance_penalty) = match playing_context {
		PlayingContext::Solo => (MOVEMENT_WEIGHT, DISTANCE_PENALTY),
		PlayingContext::Band => (BAND_MOVEMENT_WEIGHT, BAND_DISTANCE_PENALTY),
	};

	let (movements, anchors) = calculate_finger_changes(from, to);
	score += (4_i32.saturating_sub(movements as i32)) * movement_weight;
	score += (anchors as i32) * ANCHOR_BONUS;

	let shape_bonus = calculate_shape_similarity(from, to, instrument);
	score += shape_bonus;

	let distance = (to_pos as i32 - from_pos as i32).unsigned_abs() as u8;
	score -= (distance as i32) * distance_penalty;

	ChordTransition {
		from_chord,
		to_chord,
		from_fingering: from_scored.clone(),
		to_fingering: to_scored.clone(),
		score,
		finger_movements: movements,
		common_anchors: anchors,
		position_distance: distance,
	}
}

fn calculate_finger_changes(from: &Fingering, to: &Fingering) -> (usize, usize) {
	let from_strings = from.strings();
	let to_strings = to.strings();

	let string_count = from_strings.len().min(to_strings.len());

	let mut movements = 0;
	let mut anchors = 0;

	for i in 0..string_count {
		let from_state = &from_strings[i];
		let to_state = &to_strings[i];

		match (from_state, to_state) {
			(
				crate::fingering::StringState::Fretted(f1),
				crate::fingering::StringState::Fretted(f2),
			) => {
				if f1 == f2 {
					anchors += 1;
				} else {
					movements += 1;
				}
			}
			(crate::fingering::StringState::Fretted(_), crate::fingering::StringState::Muted) => {
				movements += 1;
			}
			(crate::fingering::StringState::Muted, crate::fingering::StringState::Fretted(_)) => {
				movements += 1;
			}
			_ => {}
		}
	}

	(movements, anchors)
}

fn calculate_shape_similarity<I: Instrument>(
	from: &Fingering,
	to: &Fingering,
	instrument: &I,
) -> i32 {
	let mut bonus = 0;

	if from.has_barre() && to.has_barre() {
		bonus += BARRE_SIMILARITY_BONUS;
	}

	if from.is_open_position_for(instrument) && to.is_open_position_for(instrument) {
		bonus += OPEN_POSITION_BONUS;
	}

	let from_count = from.strings().iter().filter(|s| s.is_played()).count();
	let to_count = to.strings().iter().filter(|s| s.is_played()).count();
	if (from_count as i32 - to_count as i32).abs() <= 1 {
		bonus += STRING_COUNT_SIMILARITY_BONUS;
	}

	bonus
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::instrument::Guitar;

	#[test]
	fn test_generate_simple_progression() {
		let guitar = Guitar::default();
		let chords = vec!["C", "G", "Am", "F"];
		let options = ProgressionOptions::default();

		let progressions = generate_progression(&chords, &guitar, &options);

		assert!(!progressions.is_empty());
		assert_eq!(progressions[0].chords.len(), 4);
		assert_eq!(progressions[0].fingerings.len(), 4);
		assert_eq!(progressions[0].transitions.len(), 3);
	}

	#[test]
	fn test_progression_respects_max_distance() {
		let guitar = Guitar::default();
		let chords = vec!["C", "F", "G"];
		let options = ProgressionOptions {
			max_fret_distance: 3,
			..Default::default()
		};

		let progressions = generate_progression(&chords, &guitar, &options);

		assert!(!progressions.is_empty());
		for progression in &progressions {
			for transition in &progression.transitions {
				assert!(transition.position_distance <= 3);
			}
		}
	}

	#[test]
	fn test_finger_changes_calculation() {
		let from = Fingering::parse("x32010").unwrap(); // C
		let to = Fingering::parse("x32013").unwrap(); // C with variation

		let (movements, anchors) = calculate_finger_changes(&from, &to);

		// Most strings stay the same, only high e string changes
		assert!(anchors > movements);
	}

	#[test]
	fn test_empty_chord_list() {
		let guitar = Guitar::default();
		let chords: Vec<&str> = vec![];
		let options = ProgressionOptions::default();

		let progressions = generate_progression(&chords, &guitar, &options);

		assert!(progressions.is_empty());
	}

	#[test]
	fn test_single_chord() {
		let guitar = Guitar::default();
		let chords = vec!["C"];
		let options = ProgressionOptions::default();

		let progressions = generate_progression(&chords, &guitar, &options);

		// Should return sequences with one chord, no transitions
		assert!(!progressions.is_empty());
		assert_eq!(progressions[0].chords.len(), 1);
		assert_eq!(progressions[0].transitions.len(), 0);
	}
}
