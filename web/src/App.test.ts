/**
 * App integration tests
 * Focus: User interactions and WASM API calls with correct parameters
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import App from './App.svelte';
import * as wasm from './lib/wasm';

// Mock WASM module
vi.mock('./lib/wasm', () => ({
	initializeWasm: vi.fn().mockResolvedValue(undefined),
	findFingerings: vi.fn().mockResolvedValue([
		{
			tab: 'x32010',
			score: 85,
			voicingType: 'full',
			hasRootInBass: true,
			position: 0,
			notes: ['C', 'E', 'G', 'C', 'E'],
		},
	]),
	analyzeChord: vi.fn().mockResolvedValue([
		{
			name: 'C',
			confidence: 100,
			explanation: 'C major chord',
		},
	]),
	generateProgression: vi.fn().mockResolvedValue([
		{
			chords: ['C', 'F', 'G'],
			fingerings: [],
			transitions: [],
			totalScore: 250,
			avgTransitionScore: 83.3,
		},
	]),
}));

describe('App - Find Mode Interactions', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('should call findFingerings with default options on initial search', async () => {
		const { container } = render(App);

		// Wait for WASM to initialize
		await vi.waitFor(() => {
			expect(wasm.initializeWasm).toHaveBeenCalled();
		});

		// Enter chord name (use fireEvent for text inputs - userEvent doesn't sync with Svelte bind:value)
		const input = container.querySelector('#chord-input') as HTMLInputElement;
		expect(input).toBeInTheDocument();

		await fireEvent.input(input, { target: { value: 'C' } });

		// Trigger search by pressing Enter
		await fireEvent.keyDown(input, { key: 'Enter' });

		// Verify WASM called with default options
		await vi.waitFor(() => {
			expect(wasm.findFingerings).toHaveBeenCalledWith('C', {
				limit: 10,
				capo: 0,
				voicingType: undefined, // 'all' -> undefined
				preferredPosition: undefined,
				playingContext: 'solo',
			});
		});
	});

	it('should call findFingerings with updated limit when limit slider changes', async () => {
		const user = userEvent.setup();
		const { container } = render(App);

		await vi.waitFor(() => expect(wasm.initializeWasm).toHaveBeenCalled());

		// Enter chord and search (use fireEvent for inputs)
		const input = container.querySelector('#chord-input') as HTMLInputElement;
		await fireEvent.input(input, { target: { value: 'C' } });
		await fireEvent.keyDown(input, { key: 'Enter' });

		// Wait for initial search
		await vi.waitFor(() => expect(wasm.findFingerings).toHaveBeenCalledTimes(1));

		// Open advanced options (use userEvent for clicks)
		const advancedButton = screen.getByText('Advanced');
		await user.click(advancedButton);

		// Change limit slider
		const limitSlider = container.querySelector('#find-limit') as HTMLInputElement;
		expect(limitSlider).toBeInTheDocument();
		await fireEvent.input(limitSlider, { target: { value: '20' } });

		// Should auto-trigger new search with updated limit
		await vi.waitFor(() => {
			expect(wasm.findFingerings).toHaveBeenCalledWith('C', {
				limit: 20, // Changed from 10 to 20
				capo: 0,
				voicingType: undefined,
				preferredPosition: undefined,
				playingContext: 'solo',
			});
		});
	});

	it('should render capo selector with correct options', async () => {
		const user = userEvent.setup();
		const { container } = render(App);

		await vi.waitFor(() => expect(wasm.initializeWasm).toHaveBeenCalled());

		// Open advanced options
		const advancedButton = screen.getByText('Advanced');
		await user.click(advancedButton);

		// Verify capo selector exists and has correct options
		const capoSelect = container.querySelector('#find-capo') as HTMLSelectElement;
		expect(capoSelect).toBeInTheDocument();

		// Check that all expected capo options are available
		const options = Array.from(capoSelect.options).map((opt) => opt.value);
		expect(options).toContain('0'); // No capo
		expect(options).toContain('3'); // Fret 3
		expect(options).toContain('12'); // Fret 12
		expect(options.length).toBe(13); // 0-12

		// NOTE: Testing select value changes with Svelte bind:value doesn't work reliably
		// in testing environments due to limitations of @testing-library with Svelte's
		// two-way binding. This test verifies the UI exists correctly. The capo functionality
		// itself is tested via the WASM layer unit tests.
	});

	it('should call findFingerings with voicingType when voicing radio changes', async () => {
		const user = userEvent.setup();
		const { container } = render(App);

		await vi.waitFor(() => expect(wasm.initializeWasm).toHaveBeenCalled());

		// Initial search (use fireEvent for inputs)
		const input = container.querySelector('#chord-input') as HTMLInputElement;
		await fireEvent.input(input, { target: { value: 'Cmaj7' } });
		await fireEvent.keyDown(input, { key: 'Enter' });
		await vi.waitFor(() => expect(wasm.findFingerings).toHaveBeenCalledTimes(1));

		// Open advanced options (use userEvent for clicks)
		const advancedButton = screen.getByText('Advanced');
		await user.click(advancedButton);

		// Find "Core" radio button by value attribute
		const radios = container.querySelectorAll('input[type="radio"]');
		const coreRadio = Array.from(radios).find(
			(r) => (r as HTMLInputElement).value === 'core'
		) as HTMLInputElement;

		expect(coreRadio).toBeDefined();
		await user.click(coreRadio);

		// Should auto-trigger with voicingType: 'core'
		await vi.waitFor(() => {
			expect(wasm.findFingerings).toHaveBeenCalledWith('Cmaj7', {
				limit: 10,
				capo: 0,
				voicingType: 'core', // Changed from undefined to 'core'
				preferredPosition: undefined,
				playingContext: 'solo',
			});
		});
	});

	it('should render position selector with correct options', async () => {
		const user = userEvent.setup();
		const { container } = render(App);

		await vi.waitFor(() => expect(wasm.initializeWasm).toHaveBeenCalled());

		// Open advanced options
		const advancedButton = screen.getByText('Advanced');
		await user.click(advancedButton);

		// Verify position selector exists and has correct options
		const positionSelect = container.querySelector('#find-position') as HTMLSelectElement;
		expect(positionSelect).toBeInTheDocument();

		// Check that expected position options are available
		const options = Array.from(positionSelect.options);
		const values = options.map((opt) => opt.value);

		expect(values).toContain(''); // Any position (null)
		expect(values).toContain('0'); // Open position
		expect(values).toContain('7'); // Around fret 7
		expect(values.length).toBe(14); // null + 0 + 1-12

		// NOTE: Testing select value changes with Svelte bind:value doesn't work reliably
		// in testing environments due to limitations of @testing-library with Svelte's
		// two-way binding. This test verifies the UI exists correctly. The position functionality
		// itself is tested via the WASM layer unit tests.
	});

	it('should call findFingerings with playingContext when context changes', async () => {
		const user = userEvent.setup();
		const { container } = render(App);

		await vi.waitFor(() => expect(wasm.initializeWasm).toHaveBeenCalled());

		// Initial search (use fireEvent for inputs)
		const input = container.querySelector('#chord-input') as HTMLInputElement;
		await fireEvent.input(input, { target: { value: 'Am7' } });
		await fireEvent.keyDown(input, { key: 'Enter' });
		await vi.waitFor(() => expect(wasm.findFingerings).toHaveBeenCalledTimes(1));

		// Open advanced options (use userEvent for clicks)
		const advancedButton = screen.getByText('Advanced');
		await user.click(advancedButton);

		// Find and click "Band" radio button (use userEvent for clicks)
		const radios = container.querySelectorAll('input[type="radio"]');
		const bandRadio = Array.from(radios).find(
			(r) => (r as HTMLInputElement).value === 'band'
		) as HTMLInputElement;

		expect(bandRadio).toBeInTheDocument();
		await user.click(bandRadio);

		// Should auto-trigger with playingContext: 'band'
		await vi.waitFor(() => {
			expect(wasm.findFingerings).toHaveBeenCalledWith('Am7', {
				limit: 10,
				capo: 0,
				voicingType: undefined,
				preferredPosition: undefined,
				playingContext: 'band', // Changed from 'solo' to 'band'
			});
		});
	});

	it('should NOT trigger search when filters change before initial search', async () => {
		const user = userEvent.setup();
		const { container } = render(App);

		await vi.waitFor(() => expect(wasm.initializeWasm).toHaveBeenCalled());

		// Open advanced options WITHOUT searching first
		const advancedButton = screen.getByText('Advanced');
		await user.click(advancedButton);

		// Change limit slider (use fireEvent for range inputs)
		const limitSlider = container.querySelector('#find-limit') as HTMLInputElement;
		await fireEvent.input(limitSlider, { target: { value: '25' } });

		// Should NOT trigger search (no initial search yet)
		expect(wasm.findFingerings).not.toHaveBeenCalled();
	});
});
