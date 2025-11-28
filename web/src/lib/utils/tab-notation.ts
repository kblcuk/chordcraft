/**
 * Tab Notation Utilities
 *
 * Functions for parsing and generating guitar tab notation,
 * and transposing fingerings between positions.
 */

const STRING_COUNT = 6;

/**
 * Parse tab notation string into fingering array
 * Handles formats: "x32010", "5(10)7xx", etc.
 * @param tab Tab notation string
 * @returns Fingering array where -1=muted, 0=open, 1-24=fretted
 */
export function parseTabNotation(tab: string): number[] {
	if (!tab || tab.trim() === '') {
		return Array(STRING_COUNT).fill(-1); // Empty = all muted
	}

	const result: number[] = [];
	let i = 0;

	while (i < tab.length && result.length < STRING_COUNT) {
		// Check for multi-digit fret in parentheses: (10), (12), etc.
		if (tab[i] === '(') {
			const closeIdx = tab.indexOf(')', i);
			if (closeIdx > i) {
				const fretStr = tab.substring(i + 1, closeIdx);
				const fret = parseInt(fretStr, 10);
				if (!isNaN(fret) && fret >= 0 && fret <= 24) {
					result.push(fret);
				} else {
					result.push(-1); // Invalid -> muted
				}
				i = closeIdx + 1;
				continue;
			}
		}

		// Check for single character
		const char = tab[i];
		if (char === 'x' || char === 'X') {
			result.push(-1); // Muted
		} else if (char === '0') {
			result.push(0); // Open
		} else if (char >= '1' && char <= '9') {
			result.push(parseInt(char, 10));
		}
		// Skip any other characters (spaces, etc.)

		i++;
	}

	// Pad with muted strings if we have fewer than STRING_COUNT
	while (result.length < STRING_COUNT) {
		result.push(-1);
	}

	return result.slice(0, STRING_COUNT);
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
	newPos: number,
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

/**
 * Helper: Check if two arrays are equal
 */
export function arraysEqual(a: number[], b: number[]): boolean {
	if (a.length !== b.length) return false;
	return a.every((val, i) => val === b[i]);
}
