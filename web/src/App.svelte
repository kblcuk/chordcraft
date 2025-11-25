<script lang="ts">
	import { onMount } from 'svelte';

	import {
		findFingerings,
		analyzeChord,
		generateProgression,
		initializeWasm,
		type ScoredFingering,
		type ChordMatch,
		type ProgressionSequence,
	} from './lib/wasm';
	import ChordDiagram from './lib/ChordDiagram.svelte';

	type Mode = 'find' | 'name' | 'progression';
	let currentMode: Mode = 'find';

	// Find mode state
	let findChordInput = '';
	let findResults: ScoredFingering[] = [];
	let findLoading = false;
	let findError = '';
	let findHasSearched = false; // Track if user has performed initial search

	// Advanced options state (Find mode)
	let showAdvancedFind = false;
	let findLimit = 10;
	let findCapo = 0;
	let findVoicing: 'all' | 'core' | 'full' | 'jazzy' = 'all';
	let findPosition: number | null = null; // null = any position
	let findContext: 'solo' | 'band' = 'solo';

	// Name mode state
	let nameTabInput = '';
	let nameResults: ChordMatch[] = [];
	let nameLoading = false;
	let nameError = '';

	// Progression mode state
	let progressionInput = '';
	let progressionResults: ProgressionSequence[] = [];
	let progressionLoading = false;
	let progressionError = '';
	let progressionHasSearched = false; // Track if user has performed initial search

	// Advanced options state (Progression mode)
	let showAdvancedProgression = false;
	let progressionLimit = 3;
	let progressionMaxDistance = 3;
	let progressionCapo = 0;
	let progressionContext: 'solo' | 'band' = 'solo';

	// Initialize WASM on mount
	let wasmReady = false;
	onMount(async () => {
		try {
			await initializeWasm();
			wasmReady = true;
		} catch (error) {
			console.error('Failed to initialize WASM:', error);
		}
	});

	// Find fingerings handler
	async function handleFind() {
		if (!findChordInput.trim() || findLoading) return; // Guard against concurrent calls

		findLoading = true;
		findError = '';
		findResults = [];

		try {
			const voicingType = findVoicing === 'all' ? undefined : findVoicing;

			findResults = await findFingerings(findChordInput.trim(), {
				limit: findLimit,
				capo: findCapo,
				voicingType,
				preferredPosition: findPosition ?? undefined,
				playingContext: findContext,
			});

			findHasSearched = true; // Mark that initial search is done
		} catch (error) {
			findError = error instanceof Error ? error.message : 'Unknown error';
		} finally {
			findLoading = false;
		}
	}

	// Track previous filter values to detect actual changes
	let prevFindLimit = findLimit;
	let prevFindCapo = findCapo;
	let prevFindVoicing = findVoicing;
	let prevFindPosition = findPosition;
	let prevFindContext = findContext;

	// Auto-execute when filters change (after initial search)
	$: {
		if (findHasSearched) {
			const filterChanged =
				prevFindLimit !== findLimit ||
				prevFindCapo !== findCapo ||
				prevFindVoicing !== findVoicing ||
				prevFindPosition !== findPosition ||
				prevFindContext !== findContext;

			if (filterChanged) {
				prevFindLimit = findLimit;
				prevFindCapo = findCapo;
				prevFindVoicing = findVoicing;
				prevFindPosition = findPosition;
				prevFindContext = findContext;
				handleFind();
			}
		}
	}

	// Reset find options to defaults
	function resetFindOptions() {
		findLimit = 10;
		findCapo = 0;
		findVoicing = 'all';
		findPosition = null;
		findContext = 'solo';
	}

	// Count active filters (for badge)
	$: activeFindFilters = [
		findCapo > 0 && 'Capo',
		findLimit !== 10 && 'Limit',
		findVoicing !== 'all' && 'Voicing',
		findPosition !== null && 'Position',
		findContext === 'band' && 'Band Mode',
	].filter(Boolean).length;

	// Analyze chord handler
	async function handleAnalyze() {
		if (!nameTabInput.trim()) return;

		nameLoading = true;
		nameError = '';
		nameResults = [];

		try {
			nameResults = await analyzeChord(nameTabInput.trim());
		} catch (error) {
			nameError = error instanceof Error ? error.message : 'Unknown error';
		} finally {
			nameLoading = false;
		}
	}

	// Generate progression handler
	async function handleProgression() {
		if (!progressionInput.trim() || progressionLoading) return; // Guard against concurrent calls

		progressionLoading = true;
		progressionError = '';
		progressionResults = [];

		try {
			const chords = progressionInput.trim().split(/\s+/);
			progressionResults = await generateProgression(chords, {
				limit: progressionLimit,
				maxFretDistance: progressionMaxDistance,
				generatorOptions: {
					capo: progressionCapo,
					playingContext: progressionContext,
				},
			});

			progressionHasSearched = true; // Mark that initial search is done
		} catch (error) {
			progressionError = error instanceof Error ? error.message : 'Unknown error';
		} finally {
			progressionLoading = false;
		}
	}

	// Track previous filter values to detect actual changes
	let prevProgressionLimit = progressionLimit;
	let prevProgressionMaxDistance = progressionMaxDistance;
	let prevProgressionCapo = progressionCapo;
	let prevProgressionContext = progressionContext;

	// Auto-execute when filters change (after initial search)
	$: {
		if (progressionHasSearched) {
			const filterChanged =
				prevProgressionLimit !== progressionLimit ||
				prevProgressionMaxDistance !== progressionMaxDistance ||
				prevProgressionCapo !== progressionCapo ||
				prevProgressionContext !== progressionContext;

			if (filterChanged) {
				prevProgressionLimit = progressionLimit;
				prevProgressionMaxDistance = progressionMaxDistance;
				prevProgressionCapo = progressionCapo;
				prevProgressionContext = progressionContext;
				handleProgression();
			}
		}
	}

	// Reset progression options to defaults
	function resetProgressionOptions() {
		progressionLimit = 3;
		progressionMaxDistance = 3;
		progressionCapo = 0;
		progressionContext = 'solo';
	}

	// Count active filters for progression
	$: activeProgressionFilters = [
		progressionCapo > 0 && 'Capo',
		progressionLimit !== 3 && 'Limit',
		progressionMaxDistance !== 3 && 'Max Distance',
		progressionContext === 'band' && 'Band Mode',
	].filter(Boolean).length;

	// Example chords for Find mode
	const exampleChords = ['C', 'Cmaj7', 'Fm7', 'Abm7', 'F#7b9', 'Dsus4'];

	// Example tabs for Name mode
	const exampleTabs = [
		{ tab: 'x32010', label: 'C' },
		{ tab: '022100', label: 'E' },
		{ tab: '133211', label: 'F (barre)' },
		{ tab: 'xx0232', label: 'D' },
		{ tab: '320003', label: 'G' },
	];

	// Common progressions
	const commonProgressions = [
		{ name: 'I-IV-V in C', chords: 'C F G' },
		{ name: 'I-V-vi-IV', chords: 'C G Am F' },
		{ name: 'ii-V-I Jazz', chords: 'Dm7 G7 Cmaj7' },
		{ name: 'I-vi-IV-V', chords: 'C Am F G' },
		{ name: '12-Bar Blues', chords: 'C7 F7 C7 G7' },
		{ name: 'Coltrane Changes', chords: 'Cmaj7 Ebmaj7 Abmaj7 Bmaj7' },
	];

	// Helper functions to load examples
	function loadExampleChord(chord: string) {
		findChordInput = chord;
		handleFind();
	}

	function loadExampleTab(tab: string) {
		nameTabInput = tab;
		handleAnalyze();
	}

	function loadProgression(progression: string) {
		progressionInput = progression;
		handleProgression();
	}

	function clearFindInput() {
		findChordInput = '';
		findResults = [];
		findError = '';
		findHasSearched = false; // Reset search flag
	}

	function clearNameInput() {
		nameTabInput = '';
		nameResults = [];
		nameError = '';
	}

	function clearProgressionInput() {
		progressionInput = '';
		progressionResults = [];
		progressionError = '';
		progressionHasSearched = false; // Reset search flag
	}
</script>

<main class="min-h-screen bg-gray-50">
	<!-- Header -->
	<header class="bg-white shadow-sm border-b">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
			<h1 class="text-3xl font-bold text-gray-900">ChordCraft</h1>
			<p class="mt-1 text-sm text-gray-500">
				Chord-Fingering Conversion Tool
				{#if wasmReady}
					<span class="text-green-600">● WASM Ready</span>
				{:else}
					<span class="text-yellow-600">● Loading...</span>
				{/if}
			</p>
		</div>
	</header>

	<!-- Mode Switcher -->
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
		<nav class="flex space-x-2 bg-white rounded-lg p-1 shadow-sm border">
			<button
				onclick={() => (currentMode = 'find')}
				class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors {currentMode ===
				'find'
					? 'bg-blue-600 text-white'
					: 'text-gray-600 hover:bg-gray-100'}"
			>
				Find Fingerings
			</button>
			<button
				onclick={() => (currentMode = 'name')}
				class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors {currentMode ===
				'name'
					? 'bg-blue-600 text-white'
					: 'text-gray-600 hover:bg-gray-100'}"
			>
				Name Chord
			</button>
			<button
				onclick={() => (currentMode = 'progression')}
				class="flex-1 px-4 py-2 text-sm font-medium rounded-md transition-colors {currentMode ===
				'progression'
					? 'bg-blue-600 text-white'
					: 'text-gray-600 hover:bg-gray-100'}"
			>
				Progression
			</button>
		</nav>
	</div>

	<!-- Content Area -->
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
		{#if currentMode === 'find'}
			<div class="bg-white rounded-lg shadow-sm border p-6">
				<h2 class="text-xl font-semibold text-gray-900 mb-4">Find Fingerings</h2>
				<p class="text-gray-600 mb-4">Enter a chord name to see all possible fingerings.</p>

				<!-- Example Chords -->
				<div class="mb-6">
					<p class="text-sm font-medium text-gray-700 mb-2">Quick Examples:</p>
					<div class="flex flex-wrap gap-2">
						{#each exampleChords as chord}
							<button
								onclick={() => loadExampleChord(chord)}
								disabled={!wasmReady || findLoading}
								class="px-3 py-1.5 text-sm bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
							>
								{chord}
							</button>
						{/each}
					</div>
				</div>

				<div class="space-y-4">
					<div>
						<label
							for="chord-input"
							class="block text-sm font-medium text-gray-700 mb-2"
						>
							Chord Name
						</label>
						<div class="flex gap-2">
							<input
								id="chord-input"
								type="text"
								bind:value={findChordInput}
								onkeydown={(e) => e.key === 'Enter' && handleFind()}
								onblur={handleFind}
								placeholder="e.g., Cmaj7, Abm7, G7"
								class="flex-1 px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
								disabled={!wasmReady}
							/>
							{#if findChordInput}
								<button
									onclick={clearFindInput}
									class="px-3 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
									title="Clear input"
								>
									<svg
										class="w-5 h-5 text-gray-500"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M6 18L18 6M6 6l12 12"
										/>
									</svg>
								</button>
							{/if}
						</div>
						<p class="text-xs text-gray-500 mt-1">
							Press Enter or click away to search
						</p>
					</div>

					<div class="flex gap-3">
						<button
							onclick={handleFind}
							disabled={!wasmReady || findLoading || !findChordInput.trim()}
							class="px-4 py-1.5 text-sm border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
						>
							{findLoading ? 'Finding...' : 'Search'}
						</button>

						<button
							onclick={() => (showAdvancedFind = !showAdvancedFind)}
							class="px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors flex items-center gap-2"
						>
							<span>Advanced</span>
							{#if activeFindFilters > 0}
								<span
									class="bg-blue-600 text-white text-xs font-semibold px-2 py-0.5 rounded-full"
								>
									{activeFindFilters}
								</span>
							{/if}
							<svg
								class="w-4 h-4 transition-transform {showAdvancedFind
									? 'rotate-180'
									: ''}"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M19 9l-7 7-7-7"
								/>
							</svg>
						</button>
					</div>
				</div>

				<!-- Advanced Options -->
				{#if showAdvancedFind}
					<div class="mt-6 p-6 bg-gray-50 rounded-lg border border-gray-200 space-y-6">
						<div class="flex justify-between items-center mb-4">
							<h3 class="text-lg font-semibold text-gray-900">Advanced Options</h3>
							<button
								onclick={resetFindOptions}
								class="text-sm text-gray-600 hover:text-gray-900 underline"
							>
								Reset to defaults
							</button>
						</div>

						<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
							<!-- Limit Slider -->
							<div>
								<label
									for="find-limit"
									class="block text-sm font-medium text-gray-700 mb-2"
								>
									Number of Fingerings: {findLimit}
								</label>
								<input
									id="find-limit"
									type="range"
									min="5"
									max="50"
									step="5"
									bind:value={findLimit}
									class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-600"
								/>
								<div class="flex justify-between text-xs text-gray-500 mt-1">
									<span>5</span>
									<span>50</span>
								</div>
							</div>

							<!-- Capo Selector -->
							<div>
								<label
									for="find-capo"
									class="block text-sm font-medium text-gray-700 mb-2"
								>
									Capo Position
								</label>
								<select
									id="find-capo"
									bind:value={findCapo}
									class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
								>
									<option value={0}>No capo</option>
									{#each Array(12).fill(0) as _, i}
										<option value={i + 1}>Fret {i + 1}</option>
									{/each}
								</select>
							</div>

							<!-- Position Preference -->
							<div>
								<label
									for="find-position"
									class="block text-sm font-medium text-gray-700 mb-2"
								>
									Preferred Position
								</label>
								<select
									id="find-position"
									bind:value={findPosition}
									class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
								>
									<option value={null}>Any position</option>
									<option value={0}>Open position (0-5)</option>
									{#each Array(12).fill(0) as _, i}
										<option value={i + 1}>Around fret {i + 1}</option>
									{/each}
								</select>
							</div>

							<!-- Playing Context -->
							<div>
								<div class="block text-sm font-medium text-gray-700 mb-2">
									Playing Context
								</div>
								<div class="flex gap-4">
									<label class="flex items-center cursor-pointer">
										<input
											type="radio"
											bind:group={findContext}
											value="solo"
											class="w-4 h-4 text-blue-600 focus:ring-blue-500"
										/>
										<span class="ml-2 text-sm text-gray-700">
											Solo
											<span class="text-gray-500">(full bass)</span>
										</span>
									</label>
									<label class="flex items-center cursor-pointer">
										<input
											type="radio"
											bind:group={findContext}
											value="band"
											class="w-4 h-4 text-blue-600 focus:ring-blue-500"
										/>
										<span class="ml-2 text-sm text-gray-700">
											Band
											<span class="text-gray-500">(lighter)</span>
										</span>
									</label>
								</div>
							</div>
						</div>

						<!-- Voicing Filter -->
						<div>
							<div class="block text-sm font-medium text-gray-700 mb-3">
								Voicing Type
							</div>
							<div class="grid grid-cols-2 gap-2">
								<label class="flex items-center cursor-pointer">
									<input
										type="radio"
										bind:group={findVoicing}
										value="all"
										class="w-4 h-4 text-blue-600 focus:ring-blue-500"
									/>
									<span class="ml-2 text-sm text-gray-700">
										All
										<span class="text-gray-500">(show everything)</span>
									</span>
								</label>
								<label class="flex items-center cursor-pointer">
									<input
										type="radio"
										bind:group={findVoicing}
										value="core"
										class="w-4 h-4 text-blue-600 focus:ring-blue-500"
									/>
									<span class="ml-2 text-sm text-gray-700">
										Core
										<span class="text-gray-500">(essential)</span>
									</span>
								</label>
								<label class="flex items-center cursor-pointer">
									<input
										type="radio"
										bind:group={findVoicing}
										value="full"
										class="w-4 h-4 text-blue-600 focus:ring-blue-500"
									/>
									<span class="ml-2 text-sm text-gray-700">
										Full
										<span class="text-gray-500">(complete)</span>
									</span>
								</label>
								<label class="flex items-center cursor-pointer">
									<input
										type="radio"
										bind:group={findVoicing}
										value="jazzy"
										class="w-4 h-4 text-blue-600 focus:ring-blue-500"
									/>
									<span class="ml-2 text-sm text-gray-700">
										Jazzy
										<span class="text-gray-500">(extended)</span>
									</span>
								</label>
							</div>
						</div>
					</div>
				{/if}

				{#if findError}
					<div class="mt-6 p-4 bg-red-50 rounded-md border border-red-200">
						<p class="text-sm text-red-700">{findError}</p>
					</div>
				{/if}

				{#if findResults.length > 0}
					<div class="mt-6 space-y-6">
						<h3 class="text-lg font-medium text-gray-900">
							Found {findResults.length} fingerings:
						</h3>
						<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
							{#each findResults as fingering, i}
								<div
									class="bg-white rounded-lg border-2 border-gray-200 p-4 hover:border-blue-400 transition-colors"
								>
									<!-- Chord Diagram -->
									<div class="flex justify-center mb-3">
										<ChordDiagram
											tab={fingering.tab}
											notes={fingering.notes}
											rootNote={fingering.notes[0] || ''}
											size="medium"
										/>
									</div>

									<!-- Tab Notation -->
									<div class="text-center mb-2">
										<code
											class="text-lg font-bold font-mono bg-gray-100 px-3 py-1 rounded"
										>
											{fingering.tab}
										</code>
									</div>

									<!-- Metadata -->
									<div class="flex flex-wrap gap-2 justify-center mb-2">
										<span
											class="px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded"
										>
											{fingering.voicingType}
										</span>
										<span
											class="px-2 py-1 text-xs font-medium bg-gray-100 text-gray-700 rounded"
										>
											Score: {fingering.score}
										</span>
										<span
											class="px-2 py-1 text-xs font-medium bg-gray-100 text-gray-700 rounded"
										>
											Fret {fingering.position}
										</span>
									</div>

									<!-- Notes and Root in Bass -->
									<div class="text-xs text-center text-gray-600 space-y-1">
										<div>Notes: {fingering.notes.join(', ')}</div>
										<div>
											{#if fingering.hasRootInBass}
												<span class="text-green-600">✓ Root in bass</span>
											{:else}
												<span class="text-gray-400">No root in bass</span>
											{/if}
										</div>
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		{:else if currentMode === 'name'}
			<div class="bg-white rounded-lg shadow-sm border p-6">
				<h2 class="text-xl font-semibold text-gray-900 mb-4">Name Chord</h2>
				<p class="text-gray-600 mb-4">Enter tab notation to identify a chord.</p>

				<!-- Example Tabs -->
				<div class="mb-6">
					<p class="text-sm font-medium text-gray-700 mb-2">Quick Examples:</p>
					<div class="flex flex-wrap gap-2">
						{#each exampleTabs as example}
							<button
								onclick={() => loadExampleTab(example.tab)}
								disabled={!wasmReady || nameLoading}
								class="px-3 py-1.5 text-sm bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
							>
								<span class="font-mono">{example.tab}</span>
								<span class="text-gray-500 ml-1">({example.label})</span>
							</button>
						{/each}
					</div>
				</div>

				<div class="space-y-4">
					<div>
						<label for="tab-input" class="block text-sm font-medium text-gray-700 mb-2">
							Tab Notation
						</label>
						<div class="flex gap-2">
							<input
								id="tab-input"
								type="text"
								bind:value={nameTabInput}
								onkeydown={(e) => e.key === 'Enter' && handleAnalyze()}
								onblur={handleAnalyze}
								placeholder="e.g., x32010"
								class="flex-1 px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 font-mono"
								disabled={!wasmReady}
							/>
							{#if nameTabInput}
								<button
									onclick={clearNameInput}
									class="px-3 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
									title="Clear input"
								>
									<svg
										class="w-5 h-5 text-gray-500"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M6 18L18 6M6 6l12 12"
										/>
									</svg>
								</button>
							{/if}
						</div>
						<p class="text-xs text-gray-500 mt-1">
							Press Enter or click away to identify
						</p>
					</div>

					<button
						onclick={handleAnalyze}
						disabled={!wasmReady || nameLoading || !nameTabInput.trim()}
						class="px-4 py-1.5 text-sm border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{nameLoading ? 'Analyzing...' : 'Identify'}
					</button>
				</div>

				{#if nameError}
					<div class="mt-6 p-4 bg-red-50 rounded-md border border-red-200">
						<p class="text-sm text-red-700">{nameError}</p>
					</div>
				{/if}

				{#if nameResults.length > 0}
					<div class="mt-6 space-y-3">
						<h3 class="text-lg font-medium text-gray-900">Possible matches:</h3>
						{#each nameResults as match, i}
							<div class="p-4 bg-gray-50 rounded-md border border-gray-200">
								<div class="flex justify-between items-center">
									<div>
										<span class="text-xl font-bold text-gray-900"
											>{match.name}</span
										>
										<span class="ml-3 text-sm text-gray-600"
											>{match.confidence}% confidence</span
										>
									</div>
									{#if i === 0}
										<span
											class="px-2 py-1 text-xs font-medium bg-green-100 text-green-800 rounded"
										>
											Best Match
										</span>
									{/if}
								</div>
								<p class="mt-2 text-sm text-gray-600">{match.explanation}</p>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{:else if currentMode === 'progression'}
			<div class="bg-white rounded-lg shadow-sm border p-6">
				<h2 class="text-xl font-semibold text-gray-900 mb-4">Chord Progression</h2>
				<p class="text-gray-600 mb-4">
					Enter a sequence of chords to find optimal fingering transitions.
				</p>

				<!-- Common Progressions -->
				<div class="mb-6">
					<p class="text-sm font-medium text-gray-700 mb-2">Common Progressions:</p>
					<div class="flex flex-wrap gap-2">
						{#each commonProgressions as progression}
							<button
								onclick={() => loadProgression(progression.chords)}
								disabled={!wasmReady || progressionLoading}
								class="px-3 py-1.5 text-sm bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
							>
								{progression.name}
							</button>
						{/each}
					</div>
				</div>

				<div class="space-y-4">
					<div>
						<label
							for="progression-input"
							class="block text-sm font-medium text-gray-700 mb-2"
						>
							Chord Progression (space-separated)
						</label>
						<div class="flex gap-2">
							<input
								id="progression-input"
								type="text"
								bind:value={progressionInput}
								onkeydown={(e) => e.key === 'Enter' && handleProgression()}
								onblur={handleProgression}
								placeholder="e.g., Cmaj7 Am7 Dm7 G7"
								class="flex-1 px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
								disabled={!wasmReady}
							/>
							{#if progressionInput}
								<button
									onclick={clearProgressionInput}
									class="px-3 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
									title="Clear input"
								>
									<svg
										class="w-5 h-5 text-gray-500"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M6 18L18 6M6 6l12 12"
										/>
									</svg>
								</button>
							{/if}
						</div>
						<p class="text-xs text-gray-500 mt-1">
							Press Enter or click away to generate
						</p>
					</div>

					<div class="flex gap-3">
						<button
							onclick={handleProgression}
							disabled={!wasmReady || progressionLoading || !progressionInput.trim()}
							class="px-4 py-1.5 text-sm border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
						>
							{progressionLoading ? 'Generating...' : 'Generate'}
						</button>

						<button
							onclick={() => (showAdvancedProgression = !showAdvancedProgression)}
							class="px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors flex items-center gap-2"
						>
							<span>Advanced</span>
							{#if activeProgressionFilters > 0}
								<span
									class="bg-blue-600 text-white text-xs font-semibold px-2 py-0.5 rounded-full"
								>
									{activeProgressionFilters}
								</span>
							{/if}
							<svg
								class="w-4 h-4 transition-transform {showAdvancedProgression
									? 'rotate-180'
									: ''}"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M19 9l-7 7-7-7"
								/>
							</svg>
						</button>
					</div>
				</div>

				<!-- Advanced Options -->
				{#if showAdvancedProgression}
					<div class="mt-6 p-6 bg-gray-50 rounded-lg border border-gray-200 space-y-6">
						<div class="flex justify-between items-center mb-4">
							<h3 class="text-lg font-semibold text-gray-900">Advanced Options</h3>
							<button
								onclick={resetProgressionOptions}
								class="text-sm text-gray-600 hover:text-gray-900 underline"
							>
								Reset to defaults
							</button>
						</div>

						<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
							<!-- Number of Alternatives -->
							<div>
								<label
									for="prog-limit"
									class="block text-sm font-medium text-gray-700 mb-2"
								>
									Number of Alternatives: {progressionLimit}
								</label>
								<input
									id="prog-limit"
									type="range"
									min="1"
									max="10"
									step="1"
									bind:value={progressionLimit}
									class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-600"
								/>
								<div class="flex justify-between text-xs text-gray-500 mt-1">
									<span>1</span>
									<span>10</span>
								</div>
							</div>

							<!-- Max Fret Distance -->
							<div>
								<label
									for="prog-distance"
									class="block text-sm font-medium text-gray-700 mb-2"
								>
									Max Fret Distance: {progressionMaxDistance}
								</label>
								<input
									id="prog-distance"
									type="range"
									min="1"
									max="12"
									step="1"
									bind:value={progressionMaxDistance}
									class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-600"
								/>
								<div class="flex justify-between text-xs text-gray-500 mt-1">
									<span>1 fret</span>
									<span>12 frets</span>
								</div>
								<p class="text-xs text-gray-500 mt-2">
									Maximum fret jump between consecutive fingerings
								</p>
							</div>

							<!-- Capo Selector -->
							<div>
								<label
									for="prog-capo"
									class="block text-sm font-medium text-gray-700 mb-2"
								>
									Capo Position
								</label>
								<select
									id="prog-capo"
									bind:value={progressionCapo}
									class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
								>
									<option value={0}>No capo</option>
									{#each Array(12).fill(0) as _, i}
										<option value={i + 1}>Fret {i + 1}</option>
									{/each}
								</select>
							</div>

							<!-- Playing Context -->
							<div>
								<div class="block text-sm font-medium text-gray-700 mb-2">
									Playing Context
								</div>
								<div class="flex gap-4">
									<label class="flex items-center cursor-pointer">
										<input
											type="radio"
											bind:group={progressionContext}
											value="solo"
											class="w-4 h-4 text-blue-600 focus:ring-blue-500"
										/>
										<span class="ml-2 text-sm text-gray-700">
											Solo
											<span class="text-gray-500">(full bass)</span>
										</span>
									</label>
									<label class="flex items-center cursor-pointer">
										<input
											type="radio"
											bind:group={progressionContext}
											value="band"
											class="w-4 h-4 text-blue-600 focus:ring-blue-500"
										/>
										<span class="ml-2 text-sm text-gray-700">
											Band
											<span class="text-gray-500">(lighter)</span>
										</span>
									</label>
								</div>
							</div>
						</div>
					</div>
				{/if}

				{#if progressionError}
					<div class="mt-6 p-4 bg-red-50 rounded-md border border-red-200">
						<p class="text-sm text-red-700">{progressionError}</p>
					</div>
				{/if}

				{#if progressionResults.length > 0}
					<div class="mt-6 space-y-8">
						{#each progressionResults as sequence, i}
							<div class="border-2 border-gray-300 rounded-lg p-6 bg-white">
								<div class="flex justify-between items-center mb-6">
									<h3 class="text-xl font-bold text-gray-900">
										Alternative #{i + 1}
									</h3>
									<div class="text-sm">
										<span class="text-gray-600">Avg Transition:</span>
										<span class="ml-1 font-semibold text-blue-600">
											{sequence.avgTransitionScore.toFixed(1)}
										</span>
									</div>
								</div>

								<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
									{#each sequence.fingerings as fingering, j}
										<div class="relative">
											<div
												class="bg-gray-50 rounded-lg p-4 border-2 border-gray-200"
											>
												<!-- Chord Name -->
												<div class="text-center mb-3">
													<h4 class="text-lg font-bold text-gray-900">
														{sequence.chords[j]}
													</h4>
												</div>

												<!-- Chord Diagram -->
												<div class="flex justify-center mb-3">
													<ChordDiagram
														tab={fingering.tab}
														notes={fingering.notes}
														rootNote={fingering.notes[0] || ''}
														size="small"
													/>
												</div>

												<!-- Tab Notation -->
												<div class="text-center mb-2">
													<code
														class="text-sm font-mono bg-white px-2 py-1 rounded border"
													>
														{fingering.tab}
													</code>
												</div>

												<!-- Metadata -->
												<div
													class="flex flex-wrap gap-1 justify-center text-xs"
												>
													<span
														class="px-2 py-0.5 bg-blue-100 text-blue-800 rounded"
													>
														{fingering.voicingType}
													</span>
													<span
														class="px-2 py-0.5 bg-gray-200 text-gray-700 rounded"
													>
														Fret {fingering.position}
													</span>
												</div>
											</div>

											<!-- Transition Arrow -->
											{#if j < sequence.transitions.length}
												<div
													class="absolute -right-3 top-1/2 -translate-y-1/2 z-10 hidden lg:block"
												>
													<div
														class="bg-white border-2 border-green-500 rounded-full p-2 shadow-md"
													>
														<svg
															class="w-5 h-5 text-green-600"
															fill="none"
															stroke="currentColor"
															viewBox="0 0 24 24"
														>
															<path
																stroke-linecap="round"
																stroke-linejoin="round"
																stroke-width="2"
																d="M13 7l5 5m0 0l-5 5m5-5H6"
															/>
														</svg>
													</div>
													<div class="mt-1 text-center">
														<div
															class="text-xs font-semibold text-green-600"
														>
															Score: {sequence.transitions[j].score}
														</div>
														<div class="text-xs text-gray-500">
															{sequence.transitions[j]
																.fingerMovements} move{sequence
																.transitions[j].fingerMovements !==
															1
																? 's'
																: ''}
														</div>
													</div>
												</div>

												<!-- Mobile Transition Info -->
												<div
													class="lg:hidden mt-3 p-2 bg-green-50 rounded border border-green-200 text-center"
												>
													<div class="text-sm text-green-700">
														→ Next: {sequence.transitions[j]
															.fingerMovements} finger{sequence
															.transitions[j].fingerMovements !== 1
															? 's'
															: ''} move,
														{sequence.transitions[j].commonAnchors} anchor{sequence
															.transitions[j].commonAnchors !== 1
															? 's'
															: ''}
														<span class="font-semibold"
															>(score: {sequence.transitions[j]
																.score})</span
														>
													</div>
												</div>
											{/if}
										</div>
									{/each}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{/if}
	</div>

	<!-- Footer -->
	<footer class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6 mt-12">
		<div class="text-center text-sm text-gray-500">
			<p>Built with Rust (WASM) + Svelte + Tailwind CSS</p>
			<p class="mt-1">
				<a
					href="https://github.com/kblcuk/chordcraft"
					class="text-blue-600 hover:text-blue-700"
				>
					View on GitHub
				</a>
			</p>
		</div>
	</footer>
</main>
