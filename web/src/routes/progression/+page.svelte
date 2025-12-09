<script lang="ts">
	import { page } from '$app/state';
	import {
		parseProgressionParams,
		buildProgressionParams,
		updateUrl,
		countProgressionFilters,
		PROGRESSION_DEFAULTS,
	} from '$lib/utils/url-state';
	import {
		generateProgression,
		getInstrumentInfo,
		type ProgressionSequence,
		type InstrumentInfo,
	} from '$lib/wasm';
	import Input from '$lib/components/features/progression/Input.svelte';
	import AdvancedOptions from '$lib/components/features/progression/AdvancedOptions.svelte';
	import Results from '$lib/components/features/progression/Results.svelte';
	import AdvancedOptionsWrapper from '$lib/components/shared/AdvancedOptionsWrapper.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';
	import ShareButton from '$lib/components/shared/ShareButton.svelte';
	import { Button } from '$lib/components/ui/button';
	import { commonProgressions } from '$lib/utils/examples';

	// Derive all input state from URL (single source of truth)
	const urlState = $derived(parseProgressionParams(page.url.searchParams));
	const activeFilters = $derived(countProgressionFilters(urlState));

	// Local state for results (not in URL)
	let results = $state<ProgressionSequence[]>([]);
	let loading = $state(false);
	let error = $state('');

	// Instrument info for string count (cached)
	let instrumentInfo = $state<InstrumentInfo | null>(null);
	const stringCount = $derived(instrumentInfo?.stringCount ?? 6);

	// Local input value for controlled component
	let progressionInput = $derived(urlState.chords);

	// Track last search params to detect meaningful changes
	let lastSearchKey = '';

	// Load instrument info when instrument changes
	$effect(() => {
		const instrument = urlState.instrument;
		getInstrumentInfo(instrument).then((info) => {
			instrumentInfo = info;
		});
	});

	// React to URL changes - trigger generation when we have input
	$effect(() => {
		const { chords, instrument, limit, maxDistance, capo, context } = urlState;

		// Create a key representing all search-relevant params
		const searchKey = JSON.stringify({ chords, instrument, limit, maxDistance, capo, context });

		// Only generate if params changed and we have input
		if (searchKey !== lastSearchKey && chords.trim()) {
			lastSearchKey = searchKey;
			doGenerate();
		}
	});

	async function doGenerate() {
		const { chords, instrument, limit, maxDistance, capo, context } = urlState;

		if (!chords.trim() || loading) return;

		loading = true;
		error = '';

		try {
			const chordList = chords.trim().split(/\s+/);
			results = await generateProgression(chordList, instrument, {
				limit,
				maxFretDistance: maxDistance,
				generatorOptions: {
					capo,
					playingContext: context,
				},
			});
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			loading = false;
		}
	}

	// Actions - update URL which triggers the effect
	function handleGenerate() {
		// Sync local input to URL state
		if (progressionInput !== urlState.chords) {
			updateUrl(buildProgressionParams({ ...urlState, chords: progressionInput }));
		} else if (progressionInput.trim()) {
			// Force re-generate if input hasn't changed
			doGenerate();
		}
	}

	function handleClear() {
		progressionInput = '';
		results = [];
		error = '';
		lastSearchKey = '';
		updateUrl(buildProgressionParams({ ...urlState, chords: '' }));
	}

	function handleExample(chords: string) {
		progressionInput = chords;
		updateUrl(buildProgressionParams({ ...urlState, chords }));
	}

	function handleOptionsChange(
		opts: Partial<{
			limit: number;
			maxDistance: number;
			capo: number;
			context: 'solo' | 'band';
		}>
	) {
		updateUrl(buildProgressionParams({ ...urlState, ...opts }));
	}

	function handleReset() {
		updateUrl(
			buildProgressionParams({
				...PROGRESSION_DEFAULTS,
				chords: urlState.chords,
				instrument: urlState.instrument,
			})
		);
	}
</script>

<div class="animate-fade-in rounded-xl border border-border bg-card p-6 shadow-warm sm:p-8">
	<!-- Header -->
	<div class="mb-6">
		<h2 class="font-display text-2xl font-bold tracking-tight text-foreground">
			Chord Progression
		</h2>
		<p class="mt-1 text-muted-foreground">
			Enter a sequence of chords to find optimal fingering transitions.
		</p>
	</div>

	<!-- Input -->
	<div class="space-y-4">
		<!-- Common Progressions -->
		<div>
			<p class="mb-2 text-sm font-medium text-foreground">Common Progressions:</p>
			<div class="flex flex-wrap gap-2">
				{#each commonProgressions as progression (progression.name)}
					<Button
						onclick={() => handleExample(progression.chords)}
						variant="secondary"
						size="sm"
					>
						{progression.name}
					</Button>
				{/each}
			</div>
		</div>
		<Input
			bind:value={progressionInput}
			onGenerate={handleGenerate}
			onClear={handleClear}
			disabled={false}
		/>
	</div>

	<!-- Share Button -->
	{#if progressionInput}
		<div class="mt-3 flex justify-end">
			<ShareButton url={page.url.href} title="Share Url" />
		</div>
	{/if}

	<!-- Advanced Options -->
	<AdvancedOptionsWrapper activeFiltersCount={activeFilters} onReset={handleReset}>
		{#snippet content()}
			<AdvancedOptions
				limit={urlState.limit}
				maxDistance={urlState.maxDistance}
				capo={urlState.capo}
				context={urlState.context}
				onChange={handleOptionsChange}
			/>
		{/snippet}
	</AdvancedOptionsWrapper>

	<!-- Error -->
	{#if error}
		<ErrorAlert message={error} />
	{/if}

	<!-- Results -->
	<Results sequences={results} {stringCount} />
</div>
