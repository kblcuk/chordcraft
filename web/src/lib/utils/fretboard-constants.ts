/**
 * Shared constants for fretboard diagrams
 * Used by both display and interactive chord diagrams
 *
 * Workshop Warmth Design System
 * Colors are defined in app.css as CSS custom properties
 */

// ============================================================================
// Layout Constants
// ============================================================================

export const STRING_COUNT = 6;
export const VISIBLE_FRETS = 5;
export const MARGIN_BOTTOM = 20;
export const MARGIN_SIDE = 25;

// ============================================================================
// Size Variants
// ============================================================================

export const DIMENSIONS = {
	small: { width: 120, height: 160, dotRadius: 6, marginTop: 30 },
	medium: { width: 160, height: 200, dotRadius: 8, marginTop: 35 },
	large: { width: 200, height: 250, dotRadius: 10, marginTop: 40 },
} as const;

export type DiagramSize = keyof typeof DIMENSIONS;

// ============================================================================
// Colors - CSS Variable References
// These reference CSS custom properties defined in app.css
// Use with style attributes: style="fill: {COLORS.string}"
// ============================================================================

export const COLORS = {
	// Fretboard structure - warm wood tones
	string: 'var(--diagram-string)',
	fret: 'var(--diagram-fret)',
	nut: 'var(--diagram-nut)',

	// Finger positions - deep amber
	fingerDot: 'var(--diagram-dot)',
	rootDot: 'var(--diagram-dot-root)',

	// Open strings
	openString: 'var(--diagram-string)',
	rootOpenString: 'var(--diagram-dot-root)',

	// Barre - subtle shadow
	barre: 'var(--diagram-barre)',

	// Muted string - subdued
	mutedString: 'var(--diagram-muted)',

	// Fret number label
	fretNumber: 'var(--diagram-fret-number)',

	// Dot text (finger numbers)
	dotText: 'var(--diagram-dot-text)',

	// Interactive states (for interactive mode)
	hoverDot: 'var(--diagram-hover)',
	selectedDot: 'var(--diagram-selected)',

	// Background colors for SVG
	background: 'var(--diagram-bg)',
	backgroundGradient: 'var(--diagram-bg-gradient)',
	woodGrain: 'var(--diagram-wood-grain)',
} as const;
