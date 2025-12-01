/**
 * Progression page URL synchronization tests
 * Focus: URL updates when input/settings change and state syncs from URL
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { progressionStore } from '$lib/stores/progression';
import * as urlState from '$lib/utils/url-state';

// Mock SvelteKit navigation
vi.mock('$app/navigation', () => ({
	goto: vi.fn(async () => {}),
}));

// Spy on updateUrlParams
vi.mock('$lib/utils/url-state', async () => {
	const actual = await vi.importActual<typeof urlState>('$lib/utils/url-state');
	return {
		...actual,
		updateUrlParams: vi.fn(actual.updateUrlParams),
	};
});

describe('Progression Page - URL Synchronization', () => {
	beforeEach(async () => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		progressionStore.clear();
		vi.clearAllMocks();
		vi.clearAllTimers();
	});

	it('should update URL when progression input changes', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		// Type progression input
		progressionStore.setProgressionInput('C Am F G');

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chords: 'C Am F G',
			})
		);
	});

	it('should update URL when limit is changed', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		progressionStore.setProgressionInput('Cmaj7 Am7 Dm7 G7');
		progressionStore.setOptions({ limit: 5 });

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chords: 'Cmaj7 Am7 Dm7 G7',
				limit: 5,
			})
		);
	});

	it('should update URL when maxDistance is changed', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		progressionStore.setProgressionInput('Emaj7 D Bm Cmaj7');
		progressionStore.setOptions({ maxDistance: 5 });

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chords: 'Emaj7 D Bm Cmaj7',
				maxDistance: 5,
			})
		);
	});

	it('should update URL when capo is set', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		progressionStore.setProgressionInput('F Bb Gm C');
		progressionStore.setOptions({ capo: 3 });

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chords: 'F Bb Gm C',
				capo: 3,
			})
		);
	});

	it('should update URL when context is set to band', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		progressionStore.setProgressionInput('Dm7 G7 Cmaj7');
		progressionStore.setOptions({ context: 'band' });

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chords: 'Dm7 G7 Cmaj7',
				context: 'band',
			})
		);
	});

	it('should omit default values from URL', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		progressionStore.setProgressionInput('C G Am F');
		progressionStore.setOptions({
			limit: 3, // default
			maxDistance: 3, // default
			capo: 0, // default
			context: 'solo', // default
		});

		// Wait for debounce
		vi.runAllTimers();

		const lastCall = updateUrlParamsSpy.mock.calls[updateUrlParamsSpy.mock.calls.length - 1][0];
		expect(lastCall.limit).toBeUndefined();
		expect(lastCall.maxDistance).toBeUndefined();
		expect(lastCall.capo).toBeUndefined();
		expect(lastCall.context).toBeUndefined();
	});

	it('should sync state from URL params on initFromUrl', () => {
		const params = new URLSearchParams({
			chords: 'Fmaj7 Dm7 Bbmaj7 C7',
			limit: '5',
			maxDistance: '4',
			capo: '2',
			context: 'band',
		});

		progressionStore.initFromUrl(params);

		const state = get(progressionStore);
		expect(state.progressionInput).toBe('Fmaj7 Dm7 Bbmaj7 C7');
		expect(state.limit).toBe(5);
		expect(state.maxDistance).toBe(4);
		expect(state.capo).toBe(2);
		expect(state.context).toBe('band');
	});

	it('should not trigger URL update during initFromUrl (prevents circular updates)', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');
		updateUrlParamsSpy.mockClear();

		const params = new URLSearchParams({
			chords: 'C Am F G',
			maxDistance: '5',
		});

		progressionStore.initFromUrl(params);

		// Wait to ensure no debounced updates fire
		vi.runAllTimers();

		// initFromUrl should not trigger updateUrlParams due to isUpdatingFromUrl flag
		expect(updateUrlParamsSpy).not.toHaveBeenCalled();
	});

	it('should clear URL params when input is cleared', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		// Set some input
		progressionStore.setProgressionInput('C Am F G');
		vi.runAllTimers();

		// Clear
		progressionStore.clear();
		vi.runAllTimers();

		const lastCall = updateUrlParamsSpy.mock.calls[updateUrlParamsSpy.mock.calls.length - 1][0];
		expect(lastCall.chords).toBeUndefined();
	});

	it('should handle multiple settings changes without race conditions', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		// Rapid changes
		progressionStore.setProgressionInput('C G Am F');
		progressionStore.setOptions({
			limit: 5,
			maxDistance: 4,
			capo: 2,
			context: 'band',
		});

		// Wait for debounce
		vi.runAllTimers();

		// Should have final state with all changes
		const lastCall = updateUrlParamsSpy.mock.calls[updateUrlParamsSpy.mock.calls.length - 1][0];
		expect(lastCall.chords).toBe('C G Am F');
		expect(lastCall.limit).toBe(5);
		expect(lastCall.maxDistance).toBe(4);
		expect(lastCall.capo).toBe(2);
		expect(lastCall.context).toBe('band');
	});
});
