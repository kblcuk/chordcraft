<script lang="ts">
	import { page } from '$app/state';
	import { parseNameParams, buildNameParams, updateUrl } from '$lib/utils/url-state';
	import {
		analyzeChord,
		getInstrumentInfo,
		type ChordMatch,
		type InstrumentInfo,
	} from '$lib/wasm';
	import Form from '$lib/components/features/name/Form.svelte';
	import Results from '$lib/components/features/name/Results.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';
	import InteractiveChordDiagram from '$lib/components/features/name/InteractiveChordDiagram.svelte';
	import FingerCountBadge from '$lib/components/features/name/FingerCountBadge.svelte';
	import { Label } from '$lib/components/ui/label';
	import ShareButton from '$lib/components/shared/ShareButton.svelte';

	// Derive all input state from URL (single source of truth)
	const urlState = $derived(parseNameParams(page.url.searchParams));

	// Local state for results (not in URL)
	let results = $state<ChordMatch[]>([]);
	let loading = $state(false);
	let error = $state('');

	// Instrument info for string count (cached)
	let instrumentInfo = $state<InstrumentInfo | null>(null);
	const stringCount = $derived(instrumentInfo?.stringCount ?? 6);
	const stringNames = $derived(instrumentInfo?.stringNames ?? ['E', 'A', 'D', 'G', 'B', 'e']);

	// Local input values for controlled components
	let tabInput = $state('000000');
	let startFret = $state(0);

	// Track last analysis params to detect meaningful changes
	let lastAnalysisKey = '';

	// Load instrument info when instrument changes
	$effect(() => {
		const instrument = urlState.instrument;
		getInstrumentInfo(instrument).then((info) => {
			instrumentInfo = info;
		});
	});

	// Sync local inputs with URL state (for browser back/forward)
	$effect(() => {
		// Only update if URL has a tab value
		if (urlState.tab) {
			tabInput = urlState.tab;
		}
		startFret = urlState.startFret;
	});

	// React to URL changes - trigger analysis when we have input
	$effect(() => {
		const { tab, instrument, capo } = urlState;

		// Create a key representing all analysis-relevant params
		const analysisKey = JSON.stringify({ tab, instrument, capo });

		// Only analyze if params changed and we have input
		if (analysisKey !== lastAnalysisKey && tab.trim()) {
			lastAnalysisKey = analysisKey;
			doAnalysis();
		}
	});

	async function doAnalysis() {
		const { tab, instrument } = urlState;

		if (!tab.trim() || loading) return;

		loading = true;
		error = '';

		try {
			const allResults = await analyzeChord(tab.trim(), instrument);
			results = allResults.slice(0, 5); // Top 5 results
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
			results = [];
		} finally {
			loading = false;
		}
	}

	// Watch for local tab changes and sync to URL (triggers analysis via effect)
	$effect(() => {
		if (tabInput !== urlState.tab && tabInput.trim()) {
			updateUrl(buildNameParams({ ...urlState, tab: tabInput }));
		}
	});

	// Watch for startFret changes and sync to URL
	$effect(() => {
		if (startFret !== urlState.startFret) {
			updateUrl(buildNameParams({ ...urlState, startFret }));
		}
	});

	// Actions
	function handleCapoChange(capo: number) {
		updateUrl(buildNameParams({ ...urlState, capo }));
	}
</script>

<div class="animate-fade-in rounded-xl border border-border bg-card p-6 shadow-warm sm:p-8">
	<!-- Header -->
	<div class="mb-6">
		<h2 class="font-display text-2xl font-bold tracking-tight text-foreground">Name Chord</h2>
		<p class="mt-1 text-muted-foreground">
			Identify a chord by tapping the fretboard or entering tab notation.
		</p>
	</div>

	<div class="space-y-6">
		<!-- Visual Input Section -->
		<div class="space-y-4">
			<div class="flex items-center justify-between">
				<h3 class="font-medium text-foreground">Visual Input</h3>

				<!-- Capo Selector -->
				<div class="flex items-center gap-2">
					<Label for="capo-select" class="text-sm font-medium">Capo:</Label>
					<select
						id="capo-select"
						value={urlState.capo}
						onchange={(e) => handleCapoChange(Number(e.currentTarget.value))}
						class="rounded-lg border border-border bg-card px-3 py-1.5 text-sm shadow-warm-sm transition-all duration-200 focus:border-primary focus:ring-2 focus:ring-ring/30 focus:outline-none"
					>
						<option value="0">None</option>
						{#each Array.from({ length: 12 }, (v, i) => i + 1) as fretNum (fretNum)}
							<option value={fretNum}>Fret {fretNum}</option>
						{/each}
					</select>
					{#if urlState.capo > 0}
						<span class="text-xs text-muted-foreground">
							(+{urlState.capo} semitones)
						</span>
					{/if}
				</div>
			</div>

			<!-- Interactive Fretboard -->
			<div class="flex justify-center">
				<InteractiveChordDiagram
					bind:value={tabInput}
					bind:startFret
					capo={urlState.capo}
					size="large"
					{stringCount}
					{stringNames}
				/>
			</div>
		</div>

		<!-- Divider -->
		<div class="relative">
			<div class="absolute inset-0 flex items-center">
				<div class="w-full border-t border-border"></div>
			</div>
			<div class="relative flex justify-center text-xs uppercase">
				<span class="bg-card px-3 text-muted-foreground">Or</span>
			</div>
		</div>

		<!-- Text Input Section -->
		<div class="space-y-2">
			<h3 class="font-medium text-foreground">Text Input</h3>
			<Form bind:value={tabInput} disabled={false} />
		</div>

		<!-- Share Button -->
		{#if tabInput}
			<div class="flex justify-end">
				<ShareButton url={page.url.href} title="Share Url" />
			</div>
		{/if}
	</div>

	<!-- Error -->
	{#if error}
		<div class="mt-6">
			<ErrorAlert message={error} />
		</div>
	{/if}

	<!-- Results -->
	{#if results.length > 0}
		<div class="mt-6 space-y-4">
			<!-- Finger Count Badge -->
			<div class="flex justify-center">
				<FingerCountBadge tab={urlState.tab} />
			</div>

			<Results matches={results} />
		</div>
	{/if}
</div>
