/**
 * URL State Management - Single Source of Truth
 *
 * All input/option state is derived from URL parameters.
 * Routes react to URL changes via $effect and trigger searches.
 */

import { goto } from '$app/navigation';
import { browser } from '$app/environment';
import { page } from '$app/state';
import type { Instrument } from '$lib/wasm';

// =============================================================================
// Route Configuration
// =============================================================================

export const routes = [
	{ path: '/find', label: 'Find Fingerings' },
	{ path: '/name', label: 'Name Chord' },
	{ path: '/progression', label: 'Progression' },
] as const;

// =============================================================================
// Type Definitions
// =============================================================================

export type Voicing = 'all' | 'core' | 'full' | 'jazzy';
export type PlayingContext = 'solo' | 'band';

export interface FindUrlState {
	chord: string;
	limit: number;
	capo: number;
	voicing: Voicing;
	position: number | null;
	context: PlayingContext;
	instrument: Instrument;
}

export interface NameUrlState {
	tab: string;
	capo: number;
	startFret: number;
	instrument: Instrument;
}

export interface ProgressionUrlState {
	chords: string;
	limit: number;
	maxDistance: number;
	capo: number;
	context: PlayingContext;
	instrument: Instrument;
}

// =============================================================================
// Default Values
// =============================================================================

export const FIND_DEFAULTS = {
	chord: '',
	limit: 10,
	capo: 0,
	voicing: 'all' as Voicing,
	position: null as number | null,
	context: 'solo' as PlayingContext,
} as const;

export const NAME_DEFAULTS = {
	tab: '',
	capo: 0,
	startFret: 0,
} as const;

export const PROGRESSION_DEFAULTS = {
	chords: '',
	limit: 3,
	maxDistance: 3,
	capo: 0,
	context: 'solo' as PlayingContext,
} as const;

// =============================================================================
// Parsing Helpers
// =============================================================================

function parseString(params: URLSearchParams, key: string, defaultValue: string): string {
	return params.get(key) ?? defaultValue;
}

function parseNumber(params: URLSearchParams, key: string, defaultValue: number): number {
	const value = params.get(key);
	if (value === null) return defaultValue;
	const parsed = Number(value);
	return isNaN(parsed) ? defaultValue : parsed;
}

function parseNumberOrNull(params: URLSearchParams, key: string): number | null {
	const value = params.get(key);
	if (value === null || value === '') return null;
	const parsed = Number(value);
	return isNaN(parsed) ? null : parsed;
}

function parseInstrument(params: URLSearchParams): Instrument {
	const value = params.get('instrument')?.toLowerCase();

	// List of valid instruments (matches TypeScript Instrument type)
	const validInstruments: Instrument[] = [
		'guitar',
		'ukulele',
		'baritone-ukulele',
		'bass',
		'bass-5',
		'mandolin',
		'banjo',
		'guitar-7',
		'drop-d',
		'open-g',
		'dadgad',
	];

	// Validate and return, default to guitar if invalid
	return validInstruments.includes(value as Instrument) ? (value as Instrument) : 'guitar';
}

function parseVoicing(params: URLSearchParams): Voicing {
	const value = params.get('voicing');
	if (value === 'core' || value === 'full' || value === 'jazzy') return value;
	return 'all';
}

function parseContext(params: URLSearchParams): PlayingContext {
	const value = params.get('context');
	return value === 'band' ? 'band' : 'solo';
}

// =============================================================================
// URL State Parsers
// =============================================================================

export function parseFindParams(searchParams: URLSearchParams): FindUrlState {
	return {
		chord: parseString(searchParams, 'chord', FIND_DEFAULTS.chord),
		limit: parseNumber(searchParams, 'limit', FIND_DEFAULTS.limit),
		capo: parseNumber(searchParams, 'capo', FIND_DEFAULTS.capo),
		voicing: parseVoicing(searchParams),
		position: parseNumberOrNull(searchParams, 'position'),
		context: parseContext(searchParams),
		instrument: parseInstrument(searchParams),
	};
}

export function parseNameParams(searchParams: URLSearchParams): NameUrlState {
	return {
		tab: parseString(searchParams, 'tab', NAME_DEFAULTS.tab),
		capo: parseNumber(searchParams, 'capo', NAME_DEFAULTS.capo),
		startFret: parseNumber(searchParams, 'startFret', NAME_DEFAULTS.startFret),
		instrument: parseInstrument(searchParams),
	};
}

export function parseProgressionParams(searchParams: URLSearchParams): ProgressionUrlState {
	return {
		chords: parseString(searchParams, 'chords', PROGRESSION_DEFAULTS.chords),
		limit: parseNumber(searchParams, 'limit', PROGRESSION_DEFAULTS.limit),
		maxDistance: parseNumber(searchParams, 'maxDistance', PROGRESSION_DEFAULTS.maxDistance),
		capo: parseNumber(searchParams, 'capo', PROGRESSION_DEFAULTS.capo),
		context: parseContext(searchParams),
		instrument: parseInstrument(searchParams),
	};
}

// =============================================================================
// URL Update
// =============================================================================

type UrlParamValue = string | number | boolean | null | undefined;

/**
 * Update URL with new parameters.
 * - Omits null/undefined/empty values
 * - Omits instrument if 'guitar' (default)
 * - Uses replaceState by default to avoid cluttering history
 */
export function updateUrl(
	params: Record<string, UrlParamValue>,
	options: { replaceState?: boolean } = {}
): void {
	if (!browser) return;

	const searchParams = new URLSearchParams();

	Object.entries(params).forEach(([key, value]) => {
		if (value === null || value === undefined || value === '') return;

		// Skip instrument if guitar (default)
		if (key === 'instrument' && value === 'guitar') return;

		searchParams.set(key, String(value));
	});

	const query = searchParams.toString();
	const pathname = page.url.pathname;
	const url = query ? `${pathname}?${query}` : pathname;

	// We can enalbe this back when goto supports query params:
	// https://github.com/sveltejs/kit/issues/14750
	// eslint-disable-next-line svelte/no-navigation-without-resolve
	goto(url, {
		replaceState: options.replaceState ?? true,
		keepFocus: true,
		noScroll: true,
	}).catch((e) => {
		// Ignore navigation errors (user might navigate away)
		console.error('Navigation error', e);
	});
}

/**
 * Build URL params object from current state, omitting defaults.
 * This is useful for updateUrl() calls.
 */
export function buildFindParams(
	state: FindUrlState
): Record<string, string | number | null | undefined> {
	return {
		chord: state.chord || undefined,
		limit: state.limit !== FIND_DEFAULTS.limit ? state.limit : undefined,
		capo: state.capo > 0 ? state.capo : undefined,
		voicing: state.voicing !== FIND_DEFAULTS.voicing ? state.voicing : undefined,
		position: state.position,
		context: state.context !== FIND_DEFAULTS.context ? state.context : undefined,
		instrument: state.instrument,
	};
}

export function buildNameParams(
	state: NameUrlState
): Record<string, string | number | null | undefined> {
	return {
		tab: state.tab || undefined,
		capo: state.capo > 0 ? state.capo : undefined,
		startFret: state.startFret > 0 ? state.startFret : undefined,
		instrument: state.instrument,
	};
}

export function buildProgressionParams(
	state: ProgressionUrlState
): Record<string, string | number | null | undefined> {
	return {
		chords: state.chords || undefined,
		limit: state.limit !== PROGRESSION_DEFAULTS.limit ? state.limit : undefined,
		maxDistance:
			state.maxDistance !== PROGRESSION_DEFAULTS.maxDistance ? state.maxDistance : undefined,
		capo: state.capo > 0 ? state.capo : undefined,
		context: state.context !== PROGRESSION_DEFAULTS.context ? state.context : undefined,
		instrument: state.instrument,
	};
}

// =============================================================================
// Filter Counting (for UI badges)
// =============================================================================

export function countFindFilters(state: FindUrlState): number {
	return [
		state.capo > 0,
		state.limit !== FIND_DEFAULTS.limit,
		state.voicing !== FIND_DEFAULTS.voicing,
		state.position !== null,
		state.context === 'band',
	].filter(Boolean).length;
}

export function countProgressionFilters(state: ProgressionUrlState): number {
	return [
		state.capo > 0,
		state.limit !== PROGRESSION_DEFAULTS.limit,
		state.maxDistance !== PROGRESSION_DEFAULTS.maxDistance,
		state.context === 'band',
	].filter(Boolean).length;
}
