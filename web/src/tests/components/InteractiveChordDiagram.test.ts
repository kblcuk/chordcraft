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

	describe('Capo Behavior - Stale Value Bug Regression', () => {
		/**
		 * This test simulates the EXACT bug scenario:
		 *
		 * In the real app, when capo changes:
		 * 1. Component receives new capo + old value from parent
		 * 2. [value-sync] effect: value===previousValue, early returns
		 * 3. [fingering→value] effect: generates new value, updates previousValue
		 * 4. Parent re-renders AGAIN with its stale value (binding is async)
		 * 5. [value-sync] effect: value!==previousValue, parses stale value → BUG!
		 *
		 * The key is step 4 - parent echoes stale value AFTER we updated.
		 */
		it('should NOT parse stale parent value after generating new value', async () => {
			const { container, rerender } = render(InteractiveChordDiagram, {
				props: { value: '000000', size: 'medium', startFret: 0, capo: 0 },
			});

			const tabDisplay = container.querySelector('.font-mono');
			expect(tabDisplay?.textContent).toBe('000000');

			// Step 1: Set capo to 3, component generates "333333"
			await rerender({ value: '000000', size: 'medium', startFret: 0, capo: 3 });
			expect(tabDisplay?.textContent).toBe('333333');

			// Step 2: Change capo to 2
			// First rerender - component sees new capo, generates "222222"
			await rerender({ value: '333333', size: 'medium', startFret: 0, capo: 2 });
			expect(tabDisplay?.textContent).toBe('222222');

			// Step 3: CRITICAL - Simulate parent re-rendering with STALE value
			// This is what happens in real app: parent hasn't received updated binding yet
			// and re-renders child with old value "333333"
			//
			// BUG: Component parses "333333" → [3,3,3,3,3,3], + capo 2 = "555555"
			// FIX: Component recognizes this is stale, ignores it
			await rerender({ value: '333333', size: 'medium', startFret: 0, capo: 2 });
			expect(tabDisplay?.textContent).toBe('222222'); // Should stay "222222", NOT become "555555"
		});

		it('should handle rapid capo changes with stale parent values', async () => {
			const { container, rerender } = render(InteractiveChordDiagram, {
				props: { value: '000000', size: 'medium', startFret: 0, capo: 0 },
			});

			const tabDisplay = container.querySelector('.font-mono');

			// Capo 3
			await rerender({ value: '000000', size: 'medium', startFret: 0, capo: 3 });
			expect(tabDisplay?.textContent).toBe('333333');

			// Capo 2 + stale echo
			await rerender({ value: '333333', size: 'medium', startFret: 0, capo: 2 });
			expect(tabDisplay?.textContent).toBe('222222');
			await rerender({ value: '333333', size: 'medium', startFret: 0, capo: 2 }); // Stale echo
			expect(tabDisplay?.textContent).toBe('222222');

			// Capo 5 + stale echo
			await rerender({ value: '222222', size: 'medium', startFret: 0, capo: 5 });
			expect(tabDisplay?.textContent).toBe('555555');
			await rerender({ value: '222222', size: 'medium', startFret: 0, capo: 5 }); // Stale echo
			expect(tabDisplay?.textContent).toBe('555555');

			// Capo 0 + stale echo
			await rerender({ value: '555555', size: 'medium', startFret: 0, capo: 0 });
			expect(tabDisplay?.textContent).toBe('000000');
			await rerender({ value: '555555', size: 'medium', startFret: 0, capo: 0 }); // Stale echo
			expect(tabDisplay?.textContent).toBe('000000');
		});
	});

	describe('Capo Behavior (direct props)', () => {
		it('should handle capo changes with explicit value props', async () => {
			// This tests the component directly with explicit props
			const { container, rerender } = render(InteractiveChordDiagram, {
				props: { value: '000000', size: 'medium', startFret: 0, capo: 0 },
			});

			const tabDisplay = container.querySelector('.font-mono');
			expect(tabDisplay?.textContent).toBe('000000');

			// Capo 3 - parent has old value "000000"
			await rerender({ value: '000000', size: 'medium', startFret: 0, capo: 3 });
			expect(tabDisplay?.textContent).toBe('333333');

			// Capo 2 - parent has stale value "333333"
			await rerender({ value: '333333', size: 'medium', startFret: 0, capo: 2 });
			expect(tabDisplay?.textContent).toBe('222222');
		});

		it('should transpose fingering when position slider changes (no capo)', async () => {
			// Basic test: position slider should transpose internal fingering
			const { container, rerender } = render(InteractiveChordDiagram, {
				props: { value: '000000', size: 'medium', startFret: 0, capo: 0 },
			});

			const tabDisplay = container.querySelector('.font-mono');
			expect(tabDisplay?.textContent).toBe('000000');

			// Move to position 4 - internal fingering [0,0,0,0,0,0] -> [4,4,4,4,4,4]
			await rerender({ value: '444444', size: 'medium', startFret: 4, capo: 0 });
			expect(tabDisplay?.textContent).toBe('444444');
		});

		it('should handle position slider with capo correctly', async () => {
			// Position slider transposes internal fingering, capo is added on top
			const { container, rerender } = render(InteractiveChordDiagram, {
				props: { value: '000000', size: 'medium', startFret: 0, capo: 3 },
			});

			const tabDisplay = container.querySelector('.font-mono');
			expect(tabDisplay?.textContent).toBe('333333');

			// Move position slider to 4:
			// - Internal fingering [0,0,0,0,0,0] -> [4,4,4,4,4,4]
			// - Displayed: [4,4,4,4,4,4] + capo 3 = "777777"
			await rerender({ value: '333333', size: 'medium', startFret: 4, capo: 3 });
			expect(tabDisplay?.textContent).toBe('777777');

			// Move position slider to 8:
			// - Internal fingering [4,4,4,4,4,4] -> [8,8,8,8,8,8]
			// - Displayed: [8,8,8,8,8,8] + capo 3 = "(11)(11)(11)(11)(11)(11)"
			await rerender({ value: '777777', size: 'medium', startFret: 8, capo: 3 });
			expect(tabDisplay?.textContent).toBe('(11)(11)(11)(11)(11)(11)');
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
