/**
 * Store for Name Chord mode
 */

import { writable, get } from 'svelte/store';
import { analyzeChord, type ChordMatch } from '$lib/wasm';
import { updateUrlParams, getParamValue, debounce } from '$lib/utils/url-state';

// State interface
export interface NameState {
	// Input
	tabInput: string;
	capo: number;
	startFret: number;

	// Results
	results: ChordMatch[];
	loading: boolean;
	error: string;
}

// Default state
const defaultState: NameState = {
	tabInput: '',
	capo: 0,
	startFret: 0,
	results: [],
	loading: false,
	error: '',
};

// Create the store
function createNameStore() {
	const store = writable<NameState>({ ...defaultState });

	// Circular update prevention flag
	let isUpdatingFromUrl = false;

	// Debounced URL update
	const debouncedUrlUpdate = debounce(() => {
		if (isUpdatingFromUrl) return;

		const state = get(store);
		updateUrlParams({
			tab: state.tabInput || undefined,
			capo: state.capo > 0 ? state.capo.toString() : undefined,
			startFret: state.startFret > 0 ? state.startFret.toString() : undefined,
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
			isUpdatingFromUrl = true;
			store.update((state) => ({
				...state,
				tabInput: getParamValue(searchParams, 'tab', ''),
				capo: getParamValue(searchParams, 'capo', 0, Number),
				startFret: getParamValue(searchParams, 'startFret', 0, Number),
			}));
			isUpdatingFromUrl = false;
		},

		/**
		 * Update tab input
		 */
		setTabInput(value: string) {
			store.update((state) => ({ ...state, tabInput: value }));
			debouncedUrlUpdate();
		},

		/**
		 * Set capo position
		 */
		setCapo(capo: number) {
			store.update((state) => ({ ...state, capo }));
			debouncedUrlUpdate();
		},

		/**
		 * Set start fret position
		 */
		setStartFret(startFret: number) {
			store.update((state) => ({ ...state, startFret }));
			debouncedUrlUpdate();
		},

		/**
		 * Execute analysis
		 */
		async analyze() {
			const state = get(store);
			if (!state.tabInput.trim() || state.loading) return;

			store.update((s) => ({ ...s, loading: true, error: '', results: [] }));

			try {
				const results = await analyzeChord(state.tabInput.trim());

				store.update((s) => ({
					...s,
					results: results.slice(0, 5), // we care only about top 5 results
					loading: false,
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
				tabInput: '',
				results: [],
				error: '',
			}));
			debouncedUrlUpdate();
		},
	};
}

export const nameStore = createNameStore();
