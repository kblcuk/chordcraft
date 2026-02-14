//! Chord progression optimization
//!
//! This module provides algorithms for finding optimal fingering sequences
//! for chord progressions, minimizing finger movement and maximizing smooth transitions.

use crate::chord::Chord;
use crate::fingering::Fingering;
use crate::generator::{GeneratorOptions, PlayingContext, ScoredFingering, generate_fingerings};
use crate::instrument::Instrument;
use crate::shapes;

const BASE_SCORE: i32 = 100;
const MOVEMENT_WEIGHT: i32 = 30;
const ANCHOR_BONUS: i32 = 20;
const BARRE_SIMILARITY_BONUS: i32 = 15;
const OPEN_POSITION_BONUS: i32 = 10;
const STRING_COUNT_SIMILARITY_BONUS: i32 = 5;
const DISTANCE_PENALTY: i32 = 5;
const SAME_SHAPE_SLIDE_BONUS: i32 = 50;
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

	// Beam search: keep top-K partial sequences at each step
	let beam_width = (options.limit * 3).max(10); // wider beam for better results

	let sequences =
		beam_search_progression(chord_names, &candidates, beam_width, instrument, options);

	let mut result: Vec<ProgressionSequence> = sequences;
	result.sort_by(|a, b| b.total_score.cmp(&a.total_score));
	result.truncate(options.limit);
	result
}

/// A partial sequence being built during beam search
struct BeamCandidate {
	fingerings: Vec<ScoredFingering>,
	transitions: Vec<ChordTransition>,
	total_score: i32,
}

fn beam_search_progression<I: Instrument>(
	chord_names: &[&str],
	candidates: &[Vec<ScoredFingering>],
	beam_width: usize,
	instrument: &I,
	options: &ProgressionOptions,
) -> Vec<ProgressionSequence> {
	// Initialize beam with all first-chord candidates
	let mut beam: Vec<BeamCandidate> = candidates[0]
		.iter()
		.map(|sf| BeamCandidate {
			fingerings: vec![sf.clone()],
			transitions: vec![],
			total_score: 0,
		})
		.collect();

	// Expand beam for each subsequent chord
	for i in 1..candidates.len() {
		let mut next_beam: Vec<BeamCandidate> = Vec::new();
		let from_chord_name = chord_names[i - 1].to_string();
		let to_chord_name = chord_names[i].to_string();

		for candidate in &beam {
			let from = candidate.fingerings.last().unwrap();

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

				let new_total = candidate.total_score + transition.score;
				let mut new_fingerings = candidate.fingerings.clone();
				new_fingerings.push(to.clone());
				let mut new_transitions = candidate.transitions.clone();
				new_transitions.push(transition);

				next_beam.push(BeamCandidate {
					fingerings: new_fingerings,
					transitions: new_transitions,
					total_score: new_total,
				});
			}
		}

		// Prune to beam width: keep top-K by total score
		next_beam.sort_by(|a, b| b.total_score.cmp(&a.total_score));
		next_beam.truncate(beam_width);
		beam = next_beam;

		if beam.is_empty() {
			return vec![];
		}
	}

	// Convert beam candidates to final sequences
	beam.into_iter()
		.map(|candidate| {
			let total_score = candidate.total_score;
			let avg_transition_score = if candidate.transitions.is_empty() {
				0.0
			} else {
				total_score as f32 / candidate.transitions.len() as f32
			};
			ProgressionSequence {
				chords: chord_names.iter().map(|s| s.to_string()).collect(),
				fingerings: candidate.fingerings,
				transitions: candidate.transitions,
				total_score,
				avg_transition_score,
			}
		})
		.collect()
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

	// Check if both fingerings match the same standard shape (barre slide)
	// This is the easiest transition: same hand shape, just slide up/down the neck
	let from_shape = find_shape_for_instrument(from, instrument);
	let to_shape = find_shape_for_instrument(to, instrument);

	if let (Some((from_name, _)), Some((to_name, _))) = (from_shape, to_shape)
		&& from_name == to_name
	{
		bonus += SAME_SHAPE_SLIDE_BONUS;
	}

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

/// Find which standard shape a fingering matches for the given instrument
fn find_shape_for_instrument<I: Instrument>(
	fingering: &Fingering,
	instrument: &I,
) -> Option<(&'static str, u8)> {
	match instrument.string_count() {
		6 => shapes::guitar::find_matching_shape(fingering),
		4 => shapes::ukulele::find_matching_shape(fingering)
			.or_else(|| shapes::mandolin::find_matching_shape(fingering)),
		5 => shapes::banjo::find_matching_shape(fingering),
		_ => None,
	}
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
