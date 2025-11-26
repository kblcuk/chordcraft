# ChordCraft

A multi-platform tool for bidirectional chord-fingering conversion, supporting guitar and other stringed instruments.

## Features

- **Chord → Fingering**: Input a chord name (e.g., "Abm7") and get multiple fingering options
- **Fingering → Chord**: Input tab notation (e.g., "x32010") and identify the chord
- **Multiple voicing types**: Core, Full, and Jazzy voicings for different playing contexts
- **Position-aware**: Find fingerings near a specific fret position
- **Multi-instrument**: Designed to support guitar, bass, ukulele, and more

## Project Status

✅ **Functional** - Core features implemented (CLI + Web App)

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/chordcraft.git
cd chordcraft

# Build the project
cargo build --release

# Run the CLI tool
cargo run -p chordcraft-cli -- find "Cmaj7"
```

## Usage

```bash
# Find fingerings for a chord
chordcraft find "Abm7"
chordcraft find "Abm7" --limit 3
chordcraft find "Abm7" --position 7        # Prefer fingerings near 7th fret
chordcraft find "Abm7" --capo 3            # With capo on 3rd fret
chordcraft find "Abm7" --context band      # For band playing (lighter voicings)

# Identify chord from fingering
chordcraft name "x32010"
chordcraft name "022100"

# Chord progressions (optimize transitions)
chordcraft progression "Cmaj7 Am7 Dm7 G7"
chordcraft progression "I-IV-V" --limit 5 --max-distance 3
```

## Development

See [CLAUDE.md](./CLAUDE.md) for detailed implementation plan and architecture decisions.

```bash
# Run tests
cargo test --workspace

# Run with watch mode (requires cargo-watch)
cargo watch -x test -x run

# Build all crates
cargo build --workspace
```

## Project Structure

- `crates/core` - Core music theory library (Rust)
- `crates/cli` - Command-line interface tool
- `crates/wasm` - WebAssembly bindings for web
- `web/` - SvelteKit web application

## Contributing

This project is in early development. Contributions welcome!

## License

MIT OR Apache-2.0
