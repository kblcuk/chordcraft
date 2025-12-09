/**
 * ChordDiagram component tests
 * Focus: User-facing behavior and rendering
 */

import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import ChordDiagram from './ChordDiagram.svelte';

describe('ChordDiagram - User Experience', () => {
	describe('Tab Format Support', () => {
		it('should render simple single-digit tab notation (e.g., x32010)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x32010',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toBeInTheDocument();
		});

		it('should render multi-digit frets with parentheses (e.g., x(10)(12)(11)x)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x(10)(12)(12)(11)x',
					notes: ['D', 'F#', 'A', 'C#'],
					rootNote: 'D',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toBeInTheDocument();
		});

		it('should render mixed single and multi-digit (e.g., 9(11)(11)(10)99)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '9(11)(11)(10)99',
					notes: ['A', 'C#', 'E', 'G#'],
					rootNote: 'A',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toBeInTheDocument();
		});

		it('should handle tab with separators (e.g., x-3-2-0-1-0)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x-3-2-0-1-0',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toBeInTheDocument();
		});

		it('should handle tab with spaces (e.g., x 3 2 0 1 0)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x 3 2 0 1 0',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toBeInTheDocument();
		});
	});

	describe('Edge Cases', () => {
		it('should handle all muted strings (xxxxxx)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'xxxxxx',
					notes: [],
					rootNote: '',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toBeInTheDocument();
		});

		it('should handle all open strings (000000)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '000000',
					notes: ['E', 'A', 'D', 'G', 'B', 'E'],
					rootNote: 'E',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toBeInTheDocument();
		});

		it('should handle empty tab string gracefully', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '',
					notes: [],
					rootNote: '',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toBeInTheDocument();
		});

		it('should trim whitespace from tab input', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '  x32010  ',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toBeInTheDocument();
		});
	});

	describe('Visual Elements', () => {
		it('should render 6 strings (vertical lines)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x32010',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
				},
			});

			// Count vertical lines (strings)
			const lines = container.querySelectorAll('line');
			const verticalLines = Array.from(lines).filter((line) => {
				const x1 = line.getAttribute('x1');
				const x2 = line.getAttribute('x2');
				return x1 === x2; // Vertical line
			});

			expect(verticalLines.length).toBe(6);
		});

		it('should render frets (horizontal lines)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x32010',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
				},
			});

			// Count horizontal lines (frets) - should be 6 (nut + 5 frets)
			const lines = container.querySelectorAll('line');
			const horizontalLines = Array.from(lines).filter((line) => {
				const y1 = line.getAttribute('y1');
				const y2 = line.getAttribute('y2');
				return y1 === y2; // Horizontal line
			});

			expect(horizontalLines.length).toBeGreaterThanOrEqual(6);
		});

		it('should show muted string indicator (×) for x positions', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x32010',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
				},
			});

			const text = container.textContent;
			expect(text).toContain('×');
		});

		it('should show fret number for high positions', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '(10)(12)(12)(11)(10)(10)',
					notes: ['D', 'F#', 'A', 'C#'],
					rootNote: 'D',
				},
			});

			const text = container.textContent;
			expect(text).toMatch(/\d+fr/); // Should show "10fr" or similar
		});

		it('should render finger position dots', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x32010',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
				},
			});

			const circles = container.querySelectorAll('circle');
			expect(circles.length).toBeGreaterThan(0);
		});
	});

	describe('Size Variants', () => {
		it('should render small size', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x32010',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
					size: 'small',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toHaveAttribute('width', '120');
			expect(svg).toHaveAttribute('height', '160');
		});

		it('should render medium size (default)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x32010',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
					size: 'medium',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toHaveAttribute('width', '160');
			expect(svg).toHaveAttribute('height', '200');
		});

		it('should render large size', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x32010',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
					size: 'large',
				},
			});

			const svg = container.querySelector('svg');
			expect(svg).toHaveAttribute('width', '200');
			expect(svg).toHaveAttribute('height', '250');
		});
	});

	describe('Real-World Chord Examples', () => {
		it('should render C major chord (x32010)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x32010',
					notes: ['C', 'E', 'G', 'C', 'E'],
					rootNote: 'C',
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});

		it('should render G major chord (320003)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '320003',
					notes: ['G', 'B', 'D', 'G', 'D', 'G'],
					rootNote: 'G',
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});

		it('should render D major chord (xx0232)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'xx0232',
					notes: ['D', 'A', 'D', 'F#'],
					rootNote: 'D',
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});

		it('should render F major barre chord (133211)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '133211',
					notes: ['F', 'C', 'F', 'A', 'C', 'F'],
					rootNote: 'F',
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});

		it('should render high position chord (9(11)(11)(10)99)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '9(11)(11)(10)99',
					notes: ['A', 'C#', 'E', 'G#'],
					rootNote: 'A',
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});
	});

	describe('Ukulele Support (4 strings)', () => {
		it('should render ukulele C major chord (0003)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '0003',
					notes: ['G', 'C', 'E', 'C'],
					rootNote: 'C',
					stringCount: 4,
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});

		it('should render 4 strings for ukulele', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '0003',
					notes: ['G', 'C', 'E', 'C'],
					rootNote: 'C',
					stringCount: 4,
				},
			});

			// Count vertical lines (strings)
			const lines = container.querySelectorAll('line');
			const verticalLines = Array.from(lines).filter((line) => {
				const x1 = line.getAttribute('x1');
				const x2 = line.getAttribute('x2');
				return x1 === x2; // Vertical line
			});

			expect(verticalLines.length).toBe(4);
		});

		it('should render ukulele Am chord (2000)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '2000',
					notes: ['A', 'C', 'E', 'A'],
					rootNote: 'A',
					stringCount: 4,
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});

		it('should render ukulele G chord (0232)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '0232',
					notes: ['G', 'D', 'G', 'B'],
					rootNote: 'G',
					stringCount: 4,
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});

		it('should render ukulele barre chord (4442)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '4442',
					notes: ['B', 'E', 'G#', 'B'],
					rootNote: 'E',
					stringCount: 4,
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});

		it('should handle ukulele muted strings (x232)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: 'x232',
					notes: ['D', 'G', 'B'],
					rootNote: 'G',
					stringCount: 4,
				},
			});

			const text = container.textContent;
			expect(text).toContain('×');
		});

		it('should render ukulele high position chord (7779)', () => {
			const { container } = render(ChordDiagram, {
				props: {
					tab: '7779',
					notes: ['D', 'A', 'D', 'G'],
					rootNote: 'D',
					stringCount: 4,
				},
			});

			expect(container.querySelector('svg')).toBeInTheDocument();
		});
	});
});
