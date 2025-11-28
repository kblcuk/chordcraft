/**
 * Shared constants for fretboard diagrams
 * Used by both display and interactive chord diagrams
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
// Colors (Tailwind mappings for SVG)
// ============================================================================

export const COLORS = {
	string: '#1f2937', // gray-800
	fret: '#1f2937', // gray-800
	nut: '#111827', // gray-900
	fingerDot: '#1f2937', // gray-800
	rootDot: '#2563eb', // blue-600
	openString: '#1f2937', // gray-800
	rootOpenString: '#2563eb', // blue-600
	barre: '#4b5563', // gray-600
	mutedString: '#9ca3af', // gray-400
	fretNumber: '#6b7280', // gray-500
	hoverDot: '#3b82f6', // blue-500 (for interactive mode)
	selectedDot: '#10b981', // green-500 (for interactive mode)
} as const;
