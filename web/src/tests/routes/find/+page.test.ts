/**
 * Find page integration tests
 * Focus: Core user interactions lead to expected WASM calls and results
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import { get } from 'svelte/store';
import FindPage from '../../../routes/find/+page.svelte';
import { findStore } from '$lib/stores/find';
import * as wasm from '$lib/wasm';

// Mock WASM module
vi.mock('$lib/wasm', () => ({
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
}));

describe('Find Page - Core User Flows', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		findStore.clear();
	});

	it('should call findFingerings when user enters chord and presses Enter', async () => {
		const { container } = render(FindPage);

		const input = container.querySelector('#chord-input') as HTMLInputElement;
		expect(input).toBeInTheDocument();

		await fireEvent.input(input, { target: { value: 'C' } });
		await fireEvent.keyDown(input, { key: 'Enter' });

		await vi.waitFor(() => {
			expect(wasm.findFingerings).toHaveBeenCalledWith('C', {
				limit: 10,
				capo: 0,
				voicingType: undefined,
				preferredPosition: undefined,
				playingContext: 'solo',
			});
		});
	});

	it('should display results after successful search', async () => {
		const { container } = render(FindPage);

		const input = container.querySelector('#chord-input') as HTMLInputElement;
		await fireEvent.input(input, { target: { value: 'Cmaj7' } });
		await fireEvent.keyDown(input, { key: 'Enter' });

		await vi.waitFor(() => {
			const state = get(findStore);
			expect(state.results.length).toBeGreaterThan(0);
		});
	});

	it('should clear input and results when store.clear() is called', () => {
		// Set up some state first by updating the store
		findStore.setOptions({ chordInput: 'Cmaj7' });

		let state = get(findStore);
		expect(state.chordInput).toBe('Cmaj7');

		// Clear
		findStore.clear();

		state = get(findStore);
		expect(state.chordInput).toBe('');
		expect(state.results).toEqual([]);
	});
});
