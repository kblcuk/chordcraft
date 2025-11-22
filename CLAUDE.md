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

### Phase 1: Core Music Theory ✓ CURRENT PHASE

**Goal**: Foundation for representing musical concepts

- [ ] **Note representation** (`note.rs`)
  - Pitch classes (C, C#, D, etc.)
  - Enharmonic equivalents (C# = Db)
  - Octave-aware representation
  - Semitone calculations

- [ ] **Interval system** (`interval.rs`)
  - Perfect, Major, Minor, Augmented, Diminished
  - Interval calculation between notes
  - Interval arithmetic (stack intervals)

- [ ] **Chord formulas** (`chord.rs`)
  - Chord type definitions with interval patterns
  - Basic triads: Major [R, M3, P5], Minor [R, m3, P5], Dim, Aug
  - 7th chords: maj7, min7, dom7, min7b5, dim7
  - Extended chords: 9ths, 11ths, 13ths
  - Altered chords: sus2, sus4, add9, 7b9, 7#9, etc.
  - Chord name parser: "Abm7b5" → structured representation

- [ ] **Instrument model** (`instrument.rs`)

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

- [ ] **Fingering representation** (`fingering.rs`)
  - Tab notation format (e.g., "x32010")
  - Fret positions per string
  - Physical validation (stretch, muted strings)

### Phase 2: Fingering Generation (Chord → Tabs)

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

### Phase 3: Reverse Lookup (Tabs → Chord)

**Goal**: Given fingering notation, identify the chord

**Algorithm** (`analyzer.rs`):

1. Parse tab notation (e.g., "x32010")
2. Calculate which notes are being played
3. Determine intervals relative to each possible root
4. Match interval patterns against chord formulas
5. Rank matches by:
   - Key guess from first letter of notes present
   - Commonality (C major more likely than B# major)
   - Completeness (all chord tones present)
6. Return primary match + alternatives ("Could also be...")

### Phase 4: CLI Tool

**Goal**: Quick iteration, testing, and usable terminal tool

**Commands**:

```bash
# Find fingerings for a chord
chordcraft find "Abm7"
chordcraft find "Abm7" --limit 3
chordcraft find "Abm7" --position 7        # Prefer fingerings near 7th fret
chordcraft find "Abm7" --voicing core      # Show only core voicings

# Identify chord from fingering
chordcraft name "x32010"
chordcraft name "x32010" --key C           # Context hint

# Future: chord progressions
chordcraft progression "Cmaj7 Am7 Dm7 G7" --optimize-transitions
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

### Phase 5: Web App (Vue + Rust WASM)

**Goal**: Interactive visual interface

**Features**:

- Interactive fretboard visualization
- Input modes: click fretboard OR type chord name
- Real-time suggestions as you type
- Voicing filter (core/full/jazzy)
- Position preference slider
- Instrument/tuning selector
- Save favorites (localStorage initially)

**Tech stack**:

- Rust core compiled to WASM (wasm-pack)
- Vue 3 with Composition API
- SVG-based fretboard component
- Vite for build tooling

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

- **Chord progressions**: Optimize fingering transitions between chords
- **Voice leading**: Suggest fingerings with minimal movement
- **Scales/modes**: Use same interval system
- **Rhythm patterns**: Strumming/picking patterns for practice
- **Sound synthesis**: Generate audio previews (web audio API)

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

**Last updated**: Initial creation - Phase 1 in progress
**Current focus**: Core music theory types (Note, Interval, Chord)
