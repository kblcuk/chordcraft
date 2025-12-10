# ChordCraft - Implementation Guide

## Project Vision

A multi-platform tool for bidirectional chord-fingering conversion:

- **Chord → Fingering**: Input chord name (e.g., "Abm7"), get multiple fingering options
- **Fingering → Chord**: Input tab notation (e.g., "x32010"), identify the chord
- **Multi-instrument**: Guitar and ukulele fully supported (CLI). Web app supports guitar, ukulele integration planned. Designed for bass, mandolin, and eventually keys.
- **Multi-platform**: CLI tool (immediate use), web app (SvelteKit), potential mobile apps later

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
            ┌────▼──────┐
            │ SvelteKit │
            │  Web App  │
            └───────────┘
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
│   └── wasm/              # WASM bindings
│       ├── src/
│       │   └── lib.rs
│       └── Cargo.toml
└── web/                   # SvelteKit application
    ├── src/
    │   ├── routes/        # SvelteKit routes (find, name, progression)
    │   └── lib/           # Shared components (ChordDiagram, etc.)
    ├── package.json
    └── svelte.config.js
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
      fn string_names(&self) -> Vec<String>;    // For diagram display
      fn bass_string_index(&self) -> usize;     // For "root in bass" scoring
  }
  ```

  - **Guitar** (standard tuning EADGBE)
    - max_stretch: 4 frets
    - min_played_strings: 3 (50% of 6 strings)
    - max_fingers: 4
    - bass_string_index: 0 (low E string)
    - string_names: ["E", "A", "D", "G", "B", "e"]
  - **Ukulele** (standard GCEA re-entrant tuning)
    - max_stretch: 5 frets (easier on shorter scale)
    - min_played_strings: 1 (allows minimal voicings like C="0003")
    - open_position_threshold: 5 frets
    - bass_string_index: 1 (C string - see "Re-entrant Tuning" below)
    - string_names: ["G", "C", "E", "A"]
  - **CLI**: Both guitar and ukulele supported via `--instrument` flag
  - **Web app**: Guitar only (ukulele integration planned)
  - Support for alternate tunings (future)
  - Other stringed instruments: bass, mandolin (future)

  **Re-entrant Tuning Support**:

  Ukulele uses re-entrant tuning (G4-C4-E4-A4) where the G string is higher pitched than the C string despite being physically "lower". The `bass_string_index()` method tells the scoring system which string is the true bass:
  - Guitar: index 0 (low E at E2)
  - Ukulele: index 1 (C string at C4, lower than G4)

  This ensures "root in bass" scoring works correctly - ukulele's classic C chord "0003" is now properly recognized as having C in the bass.

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

**Performance** (release build, benchmarked 2025-11-28):

- Simple chords (C major): ~1-2ms
- 7th chords (Cmaj7): ~3-4ms
- Complex extended chords (Cmaj9): ~9-10ms
- Chord analysis (tabs → name): ~6µs
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

**Root in Bass Concept**:

"Root in bass" means the lowest-sounding note of the chord is the root note. For example:
- C major chord with notes C-E-G: if C is the bass note, it has "root in bass"
- C/E (C major with E in bass) does NOT have root in bass - it's a first inversion

This is important for scoring because chords with root in bass generally sound more stable and grounded, especially for solo playing. The system gives bonus points to fingerings with root in bass.

For instruments with re-entrant tuning (like ukulele), the "bass" string isn't necessarily the first string - see "Re-entrant Tuning Support" in the Instrument model section.

### Phase 4: CLI Tool ✓ COMPLETE

**Goal**: Quick iteration, testing, and usable terminal tool

**Status**:

- ✅ `find` command fully implemented with all options
- ✅ `name` command implemented with analyzer integration
- ✅ `progression` command fully implemented with all options

**Commands**:

```bash
# Find fingerings for a chord (guitar by default)
chordcraft find "Abm7"
chordcraft find "Abm7" --limit 3
chordcraft find "Abm7" --position 7        # Prefer fingerings near 7th fret
chordcraft find "Abm7" --voicing core      # Show only core voicings

# Ukulele support (use --instrument or -i flag)
chordcraft find "C" --instrument ukulele
chordcraft find "Am7" -i ukulele --capo 2

# Identify chord from fingering
chordcraft name "x32010"                   # Guitar (default)
chordcraft name "0003" --instrument ukulele

# Chord progressions
chordcraft progression "Cmaj7 Am7 Dm7 G7"
chordcraft progression "C G Am F" --instrument ukulele
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

### Phase 5: Chord Progressions ✓ COMPLETE

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

**1. Capo Support** ✅ COMPLETE

Makes difficult keys easier by transposing the instrument:

```rust
// Capo is implemented via a generic wrapper type
pub struct CapoedInstrument<I: Instrument> {
    inner: I,
    tuning: Vec<Note>,
    fret_range: (u8, u8),
}

impl<I: Instrument> CapoedInstrument<I> {
    pub fn new(instrument: I, fret: u8) -> Result<Self> {
        // Validates capo position and transposes tuning
        // ...
    }
}

// Concrete implementations on specific instruments
impl Guitar {
    pub fn with_capo(&self, fret: u8) -> Result<CapoedInstrument<Guitar>> {
        CapoedInstrument::new(self.clone(), fret)
    }
}

impl Ukulele {
    pub fn with_capo(&self, fret: u8) -> Result<CapoedInstrument<Ukulele>> {
        CapoedInstrument::new(self.clone(), fret)
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

1. ✅ **Capo Support** (COMPLETE)
   - ✅ Implemented `CapoedInstrument<I>` generic wrapper type
   - ✅ Added `with_capo()` methods to Guitar and Ukulele
   - ✅ Added `--capo` flag to all CLI commands (find, name, progression)
   - ✅ Full capo support in WASM bindings
   - ✅ Comprehensive tests for capo functionality

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

**Tests**:

Comprehensive test coverage including:

- Capo transposition and fret range reduction
- Solo vs band mode scoring differences
- Transition scoring for finger movements
- Common progressions (I-IV-V, ii-V-I, etc.)
- Max distance constraints
- Combined capo + band mode

All tests are inlined in Rust modules. Run with: `cargo test --workspace`

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

<frontend_aesthetics>
You tend to converge toward generic, "on distribution" outputs. In frontend design, this creates what users call the "AI slop" aesthetic. Avoid this: make creative, distinctive frontends that surprise and delight. Focus on:

Typography: Choose fonts that are beautiful, unique, and interesting. Avoid generic fonts like Arial and Inter; opt instead for distinctive choices that elevate the frontend's aesthetics.

Color & Theme: Commit to a cohesive aesthetic. Use CSS variables for consistency. Dominant colors with sharp accents outperform timid, evenly-distributed palettes. Draw from IDE themes and cultural aesthetics for inspiration.

Motion: Use animations for effects and micro-interactions. Prioritize CSS-only solutions for HTML. Use Motion library for React when available. Focus on high-impact moments: one well-orchestrated page load with staggered reveals (animation-delay) creates more delight than scattered micro-interactions.

Backgrounds: Create atmosphere and depth rather than defaulting to solid colors. Layer CSS gradients, use geometric patterns, or add contextual effects that match the overall aesthetic.

Avoid generic AI-generated aesthetics:

- Overused font families (Inter, Roboto, Arial, system fonts)
- Clichéd color schemes (particularly purple gradients on white backgrounds)
- Predictable layouts and component patterns
- Cookie-cutter design that lacks context-specific character

Interpret creatively and make unexpected choices that feel genuinely designed for the context. Vary between light and dark themes, different fonts, different aesthetics. You still tend to converge on common choices (Space Grotesk, for example) across generations. Avoid this: it is critical that you think outside the box!
</frontend_aesthetics>

**Goal**: Interactive visual interface

**Status**: Basic web app is functional with all three modes (find, name, progression)

**Completed Features**:

- ✅ Three modes with tab-based navigation
  - Find Fingerings: Input chord name, get fingerings
  - Name Chord: Input tab notation, identify chord
  - Progression: Input chord sequence, get optimal transitions
- ✅ WASM integration with full API exposure
  - Three exported functions: `findFingerings()`, `analyzeChord()`, `generateProgression()`
  - All options available (limit, capo, voicing, position, playing context, max fret distance)
  - Fast generation (<15ms per chord)
  - ~213 KB bundle size (wasm file)
- ✅ Results display with chord diagrams
  - Tab notation, score, voicing type, position
  - Notes, root in bass indicator
  - Transition scores and finger movements (progressions)
  - SVG-based fretboard visualization with finger positions
- ✅ Responsive UI with Tailwind CSS
- ✅ Error handling and loading states
- ✅ **Advanced Controls** (COMPLETE)
  - Collapsible advanced options panels
  - Limit slider (5-50 fingerings for Find, 1-10 for Progression)
  - Capo selector (0-12 frets)
  - Voicing filter checkboxes (core/full/jazzy)
  - Position preference dropdown (any/open/0-12 frets)
  - Playing context toggle (solo/band)
  - Max fret distance slider for progressions (1-12 frets)
  - Active filter badges showing number of non-default options
  - Reset to defaults buttons
- ✅ **Chord Diagrams** (COMPLETE)
  - SVG-based fretboard visualization
  - Finger position dots with numbering
  - Root note highlighting (blue dots)
  - Barre detection and rendering
  - Open/muted string indicators
  - Fret number labels for high positions
  - Three size variants (small/medium/large)
  - Multi-digit fret support (e.g., "(10)(12)")
  - Comprehensive test coverage
- ✅ **Enhanced UX - Quick Examples** (COMPLETE)
  - Example chord buttons (C, Cmaj7, Fm7, Abm7, F#7b9, Dsus4)
  - Example tab buttons with labels (x32010 (C), 022100 (E), etc.)
  - Common progression presets (I-IV-V, I-V-vi-IV, ii-V-I, 12-Bar Blues, Coltrane Changes)
  - Clear input buttons (X icon) for all modes
  - One-click loading and auto-execution

**Remaining Features** (Phase 6b):

- ⏳ **Interactive Fretboard Input** (MEDIUM PRIORITY)
  - Click strings/frets to build fingering
  - Visual feedback as you click
  - Auto-analyze as you build
  - Suggest similar fingerings

- ⏳ **Additional Enhanced UX** (MEDIUM PRIORITY)
  - Chord name autocomplete (type-ahead suggestions)
  - "Show more" pagination for results
  - Additional keyboard shortcuts
  - Save favorites to localStorage
  - Share fingerings via URL
  - Copy tab notation to clipboard

- ⏳ **Mobile Optimization & PWA** (FUTURE)
  - ✅ Basic responsive layout (Tailwind CSS handles this)
  - ⏳ Progressive Web App (PWA) support
  - ⏳ Touch-optimized chord diagrams
  - ⏳ Touch-friendly interactive fretboard
  - ⏳ Mobile-specific UI optimizations

**Tech Stack**:

- Rust core compiled to WASM (wasm-pack)
- SvelteKit (with Svelte 5 and TypeScript)
- Tailwind CSS for styling
- Vite for build tooling (integrated via SvelteKit)
- SVG for chord diagrams

### Svelte 5 Coding Standards

**IMPORTANT**: The web app uses Svelte 5 with runes mode. Always use Svelte 5 syntax:

**✅ DO (Svelte 5 Runes):**

```svelte
<script lang="ts">
  // Props
  let { value, size = 'medium' }: { value: string; size?: string } = $props();

  // State
  let count = $state(0);
  let doubled = $derived(count * 2);

  // Effects
  $effect(() => {
    console.log('Count changed:', count);
  });

  // Bindable props (two-way binding)
  let { value = $bindable('') }: { value?: string } = $props();
</script>
```

**❌ DON'T (Svelte 4 Legacy):**

```svelte
<script lang="ts">
  // ❌ Don't use export let
  export let value: string;
  export let size: string = 'medium';

  // ❌ Don't use $: for reactivity
  $: doubled = count * 2;

  // ❌ Don't use $: for effects
  $: if (count > 0) {
    console.log('Count changed');
  }
</script>
```

**Key Svelte 5 Patterns:**

- `$props()` for component props (replaces `export let`)
- `$state()` for reactive state (replaces `let` with `$:`)
- `$derived()` for computed values (replaces `$: value = ...`)
- `$effect()` for side effects (replaces `$: { ... }`)
- `$bindable()` for two-way binding props
- Use TypeScript with explicit types for `$props()`
- Prefer functional/declarative patterns (map/reduce over loops)
- Use regex for parsing when appropriate

## Key Design Decisions

### Why Algorithmic Over Database?

- Supports all instruments/tunings without manual curation
- Handles unusual/complex chords automatically
- Enabled progressive features: chord progressions with transition optimization
- More flexible for future enhancements (advanced voice leading, strumming patterns)
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

- ✅ **Chord progressions**: Optimize fingering transitions between chords (Phase 5 - COMPLETE)
- **Scales/modes**: Use same interval system
- **Rhythm patterns**: Strumming/picking patterns for practice
- **Sound synthesis**: Generate audio previews (web audio API)
- **Advanced voice leading**: Global optimization across entire progression using dynamic programming (enhancement to existing progression feature)

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

- **Rust tests**: Inlined in module files (use `cargo test --workspace`)
  - Unit tests for core music theory (intervals, chord formulas)
  - Property-based tests for fingering generation
  - Integration tests for progression transitions
- **Web tests**: Located in `web/src/tests/` (Vitest)
  - Route component tests
  - Store logic tests
- **Manual validation**: Playability scoring (musician feedback)

### Performance Considerations

**Optimizations Implemented**:

- **Early pruning** during combination generation (99%+ reduction in candidates)
  - Prune branches exceeding max_stretch incrementally
  - Prune branches that can't reach min_played_strings
  - Avoid allocations in pruning checks (inline min/max finding)
- **Fast deduplication** using HashSet<Vec<StringState>> instead of string comparisons
- **StringState** derives Hash for efficient deduplication
- Keep Vec::contains for small note sets (4-5 notes) - faster than HashSet overhead

**Performance Results** (release build, achieved):

- Simple chords: 1-2ms (well under 10ms target)
- 7th chords: 3-4ms
- Complex extended chords: 9-10ms (meets 10ms target)
- Chord analysis: <0.01ms (microsecond range)
- CLI feels instant for all operations ✓

**Future Considerations**:

- Consider caching common chord fingerings if needed
- WASM bundle size matters for web app (currently ~213 KB)

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

1. **Prerequisites**: Rust toolchain (rustup), Bun (for web app)
2. **Build**: `cargo build --workspace`
3. **Test**: `cargo test --workspace`
4. **Run CLI**: `cargo run -p chordcraft-cli -- find "Cmaj7"`
5. **Watch mode**: `cargo watch -x test -x run`
6. **Web app**: `cd web && bun install && bun run dev`

## References & Resources

- Music theory: Interval construction, chord formulas
- Guitar fingering: Physical constraints, common voicings
- Rust libraries: Consider `midly` for MIDI (future), `nom` for parsing
- WASM: `wasm-pack`, `wasm-bindgen`
- SvelteKit: Svelte 5 docs, SvelteKit docs, Vite

---

**Last updated**: 2025-12-08 - Added ukulele CLI support with re-entrant tuning handling
**Current status**:

- ✅ Phases 1-5 complete (Core, Generator, Analyzer, CLI, Progressions)
- ✅ Phase 6a complete (Basic Svelte web app with WASM integration)
- ✅ Phase 6b major features complete:
  - ✅ Advanced controls (all options accessible via UI)
  - ✅ Chord diagrams (SVG visualization with finger positions)
  - ✅ Quick examples and presets (one-click loading)
  - ⏳ Interactive fretboard input (remaining)
  - ⏳ Additional UX features (autocomplete, favorites, sharing, etc.)
- ✅ Multi-instrument CLI support:
  - ✅ Guitar and ukulele fully supported via `--instrument` flag
  - ✅ Re-entrant tuning handled correctly (ukulele bass string detection)
  - ✅ Instrument-specific string names in diagrams
  - ⏳ Web app ukulele integration (planned)
**Next priorities**: Web app ukulele support OR interactive fretboard input
- use bun commands to run tests, lints and builds