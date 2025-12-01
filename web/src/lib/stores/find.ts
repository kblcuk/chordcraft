/**
 * Store for Find Fingerings mode
 */

import { writable, derived, get } from 'svelte/store';
import { findFingerings, type ScoredFingering } from '$lib/wasm';
import { updateUrlParams, getParamValue, debounce } from '$lib/utils/url-state';

// State interface
export interface FindState {
	// Input
	chordInput: string;

	// Options
	limit: number;
	capo: number;
	voicing: 'all' | 'core' | 'full' | 'jazzy';
	position: number | null;
	context: 'solo' | 'band';

	// Results
	results: ScoredFingering[];
	loading: boolean;
	error: string;
	hasSearched: boolean;
}

// Default state
const defaultState: FindState = {
	chordInput: '',
	limit: 10,
	capo: 0,
	voicing: 'all',
	position: null,
	context: 'solo',
	results: [],
	loading: false,
	error: '',
	hasSearched: false,
};

// Create the store
function createFindStore() {
	const store = writable<FindState>({ ...defaultState });

	/**
	 * Execute search
	 */
	async function search() {
		const state = get(store);
		if (!state.chordInput.trim() || state.loading) return;

		store.update((s) => ({ ...s, loading: true, error: '', results: [] }));

		try {
			const voicingType = state.voicing === 'all' ? undefined : state.voicing;
			const results = await findFingerings(state.chordInput.trim(), {
				limit: state.limit,
				capo: state.capo,
				voicingType,
				preferredPosition: state.position ?? undefined,
				playingContext: state.context,
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
	}

	// Circular update prevention flag
	let isUpdatingFromUrl = false;

	// Debounced URL update
	const debouncedUrlUpdate = debounce(() => {
		if (isUpdatingFromUrl) return;

		const state = get(store);
		updateUrlParams({
			chord: state.chordInput || undefined,
			limit: state.limit !== 10 ? state.limit : undefined,
			capo: state.capo > 0 ? state.capo : undefined,
			voicing: state.voicing !== 'all' ? state.voicing : undefined,
			position: state.position,
			context: state.context !== 'solo' ? state.context : undefined,
		});

		const shouldAutoExecute = state.chordInput.trim();
		// Auto-execute search if we've already searched once
		if (shouldAutoExecute) {
			// Use setTimeout to avoid calling search during store update
			setTimeout(() => search(), 0);
		}
	}, 200);

	return {
		subscribe: store.subscribe,
		set: store.set,
		update: store.update,
		search,
		/**
		 * Initialize from URL params
		 */
		initFromUrl(searchParams: URLSearchParams) {
			isUpdatingFromUrl = true;
			store.update((state) => ({
				...state,
				chordInput: getParamValue(searchParams, 'chord', ''),
				limit: getParamValue(searchParams, 'limit', 10),
				capo: getParamValue(searchParams, 'capo', 0),
				voicing: getParamValue(searchParams, 'voicing', 'all') as FindState['voicing'],
				position: getParamValue(searchParams, 'position', null, (v) =>
					v === 'null' ? null : Number(v)
				),
				context: getParamValue(searchParams, 'context', 'solo') as FindState['context'],
			}));
			isUpdatingFromUrl = false;
		},

		/**
		 * Update chord input
		 */
		setChordInput(value: string) {
			store.update((state) => ({ ...state, chordInput: value }));
			debouncedUrlUpdate();
		},

		/**
		 * Update options
		 */
		setOptions(
			options: Partial<Pick<FindState, 'limit' | 'capo' | 'voicing' | 'position' | 'context'>>
		) {
			store.update((s) => ({ ...s, ...options }));
			debouncedUrlUpdate();
		},

		/**
		 * Reset options to defaults
		 */
		resetOptions() {
			store.update((state) => ({
				...state,
				limit: defaultState.limit,
				capo: defaultState.capo,
				voicing: defaultState.voicing,
				position: defaultState.position,
				context: defaultState.context,
			}));
			debouncedUrlUpdate();
		},

		/**
		 * Clear input and results
		 */
		clear() {
			store.update((state) => ({
				...state,
				chordInput: '',
				results: [],
				error: '',
				hasSearched: false,
			}));
			debouncedUrlUpdate();
		},
	};
}

export const findStore = createFindStore();

// Derived stores for computed values
export const activeFindFilters = derived(findStore, ($find) => {
	return [
		$find.capo > 0 && 'Capo',
		$find.limit !== 10 && 'Limit',
		$find.voicing !== 'all' && 'Voicing',
		$find.position !== null && 'Position',
		$find.context === 'band' && 'Band Mode',
	].filter(Boolean).length;
});
