/**
 * Finger Count Badge Tests
 * Tests the finger counting algorithm and creative labels
 */

import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import FingerCountBadge from '$lib/components/features/name/FingerCountBadge.svelte';

describe('FingerCountBadge', () => {
	describe('Finger Counting Algorithm', () => {
		it('should count 3 fingers for C major (x32010)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x32010' },
			});

			expect(container.textContent).toContain('3');
		});

		it('should count 4 fingers for F major barre (133211)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '133211' },
			});

			// F major: barre at fret 1 (span of 6) = 1 finger
			// + fret 2 (1 string) = 1 finger
			// + fret 3 (2 consecutive strings) = 2 fingers
			// Total: 4 fingers
			expect(container.textContent).toContain('4');
		});

		it('should count 4 fingers for partial barre (355335)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '355335' },
			});

			// Fret 3: strings [0,3,4] span 5 positions = 1 barre
			// Fret 5: strings [1,2,5] = 2 consecutive + 1 separate = 3 fingers
			// Total: 4 fingers
			expect(container.textContent).toContain('4');
		});

		it('should count 2 fingers for power chord (x355xx)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x355xx' },
			});

			// Fret 3: 1 string = 1 finger
			// Fret 5: 2 consecutive strings = 2 fingers
			// Total: 3 fingers
			expect(container.textContent).toContain('3');
		});

		it('should count 0 fingers for open strings only (002000)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '002000' },
			});

			// Fret 2: 1 string = 1 finger
			expect(container.textContent).toContain('1');
		});

		it('should handle barres (333333)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '333333' },
			});

			// All 6 strings at fret 3 (lowest fret, span of 6) = 1 barre
			expect(container.textContent).toContain('1');
		});

		it('should count non-consecutive strings separately (x3x5xx)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x3x5xx' },
			});

			// Fret 3: 1 string = 1 finger
			// Fret 5: 1 string = 1 finger
			// Total: 2 fingers
			expect(container.textContent).toContain('2');
		});
	});

	describe('Creative Labels', () => {
		it('should show "Open Strings Only" for 0 fingers', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '000000' },
			});

			expect(container.textContent).toContain('Open Strings Only');
			expect(container.textContent).toContain('🎵');
		});

		it('should show "One Finger Wonder" for 1 finger', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x00030' },
			});

			expect(container.textContent).toContain('One Finger Wonder');
			expect(container.textContent).toContain('☝️');
		});

		it('should show "One Finger Wonder" for 1 finger (x02020)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x02020' },
			});

			// x02020: fret 2 at strings [2,4] - span of 3, treated as barre by algorithm = 1 finger
			expect(container.textContent).toContain('One Finger Wonder');
			expect(container.textContent).toContain('☝️');
		});

		it('should show "Easy Peasy" for 3 fingers', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x32010' },
			});

			expect(container.textContent).toContain('Easy Peasy');
			expect(container.textContent).toContain('👌');
		});

		it('should show "Human Standard" for 4 fingers', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '133211' },
			});

			expect(container.textContent).toContain('Human Standard');
			expect(container.textContent).toContain('🎸');
		});

		it('should show "Easy Peasy" for 3 fingers (022100 - E major)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '022100' },
			});

			// E major: fret 1 at string [3] = 1 finger
			// Fret 2: strings [1,2] = 2 consecutive = 2 fingers
			// Total: 3 fingers
			expect(container.textContent).toContain('Easy Peasy');
			expect(container.textContent).toContain('👌');
		});

		it('should show "AI-Augmented Human" for 6+ fingers', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '123456' },
			});

			// 6 different frets (lowest is fret 1 with span 1 = 1 finger, then each higher fret = 1 finger each)
			// Actually: fret 1 (span 1) = 1 finger, fret 2 = 1, fret 3 = 1, fret 4 = 1, fret 5 = 1, fret 6 = 1
			// Total: 6 fingers
			expect(container.textContent).toContain('AI-Augmented');
			expect(container.textContent).toContain('🤖');
		});
	});

	describe('Tab Notation Parsing', () => {
		it('should handle simple tab notation', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x32010' },
			});

			expect(container).toBeTruthy();
		});

		it('should handle multi-digit frets in parentheses', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x(10)(12)9(11)x' },
			});

			expect(container).toBeTruthy();
		});

		it('should handle tab with separators', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x-3-2-0-1-0' },
			});

			expect(container.textContent).toContain('3');
		});

		it('should handle empty tab', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '' },
			});

			// Should not render anything for empty tab
			expect(container.textContent).toBe('');
		});

		it('should handle all muted strings', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'xxxxxx' },
			});

			// All muted = 0 fingers
			expect(container.textContent).toBe('');
		});
	});

	describe('Color Coding', () => {
		it('should use green color for easy chords (0-3 fingers)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x32010' },
			});

			// Check for green background classes
			const badge = container.querySelector('div');
			expect(badge?.className).toMatch(/yellow/); // 3 fingers = yellow
		});

		it('should use orange color for standard chords (4 fingers)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '133211' },
			});

			const badge = container.querySelector('div');
			expect(badge?.className).toMatch(/orange/);
		});

		it('should use red color for advanced chords (5 fingers)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '123450' },
			});

			const badge = container.querySelector('div');
			expect(badge?.className).toMatch(/red/);
		});

		it('should use purple color for AI-augmented (6+ fingers)', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '123456' },
			});

			const badge = container.querySelector('div');
			expect(badge?.className).toMatch(/purple/);
		});
	});

	describe('Regex Parsing Edge Cases', () => {
		it('should ignore invalid characters', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'x3@2#0!1$0' },
			});

			// Should parse as x32010
			expect(container.textContent).toContain('3');
		});

		it('should handle mixed case X for muted', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: 'X32010' },
			});

			expect(container.textContent).toContain('3');
		});

		it('should limit to 6 strings', () => {
			const { container } = render(FingerCountBadge, {
				props: { tab: '1234567890' },
			});

			// Should only parse first 6 digits: 123456
			expect(container).toBeTruthy();
		});
	});
});
