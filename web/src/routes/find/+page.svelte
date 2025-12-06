<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { findStore, activeFindFilters } from '$lib/stores/find';
	import Input from '$lib/components/features/find/Input.svelte';
	import AdvancedOptions from '$lib/components/features/find/AdvancedOptions.svelte';
	import Results from '$lib/components/features/find/Results.svelte';
	import AdvancedOptionsWrapper from '$lib/components/shared/AdvancedOptionsWrapper.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';
	import ShareButton from '$lib/components/shared/ShareButton.svelte';

	// Subscribe to store
	let storeState = $derived($findStore);
	let activeFilters = $derived($activeFindFilters);

	// Local input value (controlled component pattern)
	let chordInput: string = $state('');

	// Track previous URL to detect changes
	let previousUrl = '';

	// Initialize from URL on mount
	onMount(() => {
		findStore.initFromUrl(page.url.searchParams);
		chordInput = storeState.chordInput;

		// If there's a chord in the URL, search immediately
		if (storeState.chordInput) {
			findStore.search();
		}
	});

	// React to URL changes (browser navigation, manual edits)
	$effect(() => {
		const currentUrl = page.url.href;

		// Only sync if URL actually changed (prevents state → URL → state loop)
		if (currentUrl !== previousUrl) {
			previousUrl = currentUrl;
			findStore.initFromUrl(page.url.searchParams);
			chordInput = storeState.chordInput; // Sync local state
		}
	});
</script>

<div class="animate-fade-in rounded-xl border border-border bg-card p-6 shadow-warm sm:p-8">
	<!-- Header -->
	<div class="mb-6">
		<h2
			class="text-2xl font-bold tracking-tight text-foreground"
			style="font-family: var(--font-display);"
		>
			Find Fingerings
		</h2>
		<p class="mt-1 text-muted-foreground">
			Enter a chord name to discover all possible fingerings.
		</p>
	</div>

	<!-- Input -->
	<Input
		bind:value={chordInput}
		onSearch={() => {
			if (chordInput !== storeState.chordInput) {
				findStore.setChordInput(chordInput);
			}
			findStore.search();
		}}
		onClear={() => {
			chordInput = '';
			findStore.clear();
		}}
		disabled={false}
		loading={storeState.loading}
	/>

	<!-- Share Button -->
	{#if chordInput}
		<div class="mt-3 flex justify-end">
			<ShareButton url={page.url.href} title="Share Url" />
		</div>
	{/if}

	<!-- Advanced Options -->
	<AdvancedOptionsWrapper
		activeFiltersCount={activeFilters}
		onReset={() => findStore.resetOptions()}
	>
		{#snippet content()}
			<AdvancedOptions
				limit={storeState.limit}
				capo={storeState.capo}
				voicing={storeState.voicing}
				position={storeState.position}
				context={storeState.context}
				onChange={(opts) => findStore.setOptions(opts)}
			/>
		{/snippet}
	</AdvancedOptionsWrapper>

	<!-- Error -->
	{#if storeState.error}
		<ErrorAlert message={storeState.error} />
	{/if}

	<!-- Results -->
	<Results fingerings={storeState.results} />
</div>
