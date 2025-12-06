/**
 * Interactive Chord Diagram - Bidirectional Sync Tests
 *
 * Tests the new features:
 * 1. Tab notation parsing (text → visual)
 * 2. Tab notation generation (visual → text)
 * 3. Position slider transposition
 * 4. Bidirectional synchronization
 */

import { describe, it, expect } from 'vitest';
import {
	parseTabNotation,
	generateTabNotation,
	transposeFingeringToNewPosition,
} from '$lib/utils/tab-notation';

describe('InteractiveChordDiagram - Bidirectional Sync', () => {
	describe('parseTabNotation', () => {
		it('should parse simple tab notation with muted strings', () => {
			const result = parseTabNotation('x32010');
			expect(result).toEqual([-1, 3, 2, 0, 1, 0]);
		});

		it('should parse all open strings', () => {
			const result = parseTabNotation('000000');
			expect(result).toEqual([0, 0, 0, 0, 0, 0]);
		});

		it('should parse all muted strings', () => {
			const result = parseTabNotation('xxxxxx');
			expect(result).toEqual([-1, -1, -1, -1, -1, -1]);
		});

		it('should parse single-digit frets', () => {
			const result = parseTabNotation('335553');
			expect(result).toEqual([3, 3, 5, 5, 5, 3]);
		});

		it('should parse multi-digit frets with parentheses', () => {
			const result = parseTabNotation('5(10)7xx(12)');
			expect(result).toEqual([5, 10, 7, -1, -1, 12]);
		});

		it('should parse mixed single and multi-digit frets', () => {
			const result = parseTabNotation('x(10)(12)(12)(10)0');
			expect(result).toEqual([-1, 10, 12, 12, 10, 0]);
		});

		it('should handle empty string as empty array', () => {
			const result = parseTabNotation('');
			expect(result).toEqual([]);
		});

		it('should handle partial tab notation (pad with muted)', () => {
			const result = parseTabNotation('x32');
			expect(result).toEqual([-1, 3, 2]);
		});

		it('should handle E minor shape (022000)', () => {
			const result = parseTabNotation('022000');
			expect(result).toEqual([0, 2, 2, 0, 0, 0]);
		});

		it('should handle F barre chord shape (133211)', () => {
			const result = parseTabNotation('133211');
			expect(result).toEqual([1, 3, 3, 2, 1, 1]);
		});
	});

	describe('generateTabNotation', () => {
		it('should generate tab for C major (x32010)', () => {
			const result = generateTabNotation([-1, 3, 2, 0, 1, 0], 0);
			expect(result).toBe('x32010');
		});

		it('should generate tab for all open strings', () => {
			const result = generateTabNotation([0, 0, 0, 0, 0, 0], 0);
			expect(result).toBe('000000');
		});

		it('should treat both -1 and -2 as muted (x)', () => {
			const result1 = generateTabNotation([-1, -1, -1, -1, -1, -1], 0);
			const result2 = generateTabNotation([-2, -2, -2, -2, -2, -2], 0);
			expect(result1).toBe('xxxxxx');
			expect(result2).toBe('xxxxxx');
		});

		it('should apply capo transposition', () => {
			// E major shape (022100) with capo 3 = G major
			const result = generateTabNotation([0, 2, 2, 1, 0, 0], 3);
			expect(result).toBe('355433');
		});

		it('should use parentheses for frets >= 10', () => {
			const result = generateTabNotation([5, 10, 12, 12, 10, 0], 0);
			expect(result).toBe('5(10)(12)(12)(10)0');
		});

		it('should handle capo causing multi-digit frets', () => {
			// E shape (022100) + capo 10 = all frets transposed by 10
			const result = generateTabNotation([0, 2, 2, 1, 0, 0], 10);
			expect(result).toBe('(10)(12)(12)(11)(10)(10)');
		});

		it('should handle mixed -1 and -2 consistently', () => {
			const result = generateTabNotation([-2, 3, -1, 0, 1, -2], 0);
			expect(result).toBe('x3x01x');
		});
	});

	describe('generateTabNotation and parseTabNotation roundtrip', () => {
		it('should be inverse operations (parse → generate)', () => {
			const original = 'x32010';
			const parsed = parseTabNotation(original);
			const generated = generateTabNotation(parsed, 0);
			expect(generated).toBe(original);
		});

		it('should normalize -2 to -1 after roundtrip', () => {
			const fingering = [-2, 3, -2, 0, 1, -2];
			const generated = generateTabNotation(fingering, 0);
			expect(generated).toBe('x3x01x');

			const parsed = parseTabNotation(generated);
			expect(parsed).toEqual([-1, 3, -1, 0, 1, -1]); // -2 becomes -1
		});

		it('should handle multi-digit roundtrip', () => {
			const original = '5(10)(12)xx0';
			const parsed = parseTabNotation(original);
			const generated = generateTabNotation(parsed, 0);
			expect(generated).toBe(original);
		});

		it('should handle capo in roundtrip', () => {
			const fingering = [0, 2, 2, 1, 0, 0];
			const withCapo = generateTabNotation(fingering, 5);
			expect(withCapo).toBe('577655');

			// Parse back and apply inverse capo
			const parsed = parseTabNotation(withCapo);
			const withoutCapo = generateTabNotation(
				parsed.map((f) => (f > 0 ? f - 5 : f)),
				0
			);
			expect(withoutCapo).toBe('022100');
		});
	});

	describe('transposeFingeringToNewPosition', () => {
		it('should transpose all fret values when moving from position 0 to 5', () => {
			const fingering = [0, 0, 2, 2, 1, 0]; // E minor shape
			const result = transposeFingeringToNewPosition(fingering, 0, 5);
			expect(result).toEqual([5, 5, 7, 7, 6, 5]);
		});

		it('should transpose down when moving to lower position', () => {
			const fingering = [5, 5, 7, 7, 6, 5];
			const result = transposeFingeringToNewPosition(fingering, 5, 0);
			expect(result).toEqual([0, 0, 2, 2, 1, 0]);
		});

		it('should keep muted strings (-1) unchanged', () => {
			const fingering = [-1, 3, 2, 0, 1, 0]; // C major
			const result = transposeFingeringToNewPosition(fingering, 0, 5);
			expect(result).toEqual([-1, 8, 7, 5, 6, 5]);
		});

		it('should keep not-set strings (-2) unchanged', () => {
			const fingering = [-2, 3, -2, 0, 1, -2];
			const result = transposeFingeringToNewPosition(fingering, 0, 3);
			expect(result).toEqual([-2, 6, -2, 3, 4, -2]);
		});

		it('should handle no change when delta is 0', () => {
			const fingering = [0, 2, 2, 1, 0, 0];
			const result = transposeFingeringToNewPosition(fingering, 3, 3);
			expect(result).toEqual([0, 2, 2, 1, 0, 0]);
		});

		it('should transpose F barre chord up 5 frets', () => {
			const fingering = [1, 3, 3, 2, 1, 1]; // F major barre
			const result = transposeFingeringToNewPosition(fingering, 0, 5);
			expect(result).toEqual([6, 8, 8, 7, 6, 6]); // Bb major barre
		});

		it('should handle negative deltas correctly', () => {
			const fingering = [7, 9, 9, 8, 7, 7]; // D major barre at 7
			const result = transposeFingeringToNewPosition(fingering, 5, 2);
			expect(result).toEqual([4, 6, 6, 5, 4, 4]); // -3 delta
		});

		it('should not create invalid fret numbers (<0)', () => {
			const fingering = [2, 3, 3, 2, 2, 2];
			const result = transposeFingeringToNewPosition(fingering, 2, 0);
			// All values should be >= 0
			expect(result.every((f) => f === -1 || f === -2 || f >= 0)).toBe(true);
			expect(result).toEqual([0, 1, 1, 0, 0, 0]);
		});

		it('should not create fret numbers >24', () => {
			const fingering = [20, 22, 22, 21, 20, 20];
			const result = transposeFingeringToNewPosition(fingering, 18, 20);
			// All values should be <= 24 (or special values)
			expect(result.every((f) => f === -1 || f === -2 || f <= 24)).toBe(true);
			expect(result).toEqual([22, 24, 24, 23, 22, 22]);
		});
	});

	describe('Bidirectional Sync Integration', () => {
		it('should converge after text → visual → text cycle', () => {
			const textInput = 'x32010';

			// Step 1: Parse text into visual
			const fingering1 = parseTabNotation(textInput);
			expect(fingering1).toEqual([-1, 3, 2, 0, 1, 0]);

			// Step 2: Generate text from visual
			const textOutput = generateTabNotation(fingering1, 0);
			expect(textOutput).toBe('x32010');

			// Step 3: Parse again - should be stable
			const fingering2 = parseTabNotation(textOutput);
			expect(fingering2).toEqual(fingering1);

			// Convergence: text hasn't changed
			expect(textOutput).toBe(textInput);
		});

		it('should converge after visual → text → visual cycle', () => {
			const initialFingering = [-2, 3, -2, 0, 1, -2];

			// Step 1: Generate text from visual
			const text1 = generateTabNotation(initialFingering, 0);
			expect(text1).toBe('x3x01x');

			// Step 2: Parse text back to visual
			const fingering1 = parseTabNotation(text1);
			expect(fingering1).toEqual([-1, 3, -1, 0, 1, -1]); // -2 normalized to -1

			// Step 3: Generate again - should be stable
			const text2 = generateTabNotation(fingering1, 0);
			expect(text2).toBe('x3x01x');

			// Convergence: text is stable
			expect(text2).toBe(text1);
		});

		it('should handle rapid alternation between text and visual input', () => {
			// Start with text
			let text = '000000';
			let fingering = parseTabNotation(text);

			// User clicks visual (change one string)
			fingering = [0, 0, 2, 0, 0, 0];
			text = generateTabNotation(fingering, 0);
			expect(text).toBe('002000');

			// User types in text
			text = '002200';
			fingering = parseTabNotation(text);
			expect(fingering).toEqual([0, 0, 2, 2, 0, 0]);

			// User clicks visual again
			fingering = [0, 0, 2, 2, 1, 0];
			text = generateTabNotation(fingering, 0);
			expect(text).toBe('002210');

			// Final state should be consistent
			const finalFingering = parseTabNotation(text);
			const finalText = generateTabNotation(finalFingering, 0);
			expect(finalText).toBe('002210');
			expect(finalFingering).toEqual([0, 0, 2, 2, 1, 0]);
		});
	});

	describe('Position Slider + Sync Integration', () => {
		it('should update tab notation when position changes', () => {
			// Start at position 0 with E minor shape
			const fingering = [0, 2, 2, 0, 0, 0];
			let text = generateTabNotation(fingering, 0);
			expect(text).toBe('022000');

			// Move to position 5
			const transposed = transposeFingeringToNewPosition(fingering, 0, 5);
			text = generateTabNotation(transposed, 0);
			expect(text).toBe('577555');
		});

		it('should maintain sync after position change and text edit', () => {
			// Position 0, C major shape
			let fingering = [-1, 3, 2, 0, 1, 0];
			let position = 0;

			// Move to position 3
			fingering = transposeFingeringToNewPosition(fingering, position, 3);
			position = 3;
			let text = generateTabNotation(fingering, 0);
			expect(text).toBe('x65343');

			// User types new tab
			text = 'x65340';
			fingering = parseTabNotation(text);
			expect(fingering).toEqual([-1, 6, 5, 3, 4, 0]);

			// Generate back - should match
			const finalText = generateTabNotation(fingering, 0);
			expect(finalText).toBe('x65340');
		});

		it('should handle capo + position change together', () => {
			// Position 0, E shape, capo 3
			const fingering = [0, 2, 2, 1, 0, 0];
			const capo = 3;

			let text = generateTabNotation(fingering, capo);
			expect(text).toBe('355433');

			// Move to position 5
			const transposed = transposeFingeringToNewPosition(fingering, 0, 5);
			text = generateTabNotation(transposed, capo);
			expect(text).toBe('8(10)(10)988');
		});
	});

	describe('Edge Cases', () => {
		it('should handle all strings muted after clear', () => {
			const fingering = [-2, -2, -2, -2, -2, -2];
			const text = generateTabNotation(fingering, 0);
			expect(text).toBe('xxxxxx');

			const parsed = parseTabNotation(text);
			expect(parsed).toEqual([-1, -1, -1, -1, -1, -1]);
		});

		it('should handle very high capo positions', () => {
			const fingering = [0, 2, 2, 1, 0, 0];
			const text = generateTabNotation(fingering, 12);
			expect(text).toBe('(12)(14)(14)(13)(12)(12)');
		});

		it('should handle position at maximum (19)', () => {
			const fingering = [0, 2, 2, 1, 0, 0];
			const transposed = transposeFingeringToNewPosition(fingering, 0, 19);
			expect(transposed).toEqual([19, 21, 21, 20, 19, 19]);
		});

		it('should handle empty initial state', () => {
			const text = '';
			const fingering = parseTabNotation(text);
			expect(fingering).toEqual([]);

			const generated = generateTabNotation(fingering, 0);
			expect(generated).toBe('');
		});

		it('should handle malformed input gracefully', () => {
			// Extra characters should be ignored or handled
			const text = 'x32010abc';
			const fingering = parseTabNotation(text);
			// Should parse first 6 valid characters
			expect(fingering.length).toBe(6);
		});
	});
});
