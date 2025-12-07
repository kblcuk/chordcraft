import { describe, it, expect } from 'vitest';
import { detectBarres } from './tab-notation';

describe('Barre Detection', () => {
	it('should detect full barre for Am shape at 5th fret (577555)', () => {
		const fingering = [5, 7, 7, 5, 5, 5];
		const barres = detectBarres(fingering);

		// Full barre at fret 5 spanning all strings
		expect(barres).toContainEqual({ fret: 5, fromString: 0, toString: 5 });
		// Mini-barre at fret 7 for strings 1-2
		expect(barres).toContainEqual({ fret: 7, fromString: 1, toString: 2 });
		expect(barres).toHaveLength(2);
	});

	it('should detect full barre for Bm shape (224432)', () => {
		const fingering = [2, 2, 4, 4, 3, 2];
		const barres = detectBarres(fingering);

		// Full barre at fret 2 spanning all strings
		expect(barres).toContainEqual({ fret: 2, fromString: 0, toString: 5 });
		// Mini-barre at fret 4 for strings 2-3
		expect(barres).toContainEqual({ fret: 4, fromString: 2, toString: 3 });
		expect(barres).toHaveLength(2);
	});

	it('should detect full barre for D with barre shape (254232)', () => {
		const fingering = [2, 5, 4, 2, 3, 2];
		const barres = detectBarres(fingering);

		// Full barre at fret 2 spanning all strings
		expect(barres).toContainEqual({ fret: 2, fromString: 0, toString: 5 });
		// No other barres (no consecutive strings on other frets)
		expect(barres).toHaveLength(1);
	});

	it('should detect only mini-barre for Emaj shape (076444)', () => {
		const fingering = [0, 7, 6, 4, 4, 4];
		const barres = detectBarres(fingering);

		// No full barre (lowest fret 4 doesn't touch first played string 1)
		// Mini-barre at fret 4 for strings 3-4-5
		expect(barres).toContainEqual({ fret: 4, fromString: 3, toString: 5 });
		expect(barres).toHaveLength(1);
	});

	it('should detect full barre for C shape (x35553)', () => {
		const fingering = [-1, 3, 5, 5, 5, 3];
		const barres = detectBarres(fingering);

		// Full barre at fret 3 spanning strings 1-5
		expect(barres).toContainEqual({ fret: 3, fromString: 1, toString: 5 });
		// Mini-barre at fret 5 for strings 2-3-4
		expect(barres).toContainEqual({ fret: 5, fromString: 2, toString: 4 });
		expect(barres).toHaveLength(2);
	});

	it('should detect full barre for F major shape (133211)', () => {
		const fingering = [1, 3, 3, 2, 1, 1];
		const barres = detectBarres(fingering);

		// Full barre at fret 1 spanning all strings
		expect(barres).toContainEqual({ fret: 1, fromString: 0, toString: 5 });
		// Mini-barre at fret 3 for strings 1-2
		expect(barres).toContainEqual({ fret: 3, fromString: 1, toString: 2 });
		expect(barres).toHaveLength(2);
	});

	it('should not detect barre for open chords like E minor (022000)', () => {
		const fingering = [0, 2, 2, 0, 0, 0];
		const barres = detectBarres(fingering);

		// Fret 2 has strings 1-2, but first/last played are 0 and 5 (open)
		// So no full barre, and strings 1-2 on fret 2 form a mini-barre
		expect(barres).toContainEqual({ fret: 2, fromString: 1, toString: 2 });
		expect(barres).toHaveLength(1);
	});

	it('should not detect barre for open C major (x32010)', () => {
		const fingering = [-1, 3, 2, 0, 1, 0];
		const barres = detectBarres(fingering);

		// No consecutive strings on same fret
		expect(barres).toHaveLength(0);
	});

	it('should not detect barre for single string or less', () => {
		const fingering = [0, 0, 0, 2, 0, 0];
		const barres = detectBarres(fingering);
		expect(barres).toHaveLength(0);
	});

	it('should not detect barre for all open strings', () => {
		const fingering = [0, 0, 0, 0, 0, 0];
		const barres = detectBarres(fingering);
		expect(barres).toHaveLength(0);
	});

	it('should handle partial muting with barre (x24442)', () => {
		const fingering = [-1, 2, 4, 4, 4, 2];
		const barres = detectBarres(fingering);

		// Full barre at fret 2 spanning strings 1-5
		expect(barres).toContainEqual({ fret: 2, fromString: 1, toString: 5 });
		// Mini-barre at fret 4 for strings 2-3-4
		expect(barres).toContainEqual({ fret: 4, fromString: 2, toString: 4 });
		expect(barres).toHaveLength(2);
	});

	it('should handle power chord shape (x244xx)', () => {
		const fingering = [-1, 2, 4, 4, -1, -1];
		const barres = detectBarres(fingering);

		// Fret 2 is on string 1 only (first played)
		// Fret 4 is on strings 2-3 (last played is 3)
		// Lowest fret 2 touches string 1 (first) but not string 3 (last)
		// So no full barre, just mini-barre at fret 4
		expect(barres).toContainEqual({ fret: 4, fromString: 2, toString: 3 });
		expect(barres).toHaveLength(1);
	});

	it('should detect barre spanning middle strings only (x13331)', () => {
		const fingering = [-1, 1, 3, 3, 3, 1];
		const barres = detectBarres(fingering);

		// Full barre at fret 1 spanning strings 1-5
		expect(barres).toContainEqual({ fret: 1, fromString: 1, toString: 5 });
		// Mini-barre at fret 3 for strings 2-3-4
		expect(barres).toContainEqual({ fret: 3, fromString: 2, toString: 4 });
		expect(barres).toHaveLength(2);
	});
});
