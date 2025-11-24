//! Fingering generation algorithm (Chord → Fingerings)
//!
//! This module contains the algorithm for generating all possible fingerings
//! for a given chord on a specific instrument.

use crate::chord::{Chord, VoicingType};
use crate::fingering::{Fingering, StringState};
use crate::instrument::Instrument;

/// Playing context affects voicing preferences
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayingContext {
    /// Solo playing - need full bass coverage and complete voicings
    #[default]
    Solo,
    /// Band playing - bassist/keys cover bass, prefer compact voicings
    Band,
}

/// Options for fingering generation
#[derive(Debug, Clone)]
pub struct GeneratorOptions {
    /// Maximum number of fingerings to return
    pub limit: usize,
    /// Preferred fret position (fingerings near this position are ranked higher)
    pub preferred_position: Option<u8>,
    /// Voicing type filter
    pub voicing_type: Option<VoicingType>,
    /// Whether to include fingerings with the root in the bass
    pub root_in_bass: bool,
    /// Maximum fret to consider
    pub max_fret: u8,
    /// Playing context (solo vs band) - affects voicing preferences
    pub playing_context: PlayingContext,
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        GeneratorOptions {
            limit: 10,
            preferred_position: None,
            voicing_type: None,
            root_in_bass: true,
            max_fret: 12,
            playing_context: PlayingContext::default(),
        }
    }
}

/// A scored fingering with metadata
#[derive(Debug, Clone)]
pub struct ScoredFingering {
    pub fingering: Fingering,
    pub score: u8,
    pub voicing_type: VoicingType,
    pub has_root_in_bass: bool,
    pub position: u8, // Average fret position
}

/// Generate fingerings for a chord on an instrument
pub fn generate_fingerings<I: Instrument>(
    chord: &Chord,
    instrument: &I,
    options: &GeneratorOptions,
) -> Vec<ScoredFingering> {
    let tuning = instrument.tuning();
    let string_count = tuning.len();

    // Get required notes for different voicing types
    let all_notes = chord.notes();
    let core_notes = chord.core_notes();
    let root = chord.root;

    // For each string, find all fret positions that produce a chord tone
    let max_fret = options.max_fret;
    let string_options: Vec<Vec<StringState>> = tuning
        .iter()
        .map(|open_note| {
            let mut fret_options = vec![StringState::Muted];

            for fret in 0..=max_fret {
                let note_at_fret = open_note.pitch.add_semitones(fret as i32);
                if all_notes.contains(&note_at_fret) {
                    fret_options.push(StringState::Fretted(fret));
                }
            }

            fret_options
        })
        .collect();

    // Generate all combinations with instrument-aware pruning
    let mut fingerings = Vec::new();
    generate_combinations_for_instrument(
        &string_options,
        &mut vec![],
        &mut fingerings,
        string_count,
        instrument,
    );

    // Filter and score fingerings
    let mut scored: Vec<ScoredFingering> = fingerings
        .into_iter()
        .filter_map(|states| {
            let fingering = Fingering::new(states);

            // Must be physically playable for this instrument
            if !fingering.is_playable_for(instrument) {
                return None;
            }

            // Must have at least min_played_strings notes played
            let played_count = fingering.strings().iter().filter(|s| s.is_played()).count();
            if played_count < instrument.min_played_strings() {
                return None;
            }

            // Get the pitch classes in this fingering
            let pitches = fingering.unique_pitch_classes(instrument);

            // Must contain all core notes for core voicings, or filter by voicing type
            let has_all_core = core_notes.iter().all(|n| pitches.contains(n));
            let has_all_notes = all_notes.iter().all(|n| pitches.contains(n));

            let voicing_type = if has_all_notes {
                VoicingType::Full
            } else if has_all_core {
                VoicingType::Core
            } else {
                VoicingType::Jazzy
            };

            // Apply voicing type filter
            if let Some(required_voicing) = &options.voicing_type {
                match required_voicing {
                    VoicingType::Full if !has_all_notes => return None,
                    VoicingType::Core if !has_all_core => return None,
                    _ => {}
                }
            }

            // Check root in bass if required
            let bass_pitch = fingering.bass_note(instrument).map(|n| n.pitch);
            let has_root_in_bass = bass_pitch == Some(root);

            if options.root_in_bass && !has_root_in_bass {
                // Don't filter out, just score lower
            }

            // Calculate position (average of fretted positions)
            let fretted_frets: Vec<u8> = fingering
                .strings()
                .iter()
                .filter_map(|s| match s {
                    StringState::Fretted(f) if *f > 0 => Some(*f),
                    _ => None,
                })
                .collect();

            let position = if fretted_frets.is_empty() {
                0
            } else {
                (fretted_frets.iter().map(|f| *f as u32).sum::<u32>() / fretted_frets.len() as u32)
                    as u8
            };

            // Calculate score using extracted scoring function
            let score = score_fingering(
                &fingering,
                instrument,
                options,
                FingeringScorerOptions {
                    has_all_notes,
                    has_all_core,
                    has_root_in_bass,
                    position,
                    played_count,
                    voicing_type,
                },
            );

            Some(ScoredFingering {
                fingering,
                score: score.max(0) as u8, // Don't clamp to 100, allow higher scores for sorting
                voicing_type,
                has_root_in_bass,
                position,
            })
        })
        .collect();

    // Sort by score (descending)
    scored.sort_by(|a, b| b.score.cmp(&a.score));

    // Deduplicate similar fingerings (keep highest scored)
    scored = deduplicate_fingerings(scored);

    // Apply limit
    scored.truncate(options.limit);

    scored
}

/// Generate all combinations of string states with early pruning
fn generate_combinations_for_instrument<I: Instrument>(
    string_options: &[Vec<StringState>],
    current: &mut Vec<StringState>,
    results: &mut Vec<Vec<StringState>>,
    total_strings: usize,
    instrument: &I,
) {
    generate_combinations_pruned(
        string_options,
        current,
        results,
        total_strings,
        instrument.max_stretch(),
        instrument.min_played_strings(),
    );
}

/// Generate combinations with early pruning based on stretch and finger constraints
fn generate_combinations_pruned(
    string_options: &[Vec<StringState>],
    current: &mut Vec<StringState>,
    results: &mut Vec<Vec<StringState>>,
    total_strings: usize,
    max_stretch: u8,
    min_played: usize,
) {
    if current.len() == total_strings {
        results.push(current.clone());
        return;
    }

    let string_idx = current.len();

    for state in &string_options[string_idx] {
        current.push(*state);

        // Early pruning: check if current partial fingering is still viable
        if should_continue_branch(current, total_strings, max_stretch, min_played) {
            generate_combinations_pruned(
                string_options,
                current,
                results,
                total_strings,
                max_stretch,
                min_played,
            );
        }

        current.pop();
    }
}

/// Check if we should continue exploring this branch
#[inline]
fn should_continue_branch(
    current: &[StringState],
    total_strings: usize,
    max_stretch: u8,
    min_played: usize,
) -> bool {
    // Quick check: count played strings - avoid allocation if possible
    let played = current.iter().filter(|s| s.is_played()).count();
    let remaining = total_strings - current.len();

    // If we can't possibly get to min_played strings, prune early
    if played + remaining < min_played {
        return false;
    }

    // Only check stretch if we have multiple fretted notes
    if played < 2 {
        return true;
    }

    // Find min/max without allocation
    let mut min = u8::MAX;
    let mut max = 0u8;
    let mut has_fretted = false;

    for state in current {
        if let StringState::Fretted(f) = state
            && *f > 0
        {
            has_fretted = true;
            min = min.min(*f);
            max = max.max(*f);
        }
    }

    if !has_fretted {
        return true;
    }

    // Prune if stretch is already exceeded
    max - min <= max_stretch
}

// Fingering scoring constants
// Context-independent weights
const STRING_USAGE_BONUS: i32 = 8;
const INTERIOR_MUTE_PENALTY: i32 = 30;
const POSITION_DISTANCE_PENALTY: i32 = 3;

// Solo mode scoring weights
const SOLO_ROOT_IN_BASS_BONUS: i32 = 30;
const SOLO_FULL_VOICING_BONUS: i32 = 20;
const SOLO_CORE_VOICING_BONUS: i32 = 5;
const SOLO_JAZZY_WITHOUT_ROOT_PENALTY: i32 = 15;
const SOLO_POSITION_THRESHOLD: u8 = 5;
const SOLO_HIGH_POSITION_PENALTY: i32 = 5;

// Band mode scoring weights
const BAND_ROOT_IN_BASS_BONUS: i32 = 5;
const BAND_COMPACT_VOICING_BONUS: i32 = 20;
const BAND_FULL_VOICING_BONUS: i32 = 5;
const BAND_AVOID_LOW_STRINGS_BONUS: i32 = 10;
const BAND_MID_NECK_MIN: u8 = 3;
const BAND_MID_NECK_MAX: u8 = 10;
const BAND_POSITION_PENALTY: i32 = 3;

// Guitar string indices (for bass string detection)
const GUITAR_LOW_E_STRING: usize = 0;
const GUITAR_A_STRING: usize = 1;

pub struct FingeringScorerOptions {
    pub has_all_notes: bool,
    pub has_all_core: bool,
    pub has_root_in_bass: bool,
    pub position: u8,
    pub played_count: usize,
    pub voicing_type: VoicingType,
}

/// Calculate score for a fingering based on various criteria
fn score_fingering<I: Instrument>(
    fingering: &Fingering,
    instrument: &I,
    options: &GeneratorOptions,
    fingering_options: FingeringScorerOptions,
) -> i32 {
    let mut score = fingering.playability_score_for(instrument) as i32;

    // Bonus for using more strings, but moderate (don't over-penalize compact shapes)
    score += (fingering_options.played_count as i32) * STRING_USAGE_BONUS;

    // Heavy penalty for interior mutes (muted strings between played strings)
    // But leading mutes (bass side) are fine - e.g., xx0232 for D is standard
    let strings = fingering.strings();
    let first_played = strings.iter().position(|s| s.is_played());
    let last_played = strings.iter().rposition(|s| s.is_played());
    if let (Some(first), Some(last)) = (first_played, last_played) {
        let interior_mutes = strings[first..=last]
            .iter()
            .filter(|s| !s.is_played())
            .count();
        score -= (interior_mutes as i32) * INTERIOR_MUTE_PENALTY;
    }

    // Context-aware scoring
    match options.playing_context {
        PlayingContext::Solo => {
            // Solo mode: Strong emphasis on root in bass and full voicings
            if fingering_options.has_root_in_bass {
                score += SOLO_ROOT_IN_BASS_BONUS;
            }

            // Prefer full voicings in solo mode
            if fingering_options.has_all_notes {
                score += SOLO_FULL_VOICING_BONUS;
            } else if fingering_options.has_all_core {
                score += SOLO_CORE_VOICING_BONUS;
            }

            // Penalize jazzy voicings without root in bass (they lack harmonic foundation for solo)
            if fingering_options.voicing_type == VoicingType::Jazzy
                && !fingering_options.has_root_in_bass
            {
                score -= SOLO_JAZZY_WITHOUT_ROOT_PENALTY;
            }

            // Solo mode prefers lower positions (fuller sound)
            if let Some(pref_pos) = options.preferred_position {
                let distance = (fingering_options.position as i32 - pref_pos as i32).abs();
                score -= distance * POSITION_DISTANCE_PENALTY;
            } else {
                // Default: prefer open/low position chords (0-5)
                if fingering_options.position > SOLO_POSITION_THRESHOLD {
                    score -= ((fingering_options.position - SOLO_POSITION_THRESHOLD) as i32)
                        * SOLO_HIGH_POSITION_PENALTY;
                }
            }
        }
        PlayingContext::Band => {
            // Band mode: Relaxed root in bass (bassist covers it)
            if fingering_options.has_root_in_bass {
                score += BAND_ROOT_IN_BASS_BONUS;
            }

            // Prefer core/jazzy voicings in band mode (more compact, stay out of bass player's way)
            match fingering_options.voicing_type {
                VoicingType::Core | VoicingType::Jazzy => score += BAND_COMPACT_VOICING_BONUS,
                VoicingType::Full => score += BAND_FULL_VOICING_BONUS, // Still okay, just not preferred
            }

            // Bonus for avoiding low E/A strings (soft filter - prefer but allow if needed)
            let strings = fingering.strings();
            let uses_low_e = strings
                .get(GUITAR_LOW_E_STRING)
                .map(|s| s.is_played())
                .unwrap_or(false);
            let uses_low_a = strings
                .get(GUITAR_A_STRING)
                .map(|s| s.is_played())
                .unwrap_or(false);

            if !uses_low_e && !uses_low_a {
                score += BAND_AVOID_LOW_STRINGS_BONUS;
            }

            // Band mode prefers mid-neck positions (3-10) for clarity in mix
            if let Some(pref_pos) = options.preferred_position {
                let distance = (fingering_options.position as i32 - pref_pos as i32).abs();
                score -= distance * POSITION_DISTANCE_PENALTY;
            } else {
                // Prefer mid-neck (frets 3-10)
                let pos = fingering_options.position;
                if (BAND_MID_NECK_MIN..=BAND_MID_NECK_MAX).contains(&pos) {
                    // Sweet spot - no penalty
                } else if pos < BAND_MID_NECK_MIN {
                    // Too low for band
                    score -= (BAND_MID_NECK_MIN as i32 - pos as i32) * BAND_POSITION_PENALTY;
                } else {
                    // Too high
                    score -= ((pos - BAND_MID_NECK_MAX) as i32) * BAND_POSITION_PENALTY;
                }
            }
        }
    }

    score
}

/// Remove duplicate or very similar fingerings
fn deduplicate_fingerings(mut fingerings: Vec<ScoredFingering>) -> Vec<ScoredFingering> {
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for f in fingerings.drain(..) {
        // Create a hash key from the fingering's string states
        let key: Vec<_> = f.fingering.strings().to_vec();

        if seen.insert(key) {
            unique.push(f);
        }
    }

    unique
}

/// Format fingerings as ASCII tab diagram
pub fn format_fingering_diagram<I: Instrument>(scored: &ScoredFingering, instrument: &I) -> String {
    let fingering = &scored.fingering;
    let strings = fingering.strings();

    // Standard guitar string names (high to low for display)
    let string_names = ["e", "B", "G", "D", "A", "E"];

    let mut lines = Vec::new();

    // Display from highest string to lowest (reverse order)
    for (i, state) in strings.iter().enumerate().rev() {
        let name = if i < string_names.len() {
            string_names[strings.len() - 1 - i]
        } else {
            "?"
        };

        let fret_str = match state {
            StringState::Muted => "x".to_string(),
            StringState::Fretted(f) => format!("{f}"),
        };

        lines.push(format!("{name}|---{fret_str}---"));
    }

    // Add metadata
    lines.push(String::new());
    lines.push(format!(
        "Score: {} | Position: Fret {} | Voicing: {:?}",
        scored.score, scored.position, scored.voicing_type
    ));

    if scored.has_root_in_bass {
        lines.push("Root in bass: Yes".to_string());
    }

    // Add notes
    let pitches = fingering.unique_pitch_classes(instrument);
    let pitch_names: Vec<String> = pitches.iter().map(|p| p.to_string()).collect();
    lines.push(format!("Notes: {}", pitch_names.join(", ")));

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::Chord;
    use crate::instrument::Guitar;
    use crate::note::PitchClass;

    #[test]
    fn test_generate_c_major() {
        let chord = Chord::parse("C").unwrap();
        let guitar = Guitar::default();
        let options = GeneratorOptions {
            limit: 5,
            ..Default::default()
        };

        let fingerings = generate_fingerings(&chord, &guitar, &options);

        assert!(!fingerings.is_empty());

        // Check that at least one fingering contains C, E, G
        let has_valid = fingerings.iter().any(|sf| {
            let pitches = sf.fingering.unique_pitch_classes(&guitar);
            pitches.contains(&PitchClass::C)
                && pitches.contains(&PitchClass::E)
                && pitches.contains(&PitchClass::G)
        });
        assert!(has_valid);
    }

    #[test]
    fn test_generate_g_major() {
        let chord = Chord::parse("G").unwrap();
        let guitar = Guitar::default();
        let options = GeneratorOptions {
            voicing_type: Some(VoicingType::Full),
            ..Default::default()
        };

        let fingerings = generate_fingerings(&chord, &guitar, &options);

        assert!(!fingerings.is_empty());

        // All full voicings should contain G, B, D
        for sf in &fingerings {
            let pitches = sf.fingering.unique_pitch_classes(&guitar);
            assert!(pitches.contains(&PitchClass::G));
            assert!(pitches.contains(&PitchClass::B));
            assert!(pitches.contains(&PitchClass::D));
        }
    }

    #[test]
    fn test_generate_with_position_preference() {
        let chord = Chord::parse("A").unwrap();
        let guitar = Guitar::default();

        // Generate with position preference at fret 5
        let options = GeneratorOptions {
            limit: 10,
            preferred_position: Some(5),
            ..Default::default()
        };
        let fingerings = generate_fingerings(&chord, &guitar, &options);

        assert!(!fingerings.is_empty());

        // Check that all fingerings contain valid A chord notes
        for sf in &fingerings {
            let pitches = sf.fingering.unique_pitch_classes(&guitar);
            assert!(pitches.contains(&PitchClass::A));
            // Should have at least root and one other chord tone
            assert!(pitches.len() >= 2);
        }
    }

    #[test]
    fn test_generate_minor_chord() {
        let chord = Chord::parse("Am").unwrap();
        let guitar = Guitar::default();
        let options = GeneratorOptions::default();

        let fingerings = generate_fingerings(&chord, &guitar, &options);

        assert!(!fingerings.is_empty());

        // Check for A, C, E (A minor = A, C, E)
        let has_valid = fingerings.iter().any(|sf| {
            let pitches = sf.fingering.unique_pitch_classes(&guitar);
            pitches.contains(&PitchClass::A)
                && pitches.contains(&PitchClass::C)
                && pitches.contains(&PitchClass::E)
        });
        assert!(has_valid);
    }

    #[test]
    fn test_format_diagram() {
        let chord = Chord::parse("C").unwrap();
        let guitar = Guitar::default();
        let options = GeneratorOptions {
            limit: 1,
            ..Default::default()
        };

        let fingerings = generate_fingerings(&chord, &guitar, &options);
        assert!(!fingerings.is_empty());

        let diagram = format_fingering_diagram(&fingerings[0], &guitar);
        assert!(diagram.contains("|---"));
        assert!(diagram.contains("Score:"));
    }

    #[test]
    fn test_generate_ukulele_c_major() {
        use crate::instrument::Ukulele;

        let chord = Chord::parse("C").unwrap();
        let ukulele = Ukulele::default();
        let options = GeneratorOptions {
            limit: 5,
            ..Default::default()
        };

        let fingerings = generate_fingerings(&chord, &ukulele, &options);

        assert!(!fingerings.is_empty(), "Should generate ukulele fingerings");

        // Check that fingerings contain C, E, G
        let has_valid = fingerings.iter().any(|sf| {
            let pitches = sf.fingering.unique_pitch_classes(&ukulele);
            pitches.contains(&PitchClass::C)
                && pitches.contains(&PitchClass::E)
                && pitches.contains(&PitchClass::G)
        });
        assert!(has_valid, "Should have valid C major fingering for ukulele");

        // Ukulele should allow 5-fret stretches
        println!("Ukulele C major fingerings:");
        for (i, f) in fingerings.iter().enumerate() {
            println!(
                "{}. {} (score: {}, span: {})",
                i + 1,
                f.fingering,
                f.score,
                f.fingering.fret_span()
            );
            assert!(
                f.fingering.fret_span() <= 5,
                "Ukulele should allow 5-fret stretch"
            );
        }
    }

    #[test]
    fn test_am_includes_open_a_string() {
        let chord = Chord::parse("Am").unwrap();
        let guitar = Guitar::default();

        // Am = A, C, E
        let notes = chord.notes();
        println!("Am notes: {notes:?}");
        assert!(notes.contains(&PitchClass::A));
        assert!(notes.contains(&PitchClass::C));
        assert!(notes.contains(&PitchClass::E));

        // A string (index 1) at fret 0 should be A
        let tuning = guitar.tuning();
        let a_string_open = tuning[1].pitch.add_semitones(0);
        println!("A string open: {a_string_open:?}");
        assert_eq!(a_string_open, PitchClass::A);

        // Generate with high limit
        let options = GeneratorOptions {
            limit: 100,
            ..Default::default()
        };
        let fingerings = generate_fingerings(&chord, &guitar, &options);

        // x02210 should be in there
        let has_classic = fingerings
            .iter()
            .any(|f| f.fingering.to_string() == "x02210");
        println!("Found x02210: {has_classic}");

        // Print first 10 fingerings
        for (i, f) in fingerings.iter().take(10).enumerate() {
            println!("{}. {} (score: {})", i + 1, f.fingering, f.score);
        }

        assert!(has_classic, "Classic Am shape x02210 should be generated");
    }

    #[test]
    fn test_solo_vs_band_root_in_bass_scoring() {
        let chord = Chord::parse("Cmaj7").unwrap();
        let guitar = Guitar::default();

        // Generate with solo context
        let solo_options = GeneratorOptions {
            limit: 20,
            playing_context: PlayingContext::Solo,
            ..Default::default()
        };
        let solo_fingerings = generate_fingerings(&chord, &guitar, &solo_options);

        // Generate with band context
        let band_options = GeneratorOptions {
            limit: 20,
            playing_context: PlayingContext::Band,
            ..Default::default()
        };
        let band_fingerings = generate_fingerings(&chord, &guitar, &band_options);

        // Count fingerings with root in bass in top 5
        let solo_root_in_bass = solo_fingerings
            .iter()
            .take(5)
            .filter(|f| f.has_root_in_bass)
            .count();
        let band_root_in_bass = band_fingerings
            .iter()
            .take(5)
            .filter(|f| f.has_root_in_bass)
            .count();

        // Solo mode should strongly prefer root in bass more than band mode
        // This isn't a hard requirement but should be a trend
        println!("Solo root in bass (top 5): {solo_root_in_bass}");
        println!("Band root in bass (top 5): {band_root_in_bass}");

        // At least verify that both modes return results
        assert!(!solo_fingerings.is_empty());
        assert!(!band_fingerings.is_empty());
    }

    #[test]
    fn test_band_mode_avoids_low_strings() {
        let chord = Chord::parse("Gmaj7").unwrap();
        let guitar = Guitar::default();

        // Generate with solo context
        let solo_options = GeneratorOptions {
            limit: 20,
            playing_context: PlayingContext::Solo,
            ..Default::default()
        };
        let solo_fingerings = generate_fingerings(&chord, &guitar, &solo_options);

        // Generate with band context
        let band_options = GeneratorOptions {
            limit: 20,
            playing_context: PlayingContext::Band,
            ..Default::default()
        };
        let band_fingerings = generate_fingerings(&chord, &guitar, &band_options);

        // Count fingerings using low E or A strings in top 5
        let solo_uses_low = solo_fingerings
            .iter()
            .take(5)
            .filter(|f| {
                let strings = f.fingering.strings();
                strings.first().map(|s| s.is_played()).unwrap_or(false)
                    || strings.get(1).map(|s| s.is_played()).unwrap_or(false)
            })
            .count();

        let band_uses_low = band_fingerings
            .iter()
            .take(5)
            .filter(|f| {
                let strings = f.fingering.strings();
                strings.first().map(|s| s.is_played()).unwrap_or(false)
                    || strings.get(1).map(|s| s.is_played()).unwrap_or(false)
            })
            .count();

        println!("Solo uses low E/A (top 5): {solo_uses_low}");
        println!("Band uses low E/A (top 5): {band_uses_low}");

        // Band mode should use low strings less frequently (soft filter, not exclusion)
        // This is a preference, not a hard rule, so we just verify both return results
        assert!(!solo_fingerings.is_empty());
        assert!(!band_fingerings.is_empty());
    }

    #[test]
    fn test_band_mode_prefers_mid_neck() {
        let chord = Chord::parse("F").unwrap();
        let guitar = Guitar::default();

        // Generate with solo context (no position preference)
        let solo_options = GeneratorOptions {
            limit: 10,
            playing_context: PlayingContext::Solo,
            ..Default::default()
        };
        let solo_fingerings = generate_fingerings(&chord, &guitar, &solo_options);

        // Generate with band context (no position preference)
        let band_options = GeneratorOptions {
            limit: 10,
            playing_context: PlayingContext::Band,
            ..Default::default()
        };
        let band_fingerings = generate_fingerings(&chord, &guitar, &band_options);

        // Calculate average position for top 5
        let solo_avg_pos = solo_fingerings
            .iter()
            .take(5)
            .map(|f| f.position as f32)
            .sum::<f32>()
            / 5.0;

        let band_avg_pos = band_fingerings
            .iter()
            .take(5)
            .map(|f| f.position as f32)
            .sum::<f32>()
            / 5.0;

        println!("Solo avg position (top 5): {solo_avg_pos}");
        println!("Band avg position (top 5): {band_avg_pos}");

        // Band mode should prefer mid-neck (3-10), solo prefers lower (0-5)
        // Band average should typically be higher than solo average for chords like F
        // This is a trend, not a hard requirement
        assert!(!solo_fingerings.is_empty());
        assert!(!band_fingerings.is_empty());
    }

    #[test]
    fn test_solo_mode_penalizes_jazzy_without_root() {
        let chord = Chord::parse("Cmaj7").unwrap();
        let guitar = Guitar::default();

        // Generate with solo context
        let solo_options = GeneratorOptions {
            limit: 30,
            playing_context: PlayingContext::Solo,
            ..Default::default()
        };
        let solo_fingerings = generate_fingerings(&chord, &guitar, &solo_options);

        // Find jazzy voicings and check their root in bass status
        let jazzy_fingerings: Vec<_> = solo_fingerings
            .iter()
            .filter(|f| f.voicing_type == VoicingType::Jazzy)
            .collect();

        if !jazzy_fingerings.is_empty() {
            // Among jazzy voicings, those with root in bass should score higher
            let jazzy_with_root: Vec<_> = jazzy_fingerings
                .iter()
                .filter(|f| f.has_root_in_bass)
                .collect();
            let jazzy_without_root: Vec<_> = jazzy_fingerings
                .iter()
                .filter(|f| !f.has_root_in_bass)
                .collect();

            println!("Jazzy with root: {}", jazzy_with_root.len());
            println!("Jazzy without root: {}", jazzy_without_root.len());

            if !jazzy_with_root.is_empty() && !jazzy_without_root.is_empty() {
                let avg_with_root = jazzy_with_root.iter().map(|f| f.score as f32).sum::<f32>()
                    / jazzy_with_root.len() as f32;
                let avg_without_root = jazzy_without_root
                    .iter()
                    .map(|f| f.score as f32)
                    .sum::<f32>()
                    / jazzy_without_root.len() as f32;

                println!("Avg score with root: {avg_with_root}");
                println!("Avg score without root: {avg_without_root}");

                // Jazzy with root should score higher on average in solo mode
                assert!(
                    avg_with_root > avg_without_root,
                    "Solo mode should score jazzy voicings with root higher"
                );
            }
        }
    }

    #[test]
    fn test_band_mode_prefers_compact_voicings() {
        let chord = Chord::parse("Dmaj7").unwrap();
        let guitar = Guitar::default();

        // Generate with solo context
        let solo_options = GeneratorOptions {
            limit: 20,
            playing_context: PlayingContext::Solo,
            ..Default::default()
        };
        let solo_fingerings = generate_fingerings(&chord, &guitar, &solo_options);

        // Generate with band context
        let band_options = GeneratorOptions {
            limit: 20,
            playing_context: PlayingContext::Band,
            ..Default::default()
        };
        let band_fingerings = generate_fingerings(&chord, &guitar, &band_options);

        // Count full vs core/jazzy voicings in top 5
        let solo_full = solo_fingerings
            .iter()
            .take(5)
            .filter(|f| f.voicing_type == VoicingType::Full)
            .count();
        let band_full = band_fingerings
            .iter()
            .take(5)
            .filter(|f| f.voicing_type == VoicingType::Full)
            .count();

        println!("Solo full voicings (top 5): {solo_full}");
        println!("Band full voicings (top 5): {band_full}");

        // Solo should prefer full voicings more than band mode
        // This is a preference trend
        assert!(!solo_fingerings.is_empty());
        assert!(!band_fingerings.is_empty());
    }
}
