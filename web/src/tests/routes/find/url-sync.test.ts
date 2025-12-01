/**
 * Find page URL synchronization tests
 * Focus: URL updates when input/settings change and state syncs from URL
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { findStore } from '$lib/stores/find';
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

describe('Find Page - URL Synchronization', () => {
	beforeEach(async () => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		findStore.clear();
		vi.clearAllMocks();
		vi.clearAllTimers();
	});

	it('should update URL when chord input changes', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		// Type chord input
		findStore.setChordInput('Cmaj7');

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chord: 'Cmaj7',
			})
		);
	});

	it('should update URL when capo is set', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		findStore.setChordInput('C');
		findStore.setOptions({ capo: 3 });

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chord: 'C',
				capo: 3,
			})
		);
	});

	it('should update URL when voicing filter is set', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		findStore.setChordInput('Cmaj7');
		findStore.setOptions({ voicing: 'core' });

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chord: 'Cmaj7',
				voicing: 'core',
			})
		);
	});

	it('should update URL when position preference is set', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		findStore.setChordInput('Am7');
		findStore.setOptions({ position: 5 });

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chord: 'Am7',
				position: 5,
			})
		);
	});

	it('should update URL when playing context is set to band', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		findStore.setChordInput('Dm7');
		findStore.setOptions({ context: 'band' });

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				chord: 'Dm7',
				context: 'band',
			})
		);
	});

	it('should omit default values from URL', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		findStore.setChordInput('G');
		findStore.setOptions({
			capo: 0, // default
			voicing: 'all', // default
			context: 'solo', // default
			limit: 10, // default
		});

		// Wait for debounce
		vi.runAllTimers();

		const lastCall = updateUrlParamsSpy.mock.calls[updateUrlParamsSpy.mock.calls.length - 1][0];
		expect(lastCall.capo).toBeUndefined();
		expect(lastCall.voicing).toBeUndefined();
		expect(lastCall.context).toBeUndefined();
		expect(lastCall.limit).toBeUndefined();
	});

	it('should sync state from URL params on initFromUrl', () => {
		const params = new URLSearchParams({
			chord: 'Fmaj7',
			capo: '2',
			voicing: 'jazzy',
			position: '7',
			context: 'band',
			limit: '5',
		});

		findStore.initFromUrl(params);

		const state = get(findStore);
		expect(state.chordInput).toBe('Fmaj7');
		expect(state.capo).toBe(2);
		expect(state.voicing).toBe('jazzy');
		expect(state.position).toBe(7);
		expect(state.context).toBe('band');
		expect(state.limit).toBe(5);
	});

	it('should not trigger URL update during initFromUrl (prevents circular updates)', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');
		updateUrlParamsSpy.mockClear();

		const params = new URLSearchParams({
			chord: 'Cmaj7',
			capo: '3',
		});

		findStore.initFromUrl(params);

		// Wait to ensure no debounced updates fire
		vi.runAllTimers();

		// initFromUrl should not trigger updateUrlParams due to isUpdatingFromUrl flag
		expect(updateUrlParamsSpy).not.toHaveBeenCalled();
	});

	it('should clear URL params when input is cleared', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		// Set some input
		findStore.setChordInput('Cmaj7');
		vi.runAllTimers();

		// Clear
		findStore.clear();
		vi.runAllTimers();

		const lastCall = updateUrlParamsSpy.mock.calls[updateUrlParamsSpy.mock.calls.length - 1][0];
		expect(lastCall.chord).toBeUndefined();
	});
});
