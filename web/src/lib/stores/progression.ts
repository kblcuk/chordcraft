/**
 * Store for Chord Progression mode
 */

import { writable, derived, get } from 'svelte/store';
import { generateProgression, type ProgressionSequence } from '$lib/wasm';
import { updateUrlParams, getParamValue, debounce } from '$lib/utils/url-state';

// State interface
export interface ProgressionState {
	// Input
	progressionInput: string;

	// Options
	limit: number;
	maxDistance: number;
	capo: number;
	context: 'solo' | 'band';

	// Results
	results: ProgressionSequence[];
	loading: boolean;
	error: string;
	hasSearched: boolean;
}

// Default state
const defaultState: ProgressionState = {
	progressionInput: '',
	limit: 3,
	maxDistance: 3,
	capo: 0,
	context: 'solo',
	results: [],
	loading: false,
	error: '',
	hasSearched: false,
};

// Create the store
function createProgressionStore() {
	const store = writable<ProgressionState>({ ...defaultState });

	// Debounced URL update
	const debouncedUrlUpdate = debounce(() => {
		const state = get(store);
		updateUrlParams({
			chords: state.progressionInput || undefined,
			limit: state.limit !== 3 ? state.limit : undefined,
			maxDistance: state.maxDistance !== 3 ? state.maxDistance : undefined,
			capo: state.capo > 0 ? state.capo : undefined,
			context: state.context !== 'solo' ? state.context : undefined,
		});
	}, 300);

	return {
		subscribe: store.subscribe,
		set: store.set,
		update: store.update,

		/**
		 * Initialize from URL params
		 */
		initFromUrl(searchParams: URLSearchParams) {
			store.update((state) => ({
				...state,
				progressionInput: getParamValue(searchParams, 'chords', ''),
				limit: getParamValue(searchParams, 'limit', 3),
				maxDistance: getParamValue(searchParams, 'maxDistance', 3),
				capo: getParamValue(searchParams, 'capo', 0),
				context: getParamValue(
					searchParams,
					'context',
					'solo'
				) as ProgressionState['context'],
			}));
		},

		/**
		 * Update progression input
		 */
		setProgressionInput(value: string) {
			store.update((state) => ({ ...state, progressionInput: value }));
			debouncedUrlUpdate();
		},

		/**
		 * Update options
		 */
		setOptions(
			options: Partial<Pick<ProgressionState, 'limit' | 'maxDistance' | 'capo' | 'context'>>
		) {
			const state = get(store);
			const shouldAutoExecute = state.hasSearched && state.progressionInput.trim();

			store.update((s) => ({ ...s, ...options }));
			debouncedUrlUpdate();

			// Auto-execute generate if we've already generated once
			if (shouldAutoExecute) {
				// Use setTimeout to avoid calling generate during store update
				setTimeout(() => this.generate(), 0);
			}
		},

		/**
		 * Reset options to defaults
		 */
		resetOptions() {
			store.update((state) => ({
				...state,
				limit: defaultState.limit,
				maxDistance: defaultState.maxDistance,
				capo: defaultState.capo,
				context: defaultState.context,
			}));
			debouncedUrlUpdate();
		},

		/**
		 * Execute progression generation
		 */
		async generate() {
			const state = get(store);
			if (!state.progressionInput.trim() || state.loading) return;

			store.update((s) => ({ ...s, loading: true, error: '', results: [] }));

			try {
				const chords = state.progressionInput.trim().split(/\s+/);
				const results = await generateProgression(chords, {
					limit: state.limit,
					maxFretDistance: state.maxDistance,
					generatorOptions: {
						capo: state.capo,
						playingContext: state.context,
					},
				});

				store.update((s) => ({
					...s,
					results,
					loading: false,
					hasSearched: true,
				}));
			} catch (error) {
				store.update((s) => ({
					...s,
					error: error instanceof Error ? error.message : 'Unknown error',
					loading: false,
				}));
			}
		},

		/**
		 * Clear input and results
		 */
		clear() {
			store.update((state) => ({
				...state,
				progressionInput: '',
				results: [],
				error: '',
				hasSearched: false,
			}));
			debouncedUrlUpdate();
		},
	};
}

export const progressionStore = createProgressionStore();

// Derived stores for computed values
export const activeProgressionFilters = derived(progressionStore, ($progression) => {
	return [
		$progression.capo > 0 && 'Capo',
		$progression.limit !== 3 && 'Limit',
		$progression.maxDistance !== 3 && 'Max Distance',
		$progression.context === 'band' && 'Band Mode',
	].filter(Boolean).length;
});
