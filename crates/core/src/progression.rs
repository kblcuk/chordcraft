//! Chord progression optimization
//!
//! This module provides algorithms for finding optimal fingering sequences
//! for chord progressions, minimizing finger movement and maximizing smooth transitions.

use crate::chord::Chord;
use crate::fingering::Fingering;
use crate::generator::{GeneratorOptions, PlayingContext, ScoredFingering, generate_fingerings};
use crate::instrument::Instrument;

// Transition scoring constants
// These weights define the priority of different transition qualities

/// Base score for any transition
const BASE_SCORE: i32 = 100;

/// Weight for each finger saved from moving (Primary factor)
/// Higher value = fewer finger movements are strongly preferred
const MOVEMENT_WEIGHT: i32 = 30;

/// Bonus points for each finger that stays anchored (Secondary factor)
/// Anchored fingers make transitions more stable and easier
const ANCHOR_BONUS: i32 = 20;

/// Bonus for similar barre patterns (Tertiary factor)
/// Transitions between barres feel more natural
const BARRE_SIMILARITY_BONUS: i32 = 15;

/// Bonus for both fingerings in open position (Tertiary factor)
/// Open position transitions are generally easier
const OPEN_POSITION_BONUS: i32 = 10;

/// Bonus for similar number of fretted strings (Tertiary factor)
/// Similar hand shapes are easier to transition between
const STRING_COUNT_SIMILARITY_BONUS: i32 = 5;

/// Penalty multiplier for fret distance (Quaternary factor)
/// Each fret of distance reduces score by this amount
const DISTANCE_PENALTY: i32 = 5;

// Band mode adjustments - prioritize compact movements over full voicings
/// Band mode movement weight (increased emphasis on minimal movement)
const BAND_MOVEMENT_WEIGHT: i32 = 40;
/// Band mode distance penalty (stronger penalty for position jumps)
const BAND_DISTANCE_PENALTY: i32 = 8;

/// Options for progression generation
#[derive(Debug, Clone)]
pub struct ProgressionOptions {
    /// Number of alternative progressions to show (default: 3)
    pub limit: usize,
    /// Maximum fret distance between consecutive fingerings (default: 3)
    pub max_fret_distance: u8,
    /// Number of fingering candidates to consider per chord (default: 20)
    pub candidates_per_chord: usize,
    /// Options for generating fingerings for each chord
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

/// Scored transition between two fingerings
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

/// Complete progression sequence with all fingerings and transitions
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
    // Parse all chords
    let chords: Vec<Chord> = chord_names
        .iter()
        .filter_map(|name| Chord::parse(name).ok())
        .collect();

    if chords.is_empty() {
        return vec![];
    }

    // Generate candidates for each chord
    let mut candidates: Vec<Vec<ScoredFingering>> = Vec::new();
    for chord in &chords {
        let mut opts = options.generator_options.clone();
        opts.limit = options.candidates_per_chord;
        let fingerings = generate_fingerings(chord, instrument, &opts);
        candidates.push(fingerings);
    }

    // If any chord has no fingerings, we can't build a progression
    if candidates.iter().any(|c| c.is_empty()) {
        return vec![];
    }

    // Build multiple progression sequences using different starting fingerings
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

    // Sort by total score (descending)
    sequences.sort_by(|a, b| b.total_score.cmp(&a.total_score));

    // Return top N
    sequences.truncate(options.limit);
    sequences
}

/// Build a single progression sequence starting from a specific fingering
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

    // Start with the specified fingering
    selected_fingerings.push(candidates[0][start_idx].clone());

    // For each subsequent chord, pick the fingering with the best transition
    for i in 1..chords.len() {
        let from = &selected_fingerings[i - 1];
        let from_chord_name = chord_names[i - 1].to_string();
        let to_chord_name = chord_names[i].to_string();

        // Score all possible transitions to fingerings of the next chord
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

            // Skip if transition exceeds max distance
            if transition.position_distance > options.max_fret_distance {
                continue;
            }

            if best_transition.is_none()
                || transition.score > best_transition.as_ref().unwrap().0.score
            {
                best_transition = Some((transition, to.clone()));
            }
        }

        // If no valid transition found, bail out
        let (transition, to_fingering) = best_transition?;

        transitions.push(transition);
        selected_fingerings.push(to_fingering);
    }

    // Calculate total score
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

/// Score a transition between two fingerings
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

    // Context-aware weight selection
    let (movement_weight, distance_penalty) = match playing_context {
        PlayingContext::Solo => (MOVEMENT_WEIGHT, DISTANCE_PENALTY),
        PlayingContext::Band => (BAND_MOVEMENT_WEIGHT, BAND_DISTANCE_PENALTY),
    };

    // 1. FINGER MOVEMENT (Primary - most important)
    // Band mode uses higher weight to prioritize compact movements
    let (movements, anchors) = calculate_finger_changes(from, to);
    score += (4_i32.saturating_sub(movements as i32)) * movement_weight;

    // 2. COMMON ANCHORS (Secondary)
    score += (anchors as i32) * ANCHOR_BONUS;

    // 3. SHAPE SIMILARITY (Tertiary)
    let shape_bonus = calculate_shape_similarity(from, to, instrument);
    score += shape_bonus;

    // 4. POSITION DISTANCE (Quaternary)
    // Band mode uses stronger penalty for position jumps
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

/// Calculate how many fingers move and how many stay anchored
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
            _ => {} // Both muted, no change
        }
    }

    (movements, anchors)
}

/// Calculate similarity between fingering shapes/patterns
fn calculate_shape_similarity<I: Instrument>(
    from: &Fingering,
    to: &Fingering,
    instrument: &I,
) -> i32 {
    let mut bonus = 0;

    // Both use barre in similar position
    if from.has_barre() && to.has_barre() {
        bonus += BARRE_SIMILARITY_BONUS;
    }

    // Both are open position
    if from.is_open_position_for(instrument) && to.is_open_position_for(instrument) {
        bonus += OPEN_POSITION_BONUS;
    }

    // Similar number of fretted strings
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
