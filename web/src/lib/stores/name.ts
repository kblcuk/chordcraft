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

	// Results
	results: ChordMatch[];
	loading: boolean;
	error: string;
}

// Default state
const defaultState: NameState = {
	tabInput: '',
	results: [],
	loading: false,
	error: '',
};

// Create the store
function createNameStore() {
	const store = writable<NameState>({ ...defaultState });

	// Debounced URL update
	const debouncedUrlUpdate = debounce(() => {
		const state = get(store);
		updateUrlParams({
			tab: state.tabInput || undefined,
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
				tabInput: getParamValue(searchParams, 'tab', ''),
			}));
		},

		/**
		 * Update tab input
		 */
		setTabInput(value: string) {
			store.update((state) => ({ ...state, tabInput: value }));
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
					results,
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
