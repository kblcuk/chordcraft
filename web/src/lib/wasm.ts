/**
 * WASM wrapper for ChordCraft
 * Provides typed interfaces for Rust WASM functions
 */

import init, {
	analyzeChord as wasmAnalyzeChord,
	findFingerings as wasmFindFingerings,
	generateProgression as wasmGenerateProgression,
	getInstrumentInfo as wasmGetInstrumentInfo,
} from 'chordcraft-wasm';

// ============================================================================
// Types (matching Rust WASM types)
// ============================================================================

/** Supported instrument types */
export type Instrument =
	| 'guitar'
	| 'ukulele'
	| 'baritone-ukulele'
	| 'bass'
	| 'bass-5'
	| 'mandolin'
	| 'banjo'
	| 'guitar-7'
	| 'drop-d'
	| 'open-g'
	| 'dadgad';

/** Instrument configuration info */
export interface InstrumentInfo {
	stringCount: number;
	stringNames: string[];
}

export interface ScoredFingering {
	tab: string;
	score: number;
	voicingType: 'core' | 'full' | 'jazzy' | 'incomplete';
	hasRootInBass: boolean;
	position: number;
	notes: string[];
}

export interface ChordMatch {
	name: string;
	confidence: number;
	explanation: string;
}

export interface ChordTransition {
	fromChord: string;
	toChord: string;
	fromFingering: ScoredFingering;
	toFingering: ScoredFingering;
	score: number;
	fingerMovements: number;
	commonAnchors: number;
	positionDistance: number;
}

export interface ProgressionSequence {
	chords: string[];
	fingerings: ScoredFingering[];
	transitions: ChordTransition[];
	totalScore: number;
	avgTransitionScore: number;
}

export interface GeneratorOptions {
	limit?: number;
	preferredPosition?: number;
	voicingType?: 'core' | 'full' | 'jazzy';
	rootInBass?: boolean;
	maxFret?: number;
	playingContext?: 'solo' | 'band';
	capo?: number;
}

export interface ProgressionOptions {
	limit?: number;
	maxFretDistance?: number;
	candidatesPerChord?: number;
	generatorOptions?: GeneratorOptions;
}

// ============================================================================
// Instrument Metadata
// ============================================================================

/** Instrument metadata for UI display */
export const INSTRUMENT_METADATA: Record<
	Instrument,
	{
		label: string;
		category: 'standard' | 'alternate-tuning';
		stringCount: number;
		tuning: string;
		description?: string;
	}
> = {
	guitar: {
		label: 'Guitar (Standard)',
		category: 'standard',
		stringCount: 6,
		tuning: 'EADGBE',
		description: 'Standard 6-string guitar',
	},
	ukulele: {
		label: 'Ukulele',
		category: 'standard',
		stringCount: 4,
		tuning: 'GCEA',
		description: 'Standard soprano ukulele',
	},
	'baritone-ukulele': {
		label: 'Baritone Ukulele',
		category: 'standard',
		stringCount: 4,
		tuning: 'DGBE',
		description: 'Larger ukulele, same as guitar top 4 strings',
	},
	bass: {
		label: 'Bass (4-string)',
		category: 'standard',
		stringCount: 4,
		tuning: 'EADG',
		description: 'Standard 4-string bass guitar',
	},
	'bass-5': {
		label: 'Bass (5-string)',
		category: 'standard',
		stringCount: 5,
		tuning: 'BEADG',
		description: '5-string bass with low B',
	},
	mandolin: {
		label: 'Mandolin',
		category: 'standard',
		stringCount: 4,
		tuning: 'GDAE',
		description: 'Standard mandolin tuning',
	},
	banjo: {
		label: 'Banjo (5-string)',
		category: 'standard',
		stringCount: 5,
		tuning: 'gDGBD',
		description: 'Standard 5-string banjo with high G drone',
	},
	'guitar-7': {
		label: '7-String Guitar',
		category: 'alternate-tuning',
		stringCount: 7,
		tuning: 'BEADGBE',
		description: 'Extended range guitar with low B',
	},
	'drop-d': {
		label: 'Drop D Guitar',
		category: 'alternate-tuning',
		stringCount: 6,
		tuning: 'DADGBE',
		description: 'Guitar with low E tuned down to D',
	},
	'open-g': {
		label: 'Open G Guitar',
		category: 'alternate-tuning',
		stringCount: 6,
		tuning: 'DGDGBD',
		description: 'Open G tuning for slide guitar',
	},
	dadgad: {
		label: 'DADGAD Guitar',
		category: 'alternate-tuning',
		stringCount: 6,
		tuning: 'DADGAD',
		description: 'Celtic/modal tuning',
	},
};

/** Instrument groupings for UI */
export const INSTRUMENT_CATEGORIES = {
	standard: [
		'guitar',
		'ukulele',
		'baritone-ukulele',
		'bass',
		'bass-5',
		'mandolin',
		'banjo',
	] as const,
	'alternate-tuning': ['drop-d', 'open-g', 'dadgad', 'guitar-7'] as const,
} as const;

/** Helper: Get display label for instrument */
export function getInstrumentLabel(instrument: Instrument): string {
	return INSTRUMENT_METADATA[instrument].label;
}

/** Helper: Get category for instrument */
export function getInstrumentCategory(instrument: Instrument): 'standard' | 'alternate-tuning' {
	return INSTRUMENT_METADATA[instrument].category;
}

// ============================================================================
// WASM Initialization
// ============================================================================

let wasmInitialized = false;
let wasmInitPromise: Promise<void> | null = null;

/**
 * Initialize WASM module
 * Uses promise caching to prevent race conditions and duplicate loads
 */
export async function initializeWasm(): Promise<void> {
	// If already initialized, return immediately
	if (wasmInitialized) {
		return;
	}

	// If initialization is in progress, wait for it
	if (wasmInitPromise) {
		await wasmInitPromise;
		return;
	}

	// Start initialization and cache the promise
	wasmInitPromise = (async () => {
		try {
			await init();
			wasmInitialized = true;
		} catch (error) {
			console.error('Error initializing WASM:', error);
			throw error;
		} finally {
			wasmInitPromise = null; // Reset promise after completion
		}
	})();

	return wasmInitPromise;
}

// ============================================================================
// API Functions
// ============================================================================

// Cache for instrument info to avoid repeated WASM calls
const instrumentInfoCache: Map<Instrument, InstrumentInfo> = new Map();

/**
 * Get instrument configuration info (string count, names)
 * Results are cached to avoid repeated WASM calls
 */
export async function getInstrumentInfo(instrument: Instrument): Promise<InstrumentInfo> {
	await initializeWasm();

	const cached = instrumentInfoCache.get(instrument);
	if (cached) return cached;

	try {
		const result = wasmGetInstrumentInfo(instrument);
		instrumentInfoCache.set(instrument, result as InstrumentInfo);
		return result as InstrumentInfo;
	} catch (error) {
		console.error('Error getting instrument info:', error);
		throw new Error(`Failed to get instrument info for "${instrument}": ${error}`);
	}
}

/**
 * Find fingerings for a chord
 */
export async function findFingerings(
	chordName: string,
	instrument: Instrument = 'guitar',
	options?: GeneratorOptions
): Promise<ScoredFingering[]> {
	await initializeWasm();

	try {
		const result = wasmFindFingerings(chordName, instrument, options || null);
		return result as ScoredFingering[];
	} catch (error) {
		console.error('Error finding fingerings:', error);
		throw new Error(`Failed to find fingerings for "${chordName}": ${error}`);
	}
}

/**
 * Analyze a fingering and identify possible chords
 */
export async function analyzeChord(
	tabNotation: string,
	instrument: Instrument = 'guitar'
): Promise<ChordMatch[]> {
	await initializeWasm();

	try {
		const result = wasmAnalyzeChord(tabNotation, instrument);
		return result as ChordMatch[];
	} catch (error) {
		console.error('Error analyzing chord:', error);
		throw new Error(`Failed to analyze "${tabNotation}": ${error}`);
	}
}

/**
 * Generate optimal fingering progression for a chord sequence
 */
export async function generateProgression(
	chordNames: string[],
	instrument: Instrument = 'guitar',
	options?: ProgressionOptions
): Promise<ProgressionSequence[]> {
	await initializeWasm();

	try {
		const result = wasmGenerateProgression(chordNames, instrument, options || null);
		return result as ProgressionSequence[];
	} catch (error) {
		console.error('Error generating progression:', error);
		throw new Error(`Failed to generate progression: ${error}`);
	}
}
