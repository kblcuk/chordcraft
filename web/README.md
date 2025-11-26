# ChordCraft Web App

Interactive web application for chord-fingering conversion, built with SvelteKit and Rust WASM.

## Features

- 🎸 **Find Fingerings**: Input chord name, get multiple fingering options with diagrams
- 🎯 **Name Chord**: Input tab notation, identify the chord
- 🎵 **Progressions**: Input chord sequence, get optimal transition fingerings
- ⚙️ **Advanced Options**: Capo, voicing filters, position preferences, playing context (solo/band)
- 📊 **Visual Diagrams**: SVG-based fretboard visualization with finger positions
- 🌙 **Dark Mode**: Full dark mode support with seamless theme switching

## Tech Stack

- **SvelteKit** - Full-stack framework with Svelte 5
- **TypeScript** - Type-safe development
- **Tailwind CSS** - Utility-first styling with semantic color system
- **mode-watcher** - Automatic dark mode detection and management
- **Rust WASM** - High-performance chord generation (via `chordcraft-wasm`)
- **Vite** - Fast build tooling (integrated with SvelteKit)
- **Vitest** - Unit testing framework

### Color System

The app uses a semantic color system that automatically adapts to dark mode:

- `bg-background` - Main background
- `bg-card` - Card/panel backgrounds
- `bg-muted` - Muted backgrounds (inputs, secondary elements)
- `text-foreground` - Primary text
- `text-muted-foreground` - Secondary text
- `border-border` - Borders and dividers

These semantic colors are defined in `src/app.css` with CSS custom properties that change based on the `.dark` class.

## Development

### Prerequisites

- Node.js 18+ (for pnpm/npm)
- pnpm (recommended) or npm
- Rust toolchain with `wasm-pack` (to build WASM module)

### Setup

```bash
# Install dependencies
pnpm install

# Build WASM module (from workspace root)
cd ../crates/wasm
wasm-pack build --target web

# Back to web directory
cd ../../web

# Start dev server
pnpm dev
```

The app will be available at `http://localhost:5173`

### Available Scripts

```bash
# Development
pnpm dev              # Start dev server with HMR
pnpm build            # Build for production
pnpm preview          # Preview production build

# Testing
pnpm test             # Run tests in watch mode
pnpm test:run         # Run tests once (CI)
pnpm test:ui          # Open Vitest UI
pnpm test:coverage    # Generate coverage report

# Code Quality
pnpm check            # Run svelte-check and TypeScript
pnpm lint             # Run ESLint
pnpm lint:fix         # Fix ESLint errors
pnpm format           # Format with Prettier
pnpm format:check     # Check formatting
```

## Project Structure

```
web/
├── src/
│   ├── routes/              # SvelteKit routes
│   │   ├── +layout.svelte   # Root layout
│   │   ├── +page.svelte     # Home page
│   │   ├── find/            # Find fingerings route
│   │   ├── name/            # Name chord route
│   │   └── progression/     # Chord progressions route
│   ├── lib/                 # Shared components & utilities
│   │   ├── ChordDiagram.svelte      # Fretboard visualization
│   │   ├── ChordDiagram.test.ts     # Component tests
│   │   └── ...
│   └── app.html             # HTML template
├── static/                  # Static assets
├── tests/                   # E2E tests (future)
├── package.json
├── svelte.config.js         # SvelteKit config
├── vite.config.ts           # Vite config (WASM plugins)
├── vitest.config.ts         # Vitest config
├── tailwind.config.js       # Tailwind config
└── tsconfig.json            # TypeScript config
```

## Key Components

### ChordDiagram

SVG-based fretboard visualization component.

**Props:**

- `tab` - Tab notation (e.g., "x32010")
- `notes` - Array of note names (e.g., ["C", "E", "G"])
- `rootNote` - Root note for highlighting (e.g., "C")
- `size` - "small" | "medium" | "large" (default: "medium")

**Features:**

- Finger position dots with numbering
- Root note highlighting (blue dots)
- Barre detection and rendering
- Open/muted string indicators
- Fret number labels for high positions
- Multi-digit fret support (e.g., "(10)(12)")

**Usage:**

```svelte
<ChordDiagram tab="x32010" notes={['C', 'E', 'G']} rootNote="C" size="medium" />
```

## WASM Integration

The web app uses the `chordcraft-wasm` package, which exposes the Rust core library to JavaScript.

**Key Functions:**

- `findFingerings(chord, options)` - Generate fingerings for a chord
- `nameChord(tab)` - Identify chord from tab notation
- `generateProgression(chords, options)` - Optimize chord progression transitions

**Options:**

- `limit` - Max fingerings to return (default: 20)
- `capo` - Capo fret (0-12)
- `voicingFilter` - Array of ["core", "full", "jazzy"]
- `position` - Preferred fret position (0-24)
- `playingContext` - "solo" | "band"
- `maxFretDistance` - Max distance between progression fingerings (1-12)

## Testing

We follow a **user-centric testing approach**: test what users see and interact with, not implementation details.

See [TESTING.md](./TESTING.md) for detailed testing philosophy and guidelines.

**Current Coverage:**

- ✅ ChordDiagram component (22 tests)
- ✅ Tab parsing (various formats)
- ✅ Visual elements rendering
- ✅ Size variants
- ⏳ Route components (future)
- ⏳ E2E tests (future)

## Deployment

The app is configured for static site generation (SSG) using `@sveltejs/adapter-static`.

```bash
# Build for production
pnpm build

# Output will be in build/ directory
# Deploy build/ to any static host (Vercel, Netlify, GitHub Pages, etc.)
```

## Known Limitations

### Testing `bind:value` with `<select>`

Svelte's two-way binding for select elements doesn't sync properly in testing environments (happy-dom/jsdom). We verify UI elements render correctly, and rely on WASM unit tests for parameter passing logic.

See [TESTING_NOTES.md](./TESTING_NOTES.md) for detailed explanation.

## Future Enhancements

- Interactive fretboard input (click to build fingering)
- Chord name autocomplete
- Save favorites to localStorage
- Share fingerings via URL
- Copy tab notation to clipboard
- Mobile optimizations (touch-friendly diagrams)
- E2E tests with Playwright

## Contributing

See [../CLAUDE.md](../CLAUDE.md) for architecture overview and implementation details.

## License

MIT OR Apache-2.0
