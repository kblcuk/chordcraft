/**
 * WASM wrapper for ChordCraft
 * Provides typed interfaces for Rust WASM functions
 */

import init, {
	analyzeChord as wasmAnalyzeChord,
	findFingerings as wasmFindFingerings,
	generateProgression as wasmGenerateProgression,
} from 'chordcraft-wasm';

// ============================================================================
// Types (matching Rust WASM types)
// ============================================================================

export interface ScoredFingering {
	tab: string;
	score: number;
	voicingType: 'core' | 'full' | 'jazzy';
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
// WASM Initialization
// ============================================================================

let wasmInitialized = false;

export async function initializeWasm(): Promise<void> {
	if (!wasmInitialized) {
		await init();
		wasmInitialized = true;
	}
}

// ============================================================================
// API Functions
// ============================================================================

/**
 * Find fingerings for a chord
 */
export async function findFingerings(
	chordName: string,
	options?: GeneratorOptions
): Promise<ScoredFingering[]> {
	await initializeWasm();

	try {
		const result = wasmFindFingerings(
			chordName,
			'guitar', // Currently only guitar supported
			options || null
		);
		return result as ScoredFingering[];
	} catch (error) {
		console.error('Error finding fingerings:', error);
		throw new Error(`Failed to find fingerings for "${chordName}": ${error}`);
	}
}

/**
 * Analyze a fingering and identify possible chords
 */
export async function analyzeChord(tabNotation: string): Promise<ChordMatch[]> {
	await initializeWasm();

	try {
		const result = wasmAnalyzeChord(
			tabNotation,
			'guitar' // Currently only guitar supported
		);
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
	options?: ProgressionOptions
): Promise<ProgressionSequence[]> {
	await initializeWasm();

	try {
		const result = wasmGenerateProgression(
			chordNames,
			'guitar', // Currently only guitar supported
			options || null
		);
		return result as ProgressionSequence[];
	} catch (error) {
		console.error('Error generating progression:', error);
		throw new Error(`Failed to generate progression: ${error}`);
	}
}
