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

interface Barre {
	fret: number;
	fromString: number;
	toString: number;
}

/**
 * Auto-detect barres from current fingering
 *
 * Rules:
 * 1. If the lowest pressed fret touches both the first AND last played strings,
 *    it's a full barre spanning all strings (even those pressed on higher frets)
 * 2. Higher frets only get mini-barres if they have 2+ truly consecutive strings
 */
export function detectBarres(fingering: number[]): Barre[] {
	// Find played strings (fret > 0) and the lowest fret used
	const playedStrings = fingering
		.map((fret, idx) => ({ fret, idx }))
		.filter(({ fret }) => fret > 0);

	if (playedStrings.length < 2) return [];

	const firstPlayedString = Math.min(...playedStrings.map((s) => s.idx));
	const lastPlayedString = Math.max(...playedStrings.map((s) => s.idx));
	const lowestFret = Math.min(...playedStrings.map((s) => s.fret));

	// Check if lowest fret touches both first and last played strings
	const lowestFretStrings = playedStrings.filter((s) => s.fret === lowestFret).map((s) => s.idx);
	const isFullBarre =
		lowestFretStrings.includes(firstPlayedString) &&
		lowestFretStrings.includes(lastPlayedString);

	const barres: Barre[] = [];

	// Add full barre if detected
	if (isFullBarre) {
		barres.push({
			fret: lowestFret,
			fromString: firstPlayedString,
			toString: lastPlayedString,
		});
	}

	// Group remaining strings by fret (excluding the full barre fret if detected)
	const fretsToCheck = isFullBarre
		? playedStrings.filter((s) => s.fret !== lowestFret)
		: playedStrings;

	const fretGroups = fretsToCheck.reduce<Record<number, number[]>>((acc, { fret, idx }) => {
		(acc[fret] ||= []).push(idx);
		return acc;
	}, {});

	// For each remaining fret, find consecutive runs of 2+ strings (mini-barres)
	for (const [fretStr, strings] of Object.entries(fretGroups)) {
		if (strings.length < 2) continue;

		const fret = Number(fretStr);
		const sorted = strings.slice().sort((a, b) => a - b);

		// Find consecutive runs
		let runStart = sorted[0];
		let runEnd = sorted[0];

		for (let i = 1; i < sorted.length; i++) {
			if (sorted[i] === runEnd + 1) {
				// Continue run
				runEnd = sorted[i];
			} else {
				// End current run, check if valid barre
				if (runEnd - runStart + 1 >= 2) {
					barres.push({ fret, fromString: runStart, toString: runEnd });
				}
				// Start new run
				runStart = sorted[i];
				runEnd = sorted[i];
			}
		}
		// Don't forget the last run
		if (runEnd - runStart + 1 >= 2) {
			barres.push({ fret, fromString: runStart, toString: runEnd });
		}
	}

	return barres;
}
