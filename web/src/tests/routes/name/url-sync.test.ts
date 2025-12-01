/**
 * Name page URL synchronization tests
 * Focus: URL updates when input/settings change and state syncs from URL
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { nameStore } from '$lib/stores/name';
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

describe('Name Page - URL Synchronization', () => {
	beforeEach(async () => {
		vi.useFakeTimers();
	});

	afterEach(() => {
		nameStore.clear();
		vi.clearAllMocks();
		vi.clearAllTimers();
	});

	it('should update URL when tab input changes', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		// Type tab input
		nameStore.setTabInput('x32010');

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				tab: 'x32010',
			})
		);
	});

	it('should update URL when capo is set', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		nameStore.setTabInput('x32010');
		nameStore.setCapo(2);

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				tab: 'x32010',
				capo: '2',
			})
		);
	});

	it('should update URL when startFret is set', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		nameStore.setTabInput('x-3-5-5-x-x');
		nameStore.setStartFret(5);

		// Wait for debounce
		vi.runAllTimers();

		expect(updateUrlParamsSpy).toHaveBeenCalledWith(
			expect.objectContaining({
				tab: 'x-3-5-5-x-x',
				startFret: '5',
			})
		);
	});

	it('should omit default values from URL', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		nameStore.setTabInput('x32010');
		nameStore.setCapo(0); // default
		nameStore.setStartFret(0); // default

		// Wait for debounce
		vi.runAllTimers();

		const lastCall = updateUrlParamsSpy.mock.calls[updateUrlParamsSpy.mock.calls.length - 1][0];
		expect(lastCall.capo).toBeUndefined();
		expect(lastCall.startFret).toBeUndefined();
	});

	it('should sync state from URL params on initFromUrl', () => {
		const params = new URLSearchParams({
			tab: '022100',
			capo: '3',
			startFret: '7',
		});

		nameStore.initFromUrl(params);

		const state = get(nameStore);
		expect(state.tabInput).toBe('022100');
		expect(state.capo).toBe(3);
		expect(state.startFret).toBe(7);
	});

	it('should not trigger URL update during initFromUrl (prevents circular updates)', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');
		updateUrlParamsSpy.mockClear();

		const params = new URLSearchParams({
			tab: 'x32010',
			capo: '2',
		});

		nameStore.initFromUrl(params);

		// Wait to ensure no debounced updates fire
		vi.runAllTimers();

		// initFromUrl should not trigger updateUrlParams due to isUpdatingFromUrl flag
		expect(updateUrlParamsSpy).not.toHaveBeenCalled();
	});

	it('should clear URL params when input is cleared', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		// Set some input
		nameStore.setTabInput('x32010');
		vi.runAllTimers();

		// Clear
		nameStore.clear();
		vi.runAllTimers();

		const lastCall = updateUrlParamsSpy.mock.calls[updateUrlParamsSpy.mock.calls.length - 1][0];
		expect(lastCall.tab).toBeUndefined();
	});

	it('should handle multiple settings changes without race conditions', async () => {
		const updateUrlParamsSpy = vi.spyOn(urlState, 'updateUrlParams');

		// Rapid changes
		nameStore.setTabInput('x32010');
		nameStore.setCapo(3);
		nameStore.setStartFret(5);

		// Wait for debounce
		vi.runAllTimers();

		// Should have final state with all changes
		const lastCall = updateUrlParamsSpy.mock.calls[updateUrlParamsSpy.mock.calls.length - 1][0];
		expect(lastCall.tab).toBe('x32010');
		expect(lastCall.capo).toBe('3');
		expect(lastCall.startFret).toBe('5');
	});
});
