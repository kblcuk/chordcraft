# ChordCraft

A multi-platform tool for bidirectional chord-fingering conversion, supporting guitar and other stringed instruments.

## Features

- **Chord → Fingering**: Input a chord name (e.g., "Abm7") and get multiple fingering options
- **Fingering → Chord**: Input tab notation (e.g., "x32010") and identify the chord
- **Multiple voicing types**: Core, Full, and Jazzy voicings for different playing contexts
- **Position-aware**: Find fingerings near a specific fret position
- **Multi-instrument**: Designed to support guitar, bass, ukulele, and more

## Project Status

🚧 **Early Development** - Core music theory implementation in progress

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

# Identify chord from fingering (coming soon)
chordcraft name "x32010"
chordcraft name "x32010" --key C           # With context hint
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
- `crates/wasm` - WebAssembly bindings (future)
- `web/` - Vue web application (future)

## Contributing

This project is in early development. Contributions welcome!

## License

MIT OR Apache-2.0
