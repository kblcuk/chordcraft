<script lang="ts">
	import { page } from '$app/state';
	import {
		parseFindParams,
		buildFindParams,
		updateUrl,
		countFindFilters,
		FIND_DEFAULTS,
	} from '$lib/utils/url-state';
	import {
		findFingerings,
		getInstrumentInfo,
		type ScoredFingering,
		type InstrumentInfo,
	} from '$lib/wasm';
	import Input from '$lib/components/features/find/Input.svelte';
	import AdvancedOptions from '$lib/components/features/find/AdvancedOptions.svelte';
	import Results from '$lib/components/features/find/Results.svelte';
	import AdvancedOptionsWrapper from '$lib/components/shared/AdvancedOptionsWrapper.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';
	import ShareButton from '$lib/components/shared/ShareButton.svelte';

	// Derive all input state from URL (single source of truth)
	const urlState = $derived(parseFindParams(page.url.searchParams));
	const activeFilters = $derived(countFindFilters(urlState));

	// Local state for results (not in URL)
	let results = $state<ScoredFingering[]>([]);
	let loading = $state(false);
	let error = $state('');

	// Instrument info for string count (cached)
	let instrumentInfo = $state<InstrumentInfo | null>(null);
	const stringCount = $derived(instrumentInfo?.stringCount ?? 6);

	// Local input value for controlled component
	// Sync local input with URL state (for browser back/forward)
	let chordInput = $derived(urlState.chord);

	// Track last search params to detect meaningful changes
	let lastSearchKey = '';

	// Load instrument info when instrument changes
	$effect(() => {
		const instrument = urlState.instrument;
		getInstrumentInfo(instrument).then((info) => {
			instrumentInfo = info;
		});
	});

	// React to URL changes - trigger search when we have input
	$effect(() => {
		const { chord, instrument, limit, capo, voicing, position, context } = urlState;

		// Create a key representing all search-relevant params
		const searchKey = JSON.stringify({
			chord,
			instrument,
			limit,
			capo,
			voicing,
			position,
			context,
		});

		// Only search if params changed and we have input
		if (searchKey !== lastSearchKey && chord.trim()) {
			lastSearchKey = searchKey;
			doSearch();
		}
	});

	async function doSearch() {
		const { chord, instrument, limit, capo, voicing, position, context } = urlState;

		if (!chord.trim() || loading) return;

		loading = true;
		error = '';

		try {
			const voicingType = voicing === 'all' ? undefined : voicing;
			results = await findFingerings(chord.trim(), instrument, {
				limit,
				capo,
				voicingType,
				preferredPosition: position ?? undefined,
				playingContext: context,
			});
		} catch (e) {
			error = e instanceof Error ? e.message : 'Unknown error';
		} finally {
			loading = false;
		}
	}

	// Actions - update URL which triggers the effect
	function handleSearch() {
		// Sync local input to URL state
		if (chordInput !== urlState.chord) {
			updateUrl(buildFindParams({ ...urlState, chord: chordInput }));
		} else if (chordInput.trim()) {
			// Force re-search if input hasn't changed
			doSearch();
		}
	}

	function handleClear() {
		chordInput = '';
		results = [];
		error = '';
		lastSearchKey = '';
		updateUrl(buildFindParams({ ...urlState, chord: '' }));
	}

	function handleOptionsChange(
		opts: Partial<{
			limit: number;
			capo: number;
			voicing: 'all' | 'core' | 'full' | 'jazzy';
			position: number | null;
			context: 'solo' | 'band';
		}>
	) {
		updateUrl(buildFindParams({ ...urlState, ...opts }));
	}

	function handleReset() {
		updateUrl(
			buildFindParams({
				...FIND_DEFAULTS,
				chord: urlState.chord,
				instrument: urlState.instrument,
			})
		);
	}
</script>

<svelte:head>
	<title>Find Chord Fingerings - ChordCraft Guitar & Ukulele Tool</title>
	<meta
		name="description"
		content="Find multiple fingering options for any chord on guitar or ukulele. Input chord names like Cmaj7, Abm7, F#7b9 to discover playable voicings with capo support and position preferences."
	/>
	<meta
		name="keywords"
		content="guitar chords, ukulele chords, chord fingerings, guitar chord finder, ukulele chord finder, chord voicings, capo chords, jazz chords, chord diagrams"
	/>
	<link rel="canonical" href="https://chordcraft.rocks/find" />

	<!-- Open Graph / Facebook -->
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://chordcraft.rocks/find" />
	<meta property="og:title" content="Find Chord Fingerings - ChordCraft Guitar & Ukulele Tool" />
	<meta
		property="og:description"
		content="Find multiple fingering options for any chord on guitar or ukulele. Discover playable voicings with advanced options like capo support and position preferences."
	/>
	<meta property="og:image" content="https://chordcraft.rocks/og-image.png" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:url" content="https://chordcraft.rocks/find" />
	<meta name="twitter:title" content="Find Chord Fingerings - ChordCraft Guitar & Ukulele Tool" />
	<meta
		name="twitter:description"
		content="Find multiple fingering options for any chord on guitar or ukulele. Discover playable voicings with advanced options."
	/>
	<meta name="twitter:image" content="https://chordcraft.rocks/og-image.png" />
</svelte:head>

<div class="animate-fade-in rounded-xl border border-border bg-card p-6 shadow-warm sm:p-8">
	<!-- Header -->
	<div class="mb-6">
		<h2 class="font-display text-2xl font-bold tracking-tight text-foreground">
			Find Fingerings
		</h2>
		<p class="mt-1 text-muted-foreground">
			Enter a chord name to discover all possible fingerings.
		</p>
	</div>

	<!-- Input -->
	<Input
		bind:value={chordInput}
		onSearch={handleSearch}
		onClear={handleClear}
		disabled={false}
		{loading}
	/>

	<!-- Share Button -->
	{#if chordInput}
		<div class="mt-3 flex justify-end">
			<ShareButton url={page.url.href} title="Share Url" />
		</div>
	{/if}

	<!-- Advanced Options -->
	<AdvancedOptionsWrapper activeFiltersCount={activeFilters} onReset={handleReset}>
		{#snippet content()}
			<AdvancedOptions
				limit={urlState.limit}
				capo={urlState.capo}
				voicing={urlState.voicing}
				position={urlState.position}
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
	<Results fingerings={results} {stringCount} />
</div>
