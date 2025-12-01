/**
 * Tab Notation Utilities
 *
 * Functions for parsing and generating guitar tab notation,
 * and transposing fingerings between positions.
 */

const STRING_COUNT = 6;

/**
 * Parse tab notation to fret positions
 * Supports: x (muted), 0-9 (single digit), (10) (multi-digit in parens)
 * Example: "x32010" → [-1, 3, 2, 0, 1, 0]
 * Example: "x(10)(12)9(11)x" → [-1, 10, 12, 9, 11, -1]
 */
export function parseTabNotation(tab: string, stringCount = STRING_COUNT): number[] {
	// Match: x/X (muted), (digits) (multi-digit), or single digit
	// Separators and invalid chars are automatically ignored
	const matches = tab.matchAll(/x|X|\((\d+)\)|\d/gi);

	return (
		Array.from(matches) // Ensure at least stringCount matches
			// Map each match to a fret number
			.map((match) => {
				const value = match[0].toLowerCase();

				// Muted string
				if (value === 'x') return -1;

				// Multi-digit in parentheses (capture group 1)
				if (match[1]) return parseInt(match[1], 10);

				// Single digit
				return parseInt(value, 10);
			})
			// Filter out any NaN (shouldn't happen, but defensive)
			.filter((fret) => !isNaN(fret))
			// Limit to X strings
			.slice(0, stringCount)
	);
}

/**
 * Convert fingering array to tab notation string
 * Handles capo transposition if needed
 * @param fingering Array where -2/-1=muted, 0=open, 1+=fretted
 * @param capo Capo position (0-12)
 * @returns Tab notation string
 */
export function generateTabNotation(fingering: number[], capo: number): string {
	return fingering
		.map((fret) => {
			if (fret === -2 || fret === -1) return 'x'; // Both not-set and muted -> 'x'

			// Apply capo transposition: player fret + capo = actual sounding fret
			// Note: Even open strings (0) are transposed when capo is applied
			const actualFret = fret + capo;
			return actualFret > 9 ? `(${actualFret})` : String(actualFret);
		})
		.join('');
}

/**
 * Transpose fingering to a new position
 * @param fingering Current fingering array
 * @param oldPos Previous start fret position
 * @param newPos New start fret position
 * @returns Transposed fingering array
 */
export function transposeFingeringToNewPosition(
	fingering: number[],
	oldPos: number,
	newPos: number
): number[] {
	const delta = newPos - oldPos;
	if (delta === 0) return fingering;

	return fingering.map((fret) => {
		// Keep special values unchanged
		if (fret === -2 || fret === -1) return fret;

		// Transpose all other values
		const transposed = fret + delta;

		// Clamp to valid range [0, 24]
		return Math.max(0, Math.min(24, transposed));
	});
}
