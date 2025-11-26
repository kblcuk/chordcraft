/**
 * Progression page integration tests
 * Focus: Core user interactions for chord progression optimization
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import { get } from 'svelte/store';
import ProgressionPage from '../../../routes/progression/+page.svelte';
import { progressionStore } from '$lib/stores/progression';
import * as wasm from '$lib/wasm';

// Mock WASM module
vi.mock('$lib/wasm', () => ({
	initializeWasm: vi.fn().mockResolvedValue(undefined),
	generateProgression: vi.fn().mockResolvedValue([
		{
			chords: ['C', 'F', 'G'],
			fingerings: [
				{
					tab: 'x32010',
					score: 85,
					voicingType: 'full',
					hasRootInBass: true,
					position: 0,
					notes: ['C', 'E', 'G'],
				},
				{
					tab: 'xx3211',
					score: 82,
					voicingType: 'full',
					hasRootInBass: true,
					position: 1,
					notes: ['F', 'A', 'C'],
				},
				{
					tab: '320003',
					score: 88,
					voicingType: 'full',
					hasRootInBass: true,
					position: 0,
					notes: ['G', 'B', 'D'],
				},
			],
			transitions: [
				{ score: 90, movements: 2, anchors: 1, distance: 1 },
				{ score: 85, movements: 2, anchors: 1, distance: 1 },
			],
			totalScore: 250,
			avgTransitionScore: 87.5,
		},
	]),
}));

describe('Progression Page - Core User Flows', () => {
	beforeEach(() => {
		vi.clearAllMocks();
		progressionStore.clear();
	});

	it('should call generateProgression when user enters progression and presses Enter', async () => {
		const { container } = render(ProgressionPage);

		const input = container.querySelector('#progression-input') as HTMLInputElement;
		expect(input).toBeInTheDocument();

		await fireEvent.input(input, { target: { value: 'C F G' } });
		await fireEvent.keyDown(input, { key: 'Enter' });

		await vi.waitFor(() => {
			expect(wasm.generateProgression).toHaveBeenCalledWith(['C', 'F', 'G'], {
				limit: 3,
				maxFretDistance: 3,
				generatorOptions: {
					capo: 0,
					playingContext: 'solo',
				},
			});
		});
	});

	it('should display progression results with fingerings and transitions', async () => {
		const { container } = render(ProgressionPage);

		const input = container.querySelector('#progression-input') as HTMLInputElement;
		await fireEvent.input(input, { target: { value: 'C F G' } });
		await fireEvent.keyDown(input, { key: 'Enter' });

		await vi.waitFor(() => {
			const state = get(progressionStore);
			expect(state.results.length).toBe(1);
			expect(state.results[0].chords).toEqual(['C', 'F', 'G']);
			expect(state.results[0].fingerings.length).toBe(3);
			expect(state.results[0].transitions.length).toBe(2); // 3 chords = 2 transitions
		});
	});

	it('should clear input and results when store.clear() is called', () => {
		// Set up some state first by updating the store
		progressionStore.setProgressionInput('C F G');

		let state = get(progressionStore);
		expect(state.progressionInput).toBe('C F G');

		// Clear
		progressionStore.clear();

		state = get(progressionStore);
		expect(state.progressionInput).toBe('');
		expect(state.results).toEqual([]);
	});
});
