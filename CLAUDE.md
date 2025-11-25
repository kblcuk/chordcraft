# ChordCraft - Implementation Guide

## Project Vision

A multi-platform tool for bidirectional chord-fingering conversion:

- **Chord → Fingering**: Input chord name (e.g., "Abm7"), get multiple fingering options
- **Fingering → Chord**: Input tab notation (e.g., "x32010"), identify the chord
- **Multi-instrument**: Guitar-first, but designed to support bass, ukulele, mandolin, and eventually keys
- **Multi-platform**: CLI tool (immediate use), web app (Vue), potential mobile apps later

## Architecture

```
┌─────────────────────────────────────┐
│   Rust Core Library (chordcraft)   │
│  - Music theory engine              │
│  - Chord algorithms                 │
│  - Instrument models                │
└──────────┬──────────────────────────┘
           │
    ┌──────┴──────┬─────────────┐
    │             │             │
┌───▼───┐   ┌────▼────┐   ┌───▼────┐
│  CLI  │   │  WASM   │   │  FFI   │
│  Tool │   │(for web)│   │(future)│
└───────┘   └────┬────┘   └────────┘
                 │
            ┌────▼────┐
            │   Vue   │
            │ Web App │
            └─────────┘
```

## Repository Structure

```
chordcraft/
├── CLAUDE.md              # This file - implementation guide
├── README.md              # User-facing documentation
├── Cargo.toml             # Workspace definition
├── crates/
│   ├── core/              # Core music theory library
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── note.rs           # Note representation
│   │   │   ├── interval.rs       # Musical intervals
│   │   │   ├── chord.rs          # Chord theory & formulas
│   │   │   ├── instrument.rs     # Instrument trait & models
│   │   │   ├── fingering.rs      # Fingering representation
│   │   │   ├── generator.rs      # Chord → fingering algorithm
│   │   │   └── analyzer.rs       # Fingering → chord algorithm
│   │   └── Cargo.toml
│   ├── cli/               # CLI application
│   │   ├── src/
│   │   │   └── main.rs
│   │   └── Cargo.toml
│   └── wasm/              # WASM bindings (future)
│       ├── src/
│       │   └── lib.rs
│       └── Cargo.toml
└── web/                   # Vue application (future)
    ├── src/
    ├── package.json
    └── vite.config.js
```

## Implementation Phases

### Phase 1: Core Music Theory ✓ COMPLETE

**Goal**: Foundation for representing musical concepts

- [x] **Note representation** (`note.rs`)
  - Pitch classes (C, C#, D, etc.)
  - Enharmonic equivalents (C# = Db)
  - Octave-aware representation
  - Semitone calculations

- [x] **Interval system** (`interval.rs`)
  - Perfect, Major, Minor, Augmented, Diminished
  - Interval calculation between notes
  - Interval arithmetic (stack intervals)

- [x] **Chord formulas** (`chord.rs`)
  - Chord type definitions with interval patterns
  - Basic triads: Major [R, M3, P5], Minor [R, m3, P5], Dim, Aug
  - 7th chords: maj7, min7, dom7, min7b5, dim7
  - Extended chords: 9ths, 11ths, 13ths
  - Altered chords: sus2, sus4, add9, 7b9, 7#9, etc.
  - Chord name parser: "Abm7b5" → structured representation

- [x] **Instrument model** (`instrument.rs`)

  ```rust
  trait Instrument {
      fn tuning(&self) -> &[Note];
      fn fret_range(&self) -> (u8, u8);
      fn max_stretch(&self) -> u8;
      fn string_count(&self) -> usize;
      fn max_fingers(&self) -> u8;
      fn open_position_threshold(&self) -> u8;
      fn main_barre_threshold(&self) -> usize;
      fn min_played_strings(&self) -> usize;
  }
  ```

  - **Guitar** (standard tuning EADGBE)
    - max_stretch: 4 frets
    - min_played_strings: 3 (50% of 6 strings)
    - max_fingers: 4
  - **Ukulele** (standard tuning GCEA)
    - max_stretch: 5 frets (easier on shorter scale)
    - min_played_strings: 1 (allows minimal voicings like C="0003")
    - open_position_threshold: 5 frets
    - Only 4 strings, so lower min doesn't cause performance issues
  - Support for alternate tunings (future)
  - Other stringed instruments (future)

- [x] **Fingering representation** (`fingering.rs`)
  - Tab notation format (e.g., "x32010")
  - Fret positions per string
  - Physical validation (stretch, muted strings)
  - Playability scoring

### Phase 2: Fingering Generation (Chord → Tabs) ✓ COMPLETE

**Goal**: Given chord name, generate all playable fingerings

**Algorithm** (`generator.rs`):

1. Parse chord name to required notes/intervals
2. For each string, find positions where required notes appear (within fret range)
3. Generate combinations using **early pruning** to avoid combinatorial explosion:
   - Use recursive generation with branch pruning
   - Prune branches that exceed max_stretch (instrument-specific)
   - Prune branches that can't reach min_played_strings (instrument-specific)
   - Check constraints incrementally during generation, not after
   - This reduces 46K+ candidates down to ~100s for complex chords
4. Filter generated fingerings:
   - Must be physically playable (stretch, finger count)
   - Must have minimum played strings (instrument-specific)
   - Must include required notes based on voicing type
5. Score each fingering by:
   - Playability (stretch, barre requirements, finger efficiency)
   - Position (open vs. high on neck)
   - Voicing completeness (full > core > jazzy)
   - Voicing quality (root in bass, no interior mutes)
6. Deduplicate using HashSet on StringState vectors
7. Return top N fingerings, sorted by score

**Performance**:

- Simple chords (3 notes): ~4-5ms
- Complex chords (4+ notes): ~8-10ms
- Early pruning reduces search space by 99%+ for complex chords

**Voicing Classifications**:

- **Core**: Essential notes only (root, 3rd, 7th for 7th chords; root, 3rd, 5th for triads)
- **Full**: All chord tones present, no omissions
- **Jazzy**: Extended voicings, possible omissions of less essential notes (often 5th), jazz-style colorings

### Phase 3: Reverse Lookup (Tabs → Chord) ✓ COMPLETE

**Goal**: Given fingering notation, identify the chord

**Algorithm** (`analyzer.rs`):

1. Parse tab notation (e.g., "x32010")
2. Extract unique pitch classes from fingering
3. For each pitch class as potential root:
   - Calculate intervals from that root to all notes
   - Try to match intervals against all known chord qualities
4. Score each match by:
   - **Completeness** (0-100 points): percentage of required notes present
   - **Root in bass** (+20 points): bass note matches root
   - **Chord complexity** (+3 per required note): prefer more specific chords (G7 over G)
   - **Optional notes** (+5 per optional note present)
   - **Extra notes penalty** (-10 per note not in chord)
   - **Simplicity bonus** (+5 for major/minor if 100% complete)
5. Sort by score, deduplicate, return top matches

**Features**:
- ✅ Identifies 30+ chord types (triads, 7ths, extended, altered)
- ✅ Provides confidence percentage (completeness)
- ✅ Shows alternative interpretations
- ✅ Handles ambiguous fingerings (e.g., C vs Em/C)
- ✅ Prefers complete, specific chords (G7 over G when 7th is present)

### Phase 4: CLI Tool ✓ COMPLETE

**Goal**: Quick iteration, testing, and usable terminal tool

**Status**:

- ✅ `find` command fully implemented with all options
- ✅ `name` command implemented with analyzer integration
- ⏳ Chord progressions (future - Phase 6)

**Commands**:

```bash
# Find fingerings for a chord
chordcraft find "Abm7"
chordcraft find "Abm7" --limit 3
chordcraft find "Abm7" --position 7        # Prefer fingerings near 7th fret
chordcraft find "Abm7" --voicing core      # Show only core voicings

# Identify chord from fingering
chordcraft name "x32010"

# Chord progressions (Phase 5 - Planned)
chordcraft progression "Cmaj7 Am7 Dm7 G7"
chordcraft progression "Emaj7 D Bm Cmaj7" --limit 5 --max-distance 3
```

**Output format**:

```
Abm7 fingerings (top 5):

1. [Playability: 8/10, Position: Fret 4, Voicing: Full]
   e|---4---
   B|---4---
   G|---4---
   D|---5---
   A|---6---
   E|---4---
   Notes: Ab Eb Gb Cb (Db)

2. [Playability: 6/10, Position: Fret 11, Voicing: Core]
   ...

Show more? (y/n)
```

### Phase 5: Chord Progressions ✓ PLANNED

**Goal**: Find optimal fingering sequences for chord progressions

**Overview**:
Given a sequence of chords (e.g., "Emaj7 D Bm Cmaj7"), find fingering combinations that are easy to play together by minimizing finger movement and maximizing smooth transitions.

**Core Types** (`progression.rs`):

```rust
/// Options for progression generation
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

/// Scored transition between two fingerings
pub struct ChordTransition {
    pub from_chord: String,  // e.g., "Emaj7"
    pub to_chord: String,    // e.g., "D"
    pub from_fingering: ScoredFingering,
    pub to_fingering: ScoredFingering,
    pub score: i32,          // Transition ease score (higher = easier)
    pub finger_movements: usize,  // Number of fingers that move
    pub common_anchors: usize,    // Fingers that stay in place
    pub position_distance: u8,    // Fret distance between positions
}

/// Complete progression sequence with all fingerings and transitions
pub struct ProgressionSequence {
    pub chords: Vec<String>,           // Chord names in order
    pub fingerings: Vec<ScoredFingering>,  // Selected fingering for each chord
    pub transitions: Vec<ChordTransition>, // Transitions between consecutive chords
    pub total_score: i32,              // Sum of all transition scores
    pub avg_transition_score: f32,     // Average transition ease
}
```

**Algorithm** (`generate_progression()`):

1. **Parse & Generate Candidates**
   - Parse progression string into `Vec<Chord>`
   - For each chord, generate top N fingerings using existing generator
   - Store candidates: `Vec<Vec<ScoredFingering>>`

2. **Score All Transitions** (pairwise)
   - For each consecutive chord pair (chord[i] → chord[i+1])
   - Score all fingering combinations (N × N matrix)
   - Use `score_transition()` to evaluate each transition
   - Keep only transitions within `max_fret_distance` constraint

3. **Build Complete Progressions** (greedy approach)
   - Start with top fingering for first chord
   - For each subsequent chord, pick fingering with best transition from previous
   - Build K different progressions using different starting fingerings
   - Alternative: Use dynamic programming for true global optimization (future)

4. **Rank & Return**
   - Score complete progressions (sum of transition scores)
   - Sort by total score (descending)
   - Return top K progressions

**Transition Scoring Function**:

```rust
fn score_transition<I: Instrument>(
    from: &Fingering,
    to: &Fingering,
    from_pos: u8,
    to_pos: u8,
    instrument: &I,
) -> TransitionScore {
    let mut score = 100; // Base score

    // 1. FINGER MOVEMENT (Primary - most important)
    //    Calculate which fingers need to move to new positions
    let (movements, anchors) = calculate_finger_changes(from, to, instrument);
    score += (4 - movements as i32) * 30;  // Fewer movements = higher score

    // 2. COMMON ANCHORS (Secondary)
    //    Bonus for fingers that stay in same position
    score += (anchors as i32) * 20;

    // 3. SHAPE SIMILARITY (Tertiary)
    //    Bonus for similar fingering patterns (barre→barre, open→open)
    let shape_bonus = calculate_shape_similarity(from, to, instrument);
    score += shape_bonus;

    // 4. POSITION DISTANCE (Quaternary - mentioned by user)
    //    Penalty for large position jumps
    let distance = (to_pos as i32 - from_pos as i32).abs();
    score -= distance * 5;

    TransitionScore {
        score,
        movements,
        anchors,
        distance: distance as u8,
    }
}

/// Calculate how many fingers move and how many stay anchored
fn calculate_finger_changes<I: Instrument>(
    from: &Fingering,
    to: &Fingering,
    instrument: &I,
) -> (usize, usize) {
    // Compare fretted positions across strings
    // Count strings where fret changes (movement)
    // Count strings where fret stays same (anchor)
    // Smart: account for barres (one finger, multiple strings)
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
        bonus += 15;
    }

    // Both are open position
    if from.is_open_position_for(instrument) && to.is_open_position_for(instrument) {
        bonus += 10;
    }

    // Similar number of fretted strings
    let from_count = from.strings().iter().filter(|s| s.is_played()).count();
    let to_count = to.strings().iter().filter(|s| s.is_played()).count();
    if (from_count as i32 - to_count as i32).abs() <= 1 {
        bonus += 5;
    }

    bonus
}
```

**Performance Considerations**:

- For 4-chord progression with 20 candidates each:
  - Candidates: 4 × 20 = 80 fingerings
  - Transitions: 3 × (20 × 20) = 1,200 transition scores
  - Progressions: ~20-100 complete sequences to evaluate
- Expected time: <50ms (well within target)
- Optimization: Cache transition scores in HashMap if needed

**CLI Integration**:

```bash
# Basic usage
chordcraft progression "Emaj7 D Bm Cmaj7"

# With options
chordcraft progression "Emaj7 D Bm Cmaj7" --limit 5 --max-distance 3

# Prefer certain positions
chordcraft progression "Cmaj7 Am7 Dm7 G7" --position 3

# Filter by voicing
chordcraft progression "Cmaj7 Am7 Dm7 G7" --voicing core
```

**Output Format**:

```
Progression: Emaj7 → D → Bm → Cmaj7

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Alternative #1
Total Score: 285 | Avg Transition: 95.0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[1] Emaj7 - Fret 4
e|---4---
B|---5---
G|---4---
D|---6---
A|---7---
E|---x---
Score: 85 | Voicing: Full | Notes: E, G#, B, D#

  ↓ Transition Score: 92
    Movements: 2 fingers | Anchors: 1 | Distance: 2 frets

[2] D - Fret 5
e|---5---
B|---7---
G|---7---
D|---7---
A|---5---
E|---x---
Score: 78 | Voicing: Full | Notes: D, F#, A

  ↓ Transition Score: 88
    Movements: 3 fingers | Anchors: 0 | Distance: 2 frets

[3] Bm - Fret 7
e|---7---
B|---7---
G|---7---
D|---9---
A|---9---
E|---7---
Score: 81 | Voicing: Full | Notes: B, D, F#

  ↓ Transition Score: 105
    Movements: 1 finger | Anchors: 2 | Distance: 1 fret

[4] Cmaj7 - Fret 8
e|---8---
B|---8---
G|---9---
D|---9---
A|---10--
E|---8---
Score: 79 | Voicing: Core | Notes: C, E, G, B

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Alternative #2
Total Score: 278 | Avg Transition: 92.7
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[1] Emaj7 - Fret 0
...
```

**Enhanced Features**:

**1. Capo Support** (Prototype in `find` first, then add to progressions)

Makes difficult keys easier by transposing the instrument:

```rust
// Add to Instrument trait
trait Instrument {
    // ... existing methods ...

    /// Create a version of this instrument with a capo at the specified fret
    fn with_capo(&self, fret: u8) -> Self;
}

impl Guitar {
    fn with_capo(&self, fret: u8) -> Self {
        // Transpose tuning up by capo frets
        let new_tuning: Vec<Note> = self.tuning()
            .iter()
            .map(|note| Note::new(
                note.pitch.add_semitones(fret as i32),
                note.octave
            ))
            .collect();

        Guitar {
            tuning: new_tuning,
            max_fret: self.max_fret - fret,  // Reduce available frets
            // ... other fields unchanged
        }
    }
}
```

**CLI Usage:**
```bash
# Find F chord with capo on 3rd fret (shows D shape)
chordcraft find "F" --capo 3

# Progression in F with capo (much easier shapes!)
chordcraft progression "F Bb Gm C" --capo 3
```

**Display:**
- Show "Capo: 3" in output header
- Fingerings shown relative to capo (0 = capo position)
- Indicate actual chord vs shape: "D shape (capo 3) → F"

**2. Playing Context: Solo vs Band**

Adjusts voicing preferences based on whether you're playing with a band:

```rust
pub enum PlayingContext {
    Solo,  // Singer-songwriter, need full bass coverage
    Band,  // Bassist/keys cover bass, prefer compact voicings
}

// Add to GeneratorOptions
pub struct GeneratorOptions {
    // ... existing fields ...
    pub playing_context: PlayingContext,
}
```

**Scoring adjustments:**

**Solo mode (default):**
- Strong bonus for root in bass (+30)
- Prefer full voicings (+20)
- Prefer lower positions (fuller sound)
- Avoid jazzy voicings without bass notes

**Band mode:**
- Relaxed root in bass (+5 instead of +30)
- Prefer core/jazzy voicings (stay out of bass player's way)
- Prefer mid-neck positions (better mix clarity)
- Bonus for voicings that avoid low E/A strings (+10)

**CLI Usage:**
```bash
# Solo mode (default)
chordcraft find "Fmaj7"
chordcraft progression "F Bb C F"

# Band mode - lighter voicings
chordcraft find "Fmaj7" --context band
chordcraft progression "F Bb C F" --context band

# Combined with capo
chordcraft progression "F Bb Gm C" --capo 3 --context band
```

**Implementation Tasks**:

1. ✅ **Capo Support (Phase 5a - Prototype)**
   - Implement `with_capo()` for Instrument trait
   - Add `--capo` flag to `find` command
   - Update output formatting to show capo info
   - Write tests for capo functionality
   - Validate approach before adding to progressions

2. ✅ Create `progression.rs` module
   - Define core types (ProgressionOptions, ChordTransition, ProgressionSequence)
   - Implement progression parsing
   - Implement transition scoring functions
   - Implement progression generation algorithm

3. ✅ Add helper methods to `Fingering`
   - `has_barre()` - detect if fingering uses barre
   - Methods needed for finger movement calculation

4. ✅ **Playing Context Support** (COMPLETE)
   - ✅ Add `PlayingContext` enum (Solo/Band)
   - ✅ Add `playing_context` field to GeneratorOptions (defaults to Solo)
   - ✅ Adjust scoring in `score_fingering()` based on context:
     - Solo: +30 root in bass, +20 full voicings, prefer frets 0-5, -15 jazzy without root
     - Band: +5 root in bass, +20 core/jazzy voicings, prefer frets 3-10, +10 avoiding low E/A
   - ✅ Adjust transition scoring for band mode (40 movement weight, 8 distance penalty vs 30/5)
   - ✅ Add `--context` flag to CLI `find` and `progression` commands
   - ✅ Comprehensive unit tests validating scoring differences

5. ✅ Add CLI subcommand
   - Add `Progression` command to CLI
   - Parse options (limit, max-distance, position, voicing, context, capo)
   - Format and display progression output

6. ✅ Write tests
   - Unit tests for capo support
   - Unit tests for solo vs band scoring differences
   - Unit tests for transition scoring
   - Integration tests for common progressions (I-IV-V, ii-V-I, I-V-vi-IV)
   - Test capo + band mode combinations
   - Performance tests (ensure <100ms for 4-chord progressions)

7. ✅ Update documentation
   - Update CLAUDE.md with implementation status
   - Add doc comments with examples

**Testing Strategy**:

```rust
#[test]
fn test_capo_transposes_tuning() {
    let guitar = Guitar::default();
    let capo_guitar = guitar.with_capo(2);

    // Open strings should be 2 semitones higher
    assert_eq!(
        capo_guitar.tuning()[0].pitch,
        guitar.tuning()[0].pitch.add_semitones(2)
    );

    // Max fret should be reduced
    assert_eq!(capo_guitar.max_fret, guitar.max_fret - 2);
}

#[test]
fn test_find_with_capo_easier_shapes() {
    let guitar = Guitar::default();

    // F without capo - lots of barres
    let f_no_capo = generate_fingerings(
        &Chord::parse("F").unwrap(),
        &guitar,
        &GeneratorOptions::default()
    );

    // F with capo 3 (actually D shapes)
    let capo_guitar = guitar.with_capo(3);
    let chord = Chord::parse("F").unwrap().transpose(-3);  // Search for D
    let f_with_capo = generate_fingerings(
        &chord,
        &capo_guitar,
        &GeneratorOptions::default()
    );

    // Capo version should have easier shapes (lower avg playability score)
    let avg_no_capo = f_no_capo.iter().take(5)
        .map(|f| f.score).sum::<u8>() / 5;
    let avg_capo = f_with_capo.iter().take(5)
        .map(|f| f.score).sum::<u8>() / 5;

    assert!(avg_capo > avg_no_capo);
}

#[test]
fn test_band_mode_avoids_bass_strings() {
    let guitar = Guitar::default();
    let chord = Chord::parse("Cmaj7").unwrap();

    let solo_opts = GeneratorOptions {
        playing_context: PlayingContext::Solo,
        ..Default::default()
    };
    let band_opts = GeneratorOptions {
        playing_context: PlayingContext::Band,
        ..Default::default()
    };

    let solo_fingerings = generate_fingerings(&chord, &guitar, &solo_opts);
    let band_fingerings = generate_fingerings(&chord, &guitar, &band_opts);

    // Band mode should prefer fingerings without low E/A
    let band_low_strings = band_fingerings.iter().take(5)
        .filter(|f| {
            f.fingering.strings()[0].is_played() ||
            f.fingering.strings()[1].is_played()
        })
        .count();

    let solo_low_strings = solo_fingerings.iter().take(5)
        .filter(|f| {
            f.fingering.strings()[0].is_played() ||
            f.fingering.strings()[1].is_played()
        })
        .count();

    // Band mode should use low strings less frequently
    assert!(band_low_strings < solo_low_strings);
}

#[test]
fn test_transition_score_minimal_movement() {
    // Test that fingerings with minimal movement score higher
    let from = Fingering::parse("x32010").unwrap();  // C
    let to = Fingering::parse("x32013").unwrap();    // C with pinky change
    // Should score high (only 1 finger moves)
}

#[test]
fn test_common_progressions() {
    // Test classic progressions
    let guitar = Guitar::default();

    // I-IV-V in C: C-F-G
    let progression = generate_progression(
        &["C", "F", "G"],
        &guitar,
        &ProgressionOptions::default(),
    );

    assert!(!progression.is_empty());
    assert!(progression[0].avg_transition_score > 50.0);
}

#[test]
fn test_max_distance_constraint() {
    let guitar = Guitar::default();
    let options = ProgressionOptions {
        max_fret_distance: 3,
        ..Default::default()
    };

    let progression = generate_progression(
        &["C", "G", "Am", "F"],
        &guitar,
        &options,
    );

    // Verify all transitions respect max distance
    for trans in &progression[0].transitions {
        assert!(trans.position_distance <= 3);
    }
}

#[test]
fn test_capo_with_band_mode() {
    let guitar = Guitar::default();
    let capo_guitar = guitar.with_capo(3);

    let options = ProgressionOptions {
        playing_context: PlayingContext::Band,
        ..Default::default()
    };

    // Should work seamlessly together
    let progression = generate_progression(
        &["D", "G", "Em", "A"],  // Easy shapes with capo
        &capo_guitar,
        &options,
    );

    assert!(!progression.is_empty());
}
```

**Future Enhancements** (Phase 6+):

- Global optimization using dynamic programming (optimal substructure)
- Support for repeated sections (verse, chorus patterns)
- Strumming pattern integration (avoid difficult transitions on beat)
- Visual finger movement diagrams (which finger goes where)
- Support for slash chords in progressions (e.g., C/G)
- Save/load favorite progressions

**Edge Cases to Handle**:

- Single chord (no transitions to optimize)
- Progression where no fingerings meet distance constraint
- Very long progressions (10+ chords)
- Chords with very few fingering options

### Phase 6: Web App (Svelte + Rust WASM) ✓ PARTIALLY COMPLETE

**Goal**: Interactive visual interface

**Status**: Basic web app is functional with all three modes (find, name, progression)

**Completed Features**:
- ✅ Three modes with tab-based navigation
  - Find Fingerings: Input chord name, get fingerings
  - Name Chord: Input tab notation, identify chord
  - Progression: Input chord sequence, get optimal transitions
- ✅ WASM integration with full API exposure
  - All options available (limit, capo, voicing, position, playing context)
  - Fast generation (<15ms per chord)
  - 232 KB bundle size
- ✅ Basic results display
  - Tab notation, score, voicing type, position
  - Notes, root in bass indicator
  - Transition scores and finger movements (progressions)
- ✅ Responsive UI with Tailwind CSS
- ✅ Error handling and loading states

**Remaining Features** (Phase 6b):
- ⏳ **Advanced Controls** (HIGH PRIORITY)
  - Limit slider (1-50 fingerings)
  - Capo selector (0-12)
  - Voicing filter toggles (core/full/jazzy/all)
  - Position preference slider (0-12 frets)
  - Playing context toggle (solo/band)
  - Instrument selector (future: ukulele, bass)

- ⏳ **Chord Diagrams** (HIGH PRIORITY)
  - SVG-based fretboard visualization
  - Show finger positions on diagram
  - Highlight root notes
  - Indicate barres and finger numbers
  - Click-to-copy tab notation

- ⏳ **Interactive Fretboard Input** (MEDIUM PRIORITY)
  - Click strings/frets to build fingering
  - Visual feedback as you click
  - Auto-analyze as you build
  - Suggest similar fingerings

- ⏳ **Enhanced UX** (MEDIUM PRIORITY)
  - Chord name autocomplete
  - Common progressions presets (I-IV-V, ii-V-I, etc.)
  - "Show more" pagination for results
  - Keyboard shortcuts (Enter to search, etc.)
  - Save favorites to localStorage
  - Share fingerings via URL

- ⏳ **Mobile Optimization** (LOW PRIORITY)
  - Touch-friendly chord diagrams
  - Responsive fretboard input
  - Optimized layout for small screens

**Tech Stack**:
- Rust core compiled to WASM (wasm-pack)
- Svelte 5 with TypeScript
- Tailwind CSS for styling
- Vite for build tooling
- SVG for chord diagrams (planned)

## Key Design Decisions

### Why Algorithmic Over Database?

- Supports all instruments/tunings without manual curation
- Handles unusual/complex chords automatically
- More flexible for future features (voice leading, progressions)
- Consistent behavior across all chord types

### Instrument Abstraction

Generic `Instrument` trait allows guitar, bass, ukulele, mandolin to use the same core logic. Piano/keys would need different constraint model but can share chord theory.

### Voicing Classification System

Instead of binary "valid/invalid", classify voicings by use case:

- **Core**: For clarity, ensemble playing, when others cover the full harmony
- **Full**: For solo playing, complete harmonic picture
- **Jazzy**: For advanced players, color tones, sophisticated voicings

### Scoring Weights (to be tuned)

- Playability: How easy to finger (stretch, barres, hand position)
- Position: Preference for open/low positions (configurable)
- Voicing: Completeness, voice leading quality, root position
- Context: Match requested position/voicing type

### Future Extensibility

- ✅ **Chord progressions**: Optimize fingering transitions between chords (Phase 5 - Planned)
- **Scales/modes**: Use same interval system
- **Rhythm patterns**: Strumming/picking patterns for practice
- **Sound synthesis**: Generate audio previews (web audio API)
- **Advanced voice leading**: Global optimization across entire progression (dynamic programming)

## Code Quality & Architecture Notes

### API Design Principles

**Instrument-Aware API Pattern**: All fingering analysis methods require an explicit `Instrument` parameter. This design:
- Forces users to be explicit about instrument constraints
- Avoids hidden assumptions (no "default guitar" behavior)
- Supports multi-instrument use cases cleanly
- Makes code more maintainable (no hardcoded defaults scattered throughout)

Example:
```rust
// Good: Explicit about instrument
fingering.is_playable_for(&guitar)
fingering.playability_score_for(&ukulele)

// Removed: Generic methods with hidden assumptions
// fingering.is_playable(4)  // What instrument? What context?
```

### Music Theory Encapsulation

Music theory rules are encapsulated in methods rather than scattered as hardcoded logic:

```rust
impl ChordQuality {
    /// Check if the 5th can be omitted in voicings
    pub fn can_omit_fifth(&self) -> bool {
        // Encapsulates the rule: 7th chords can omit the 5th
        matches!(self, Dominant7 | Major7 | Minor7 | ...)
    }
}
```

This makes the codebase:
- Self-documenting (method name explains the rule)
- Easy to maintain (change rule in one place)
- Extensible (add new chord types without hunting for all uses)

### Scoring Architecture

Fingering scoring is separated into distinct concerns:

1. **Playability Scoring** (`Fingering::playability_score_for()`) - 0-100 scale
   - Physical difficulty (stretch, finger count, barre awkwardness)
   - Position preferences (open position bonus, high fret penalty)
   - Independent of chord context

2. **Generation Scoring** (`generator::score_fingering()`) - Unbounded
   - Builds on playability score
   - Adds chord-specific bonuses (root in bass, voicing completeness)
   - Adds musical preferences (interior mutes penalty, string coverage)
   - Used for ranking/sorting fingerings

This separation allows:
- Testing scoring components independently
- Tuning scoring weights without touching core logic
- Different scoring strategies for different use cases

## Development Guidelines

### Testing Strategy

- Unit tests for core music theory (intervals, chord formulas)
- Property-based tests for fingering generation (all generated fingerings must be valid)
- Integration tests for CLI commands
- Manual testing for playability scoring (needs musician feedback)

### Performance Considerations

**Optimizations Implemented**:

- **Early pruning** during combination generation (99%+ reduction in candidates)
  - Prune branches exceeding max_stretch incrementally
  - Prune branches that can't reach min_played_strings
  - Avoid allocations in pruning checks (inline min/max finding)
- **Fast deduplication** using HashSet<Vec<StringState>> instead of string comparisons
- **StringState** derives Hash for efficient deduplication
- Keep Vec::contains for small note sets (4-5 notes) - faster than HashSet overhead

**Performance Targets** (achieved):

- Simple chords (3 notes): <10ms
- Complex chords (4+ notes): <10ms
- CLI should feel instant for all operations

**Future Considerations**:

- Consider caching common chord fingerings if needed
- WASM bundle size matters for web app
- May add chord progression optimization (voice leading between chords)

### Code Style

- Idiomatic Rust (leverage type system, avoid panics)
- Comprehensive documentation for music theory concepts
- Examples in doc comments
- Clear error messages (especially for chord name parsing)
- Verify code quality with clippy and rustfmt

## Open Questions & Future Decisions

1. **Omission rules**: What can be safely omitted in each chord type?
2. **Scoring tuning**: What weights for playability vs. position vs. voicing?
3. **WASM architecture**: Full core in WASM, or thin API wrapper?
4. **Mobile strategy**: React Native + Rust FFI, or separate native apps?
5. **Audio**: Should we generate audio previews of chords?
6. **User accounts**: Save preferences, favorites across devices?

## Getting Started

1. **Prerequisites**: Rust toolchain (rustup), Node.js (for web later)
2. **Build**: `cargo build --workspace`
3. **Test**: `cargo test --workspace`
4. **Run CLI**: `cargo run -p chordcraft-cli -- find "Cmaj7"`
5. **Watch mode**: `cargo watch -x test -x run`

## References & Resources

- Music theory: Interval construction, chord formulas
- Guitar fingering: Physical constraints, common voicings
- Rust libraries: Consider `midly` for MIDI (future), `nom` for parsing
- WASM: `wasm-pack`, `wasm-bindgen`
- Vue: Vue 3 docs, Vite

---

**Last updated**: Phase 6 partially complete - Basic Svelte web app with all three modes operational
**Current status**:
- ✅ Phases 1-5 complete (Core, Generator, Analyzer, CLI, Progressions)
- ✅ Phase 6a complete (Basic Svelte web app with WASM integration)
- ⏳ Phase 6b in progress (Advanced controls, chord diagrams, enhanced UX)
**Next priorities**: Advanced controls OR chord diagram visualization
