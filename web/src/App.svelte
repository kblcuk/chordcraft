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
		if (!findChordInput.trim()) return;

		findLoading = true;
		findError = '';
		findResults = [];

		try {
			findResults = await findFingerings(findChordInput.trim(), { limit: 10 });
		} catch (error) {
			findError = error instanceof Error ? error.message : 'Unknown error';
		} finally {
			findLoading = false;
		}
	}

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
		if (!progressionInput.trim()) return;

		progressionLoading = true;
		progressionError = '';
		progressionResults = [];

		try {
			const chords = progressionInput.trim().split(/\s+/);
			progressionResults = await generateProgression(chords, { limit: 3 });
		} catch (error) {
			progressionError = error instanceof Error ? error.message : 'Unknown error';
		} finally {
			progressionLoading = false;
		}
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
				<p class="text-gray-600 mb-6">
					Enter a chord name to see all possible fingerings. Try: Cmaj7, Abm7, G7
				</p>

				<div class="space-y-4">
					<div>
						<label
							for="chord-input"
							class="block text-sm font-medium text-gray-700 mb-2"
						>
							Chord Name
						</label>
						<input
							id="chord-input"
							type="text"
							bind:value={findChordInput}
							onkeydown={(e) => e.key === 'Enter' && handleFind()}
							placeholder="e.g., Cmaj7, Abm7, G7"
							class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
							disabled={!wasmReady}
						/>
					</div>

					<button
						onclick={handleFind}
						disabled={!wasmReady || findLoading || !findChordInput.trim()}
						class="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed"
					>
						{findLoading ? 'Finding...' : 'Find Fingerings'}
					</button>
				</div>

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
				<p class="text-gray-600 mb-6">
					Enter tab notation to identify a chord. Try: x32010, 022100, 320003
				</p>

				<div class="space-y-4">
					<div>
						<label for="tab-input" class="block text-sm font-medium text-gray-700 mb-2">
							Tab Notation
						</label>
						<input
							id="tab-input"
							type="text"
							bind:value={nameTabInput}
							onkeydown={(e) => e.key === 'Enter' && handleAnalyze()}
							placeholder="e.g., x32010"
							class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
							disabled={!wasmReady}
						/>
					</div>

					<button
						onclick={handleAnalyze}
						disabled={!wasmReady || nameLoading || !nameTabInput.trim()}
						class="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed"
					>
						{nameLoading ? 'Analyzing...' : 'Identify Chord'}
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
				<p class="text-gray-600 mb-6">
					Enter a sequence of chords to find optimal fingering transitions. Try: C Am F G
				</p>

				<div class="space-y-4">
					<div>
						<label
							for="progression-input"
							class="block text-sm font-medium text-gray-700 mb-2"
						>
							Chord Progression (space-separated)
						</label>
						<input
							id="progression-input"
							type="text"
							bind:value={progressionInput}
							onkeydown={(e) => e.key === 'Enter' && handleProgression()}
							placeholder="e.g., Cmaj7 Am7 Dm7 G7"
							class="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
							disabled={!wasmReady}
						/>
					</div>

					<button
						onclick={handleProgression}
						disabled={!wasmReady || progressionLoading || !progressionInput.trim()}
						class="px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed"
					>
						{progressionLoading ? 'Generating...' : 'Generate Progression'}
					</button>
				</div>

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
