//! Fingering generation algorithm (Chord → Fingerings)
//!
//! This module contains the algorithm for generating all possible fingerings
//! for a given chord on a specific instrument.

use crate::chord::{Chord, VoicingType};
use crate::fingering::{Fingering, StringState};
use crate::instrument::Instrument;

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
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        GeneratorOptions {
            limit: 10,
            preferred_position: None,
            voicing_type: None,
            root_in_bass: true,
            max_fret: 12,
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
    pub position: u8,  // Average fret position
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
                (fretted_frets.iter().map(|f| *f as u32).sum::<u32>() / fretted_frets.len() as u32) as u8
            };

            // Calculate score using instrument-aware playability
            let mut score = fingering.playability_score_for(instrument) as i32;

            // Bonus for using more strings, but moderate (don't over-penalize compact shapes)
            score += (played_count as i32) * 8;

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
                score -= (interior_mutes as i32) * 30;
            }

            // Bonus for root in bass
            if has_root_in_bass {
                score += 20;
            }

            // Bonus for full voicing
            if has_all_notes {
                score += 15;
            } else if has_all_core {
                score += 5;
            }

            // Bonus/penalty for position preference
            if let Some(pref_pos) = options.preferred_position {
                let distance = (position as i32 - pref_pos as i32).abs();
                score -= distance * 3;
            } else {
                // Default: prefer open/low position chords
                if position > 5 {
                    score -= ((position - 5) as i32) * 5;
                }
            }

            Some(ScoredFingering {
                fingering,
                score: score.max(0) as u8,  // Don't clamp to 100, allow higher scores for sorting
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
            generate_combinations_pruned(string_options, current, results, total_strings, max_stretch, min_played);
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
        if let StringState::Fretted(f) = state {
            if *f > 0 {
                has_fretted = true;
                min = min.min(*f);
                max = max.max(*f);
            }
        }
    }

    if !has_fretted {
        return true;
    }

    // Prune if stretch is already exceeded
    max - min <= max_stretch
}

/// Remove duplicate or very similar fingerings
fn deduplicate_fingerings(mut fingerings: Vec<ScoredFingering>) -> Vec<ScoredFingering> {
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for f in fingerings.drain(..) {
        // Create a simple hash key from the fingering without string allocation
        let key: Vec<_> = f.fingering.strings().to_vec();

        if seen.insert(key) {
            unique.push(f);
        }
    }

    unique
}

/// Format fingerings as ASCII tab diagram
pub fn format_fingering_diagram<I: Instrument>(
    scored: &ScoredFingering,
    instrument: &I,
) -> String {
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
            println!("{}. {} (score: {}, span: {})",
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
        let has_classic = fingerings.iter().any(|f| f.fingering.to_string() == "x02210");
        println!("Found x02210: {has_classic}");

        // Print first 10 fingerings
        for (i, f) in fingerings.iter().take(10).enumerate() {
            println!("{}. {} (score: {})", i + 1, f.fingering, f.score);
        }

        assert!(has_classic, "Classic Am shape x02210 should be generated");
    }
}
