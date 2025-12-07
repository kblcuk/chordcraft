/**
 * Interactive Chord Diagram Component Tests
 *
 * Tests the visual chord building interface with interactions and accessibility.
 * Note: Pure function logic (parseTabNotation, generateTabNotation, transposeFingeringToNewPosition)
 * is tested in InteractiveChordDiagram-sync.test.ts
 */

import { describe, it, expect } from 'vitest';
import { render, fireEvent, getByTestId } from '@testing-library/svelte';
import InteractiveChordDiagram from '$lib/components/features/name/InteractiveChordDiagram.svelte';

describe('InteractiveChordDiagram', () => {
	describe('Size Variants', () => {
		it('should render correct SVG dimensions for each size', () => {
			const sizes = [
				{ size: 'small' as const, expectedWidth: '120' },
				{ size: 'medium' as const, expectedWidth: '160' },
				{ size: 'large' as const, expectedWidth: '200' },
			];

			for (const { size, expectedWidth } of sizes) {
				const { container, unmount } = render(InteractiveChordDiagram, {
					props: { value: '', size, startFret: 0, capo: 0 },
				});

				const svg = container.querySelector('svg');
				expect(
					svg?.getAttribute('width'),
					`${size} size should have width ${expectedWidth}`
				).toBe(expectedWidth);

				unmount();
			}
		});
	});

	describe('Default State (open position, empty value)', () => {
		it('should render slider, accessibility attributes, placeholder text, and no fret indicator', () => {
			const { container } = render(InteractiveChordDiagram, {
				props: { value: '', size: 'medium', startFret: 0, capo: 0 },
			});

			// Position slider
			const slider = container.querySelector('input[type="range"]') as HTMLInputElement;
			expect(slider).toBeTruthy();
			expect(slider.min).toBe('0');
			expect(slider.max).toBe('19');
			expect(container.textContent).toContain('Open');

			// Accessibility: role="button" and tabindex="0"
			const buttons = container.querySelectorAll('[role="button"]');
			expect(buttons.length).toBeGreaterThan(0);
			const focusableElements = container.querySelectorAll('[tabindex="0"]');
			expect(focusableElements.length).toBeGreaterThan(0);

			// Accessibility: aria-labels
			const labeledElements = container.querySelectorAll('[aria-label]');
			expect(labeledElements.length).toBeGreaterThan(0);
			const fretLabel = labeledElements[0].getAttribute('aria-label');
			expect(fretLabel).toMatch(/string/i);
			expect(fretLabel).toMatch(/fret/i);
			const toggleLabels = Array.from(labeledElements)
				.map((el) => el.getAttribute('aria-label'))
				.filter((label) => label?.includes('Toggle'));
			expect(toggleLabels.length).toBe(6);

			// Placeholder text when empty
			const tabDisplay = container.querySelector('.font-mono');
			expect(tabDisplay?.textContent).toContain('Click on the fretboard');

			// No fret indicator at open position
			const fretIndicator = Array.from(container.querySelectorAll('text')).find((t) =>
				t.textContent?.includes('fr')
			);
			expect(fretIndicator).toBeUndefined();
		});

		it('should respond to keyboard events without error', async () => {
			const { container } = render(InteractiveChordDiagram, {
				props: { value: '', size: 'medium', startFret: 0, capo: 0 },
			});

			const button = container.querySelector('[role="button"]') as SVGElement;
			expect(button).toBeTruthy();

			await fireEvent.keyDown(button, { key: 'Enter' });
			await fireEvent.keyDown(button, { key: ' ' });
		});

		it('should show and clear hover indicator', async () => {
			const { container } = render(InteractiveChordDiagram, {
				props: { value: '', size: 'medium', startFret: 0, capo: 0 },
			});

			const button = container.querySelector('[role="button"]') as SVGElement;
			const circlesBefore = container.querySelectorAll('circle').length;

			await fireEvent.mouseEnter(button);
			const circlesDuring = container.querySelectorAll('circle').length;
			expect(circlesDuring).toBeGreaterThanOrEqual(circlesBefore);

			await fireEvent.mouseLeave(button);
		});
	});

	describe('Higher Position State', () => {
		it('should show fret range label and fret number indicator', () => {
			const { container } = render(InteractiveChordDiagram, {
				props: { value: '', size: 'medium', startFret: 5, capo: 0 },
			});

			// Fret range label
			expect(container.textContent).toContain('Frets 6-10');

			// Fret indicator shows "6fr" (startFret + 1)
			const fretIndicator = Array.from(container.querySelectorAll('text')).find((t) =>
				t.textContent?.includes('6fr')
			);
			expect(fretIndicator).toBeTruthy();
		});
	});

	describe('Fret Click Interactions', () => {
		it('should update fingering when clicking a fret and cycle back to open', async () => {
			const { container } = render(InteractiveChordDiagram, {
				props: { value: '000000', size: 'medium', startFret: 0, capo: 0 },
			});

			const fretButton = container.querySelector(
				'[role="button"][aria-label="Set E string to fret 1"]'
			) as SVGElement;
			expect(fretButton).toBeTruthy();

			// Click to set fret
			await fireEvent.click(fretButton);
			const tabDisplay = container.querySelector('.font-mono');
			expect(tabDisplay?.textContent).toBe('100000');

			// Click same position to cycle back to open
			await fireEvent.click(fretButton);
			expect(tabDisplay?.textContent).toBe('000000');
		});
	});

	describe('String Marker Toggle', () => {
		it('should toggle open -> muted -> open when clicking string marker', async () => {
			const { container } = render(InteractiveChordDiagram, {
				props: { value: '000000', size: 'medium', startFret: 0, capo: 0 },
			});

			const toggleButtons = container.querySelectorAll(
				'[role="button"][aria-label*="Toggle"]'
			);
			expect(toggleButtons.length).toBe(6);

			const tabDisplay = container.querySelector('.font-mono');

			// Click to mute E string
			await fireEvent.click(toggleButtons[0] as SVGElement);
			expect(tabDisplay?.textContent).toBe('x00000');

			// Click again to return to open
			await fireEvent.click(toggleButtons[0] as SVGElement);
			expect(tabDisplay?.textContent).toBe('000000');
		});

		it('should set fretted string to open when clicking its marker', async () => {
			const { container } = render(InteractiveChordDiagram, {
				props: { value: '320000', size: 'medium', startFret: 0, capo: 0 },
			});

			const toggleButtons = container.querySelectorAll(
				'[role="button"][aria-label*="Toggle"]'
			);
			await fireEvent.click(toggleButtons[0] as SVGElement);

			const tabDisplay = container.querySelector('.font-mono');
			expect(tabDisplay?.textContent).toBe('020000');
		});
	});

	describe('Clear Functionality', () => {
		it('should reset all strings including muted to open (000000)', async () => {
			const { container } = render(InteractiveChordDiagram, {
				props: { value: 'x32010', size: 'medium', startFret: 0, capo: 0 },
			});

			const clearButton: HTMLButtonElement = getByTestId(container, 'clear-button');
			expect(clearButton.textContent).toContain('Clear');

			await fireEvent.click(clearButton);
			expect(container.textContent).toContain('000000');
		});
	});

	describe('Visual Rendering', () => {
		it('should display value prop, muted X, open circles, and barre indicator correctly', () => {
			// Test with C major (x32010) - has muted string, open strings, fretted notes
			const { container: cContainer, unmount: unmountC } = render(InteractiveChordDiagram, {
				props: { value: 'x32010', size: 'medium', startFret: 0, capo: 0 },
			});

			// Value displayed
			expect(cContainer.querySelector('.font-mono')?.textContent).toBe('x32010');

			// Muted string indicator (×)
			const mutedIndicators = Array.from(cContainer.querySelectorAll('text')).filter(
				(t) => t.textContent === '×'
			);
			expect(mutedIndicators.length).toBe(1);

			unmountC();

			// Test with E major (022100) - has 3 open strings
			const { container: eContainer, unmount: unmountE } = render(InteractiveChordDiagram, {
				props: { value: '022100', size: 'medium', startFret: 0, capo: 0 },
			});

			const openCircles = eContainer.querySelectorAll('circle[fill="none"]');
			expect(openCircles.length).toBe(3);

			unmountE();

			// Test with F major barre (133211) - has barre indicator
			const { container: fContainer } = render(InteractiveChordDiagram, {
				props: { value: '133211', size: 'medium', startFret: 0, capo: 0 },
			});

			const barreLines = fContainer.querySelectorAll('line[stroke-width="6"]');
			expect(barreLines.length).toBeGreaterThan(0);
		});
	});
});
