<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { progressionStore, activeProgressionFilters } from '$lib/stores/progression';
	import Input from '$lib/components/features/progression/Input.svelte';
	import AdvancedOptions from '$lib/components/features/progression/AdvancedOptions.svelte';
	import Results from '$lib/components/features/progression/Results.svelte';
	import AdvancedOptionsWrapper from '$lib/components/shared/AdvancedOptionsWrapper.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';
	import ShareButton from '$lib/components/shared/ShareButton.svelte';

	// Subscribe to store
	let storeState = $derived($progressionStore);
	let activeFilters = $derived($activeProgressionFilters);

	// Local input value (controlled component pattern)
	let progressionInput: string = $state('');

	// Track previous URL to detect changes
	let previousUrl = '';

	// Initialize from URL on mount
	onMount(() => {
		progressionStore.initFromUrl(page.url.searchParams);
		progressionInput = storeState.progressionInput;

		// If there's a progression in the URL, generate immediately
		if (storeState.progressionInput) {
			progressionStore.generate();
		}
	});

	// React to URL changes (browser navigation, manual edits)
	$effect(() => {
		const currentUrl = page.url.href;

		// Only sync if URL actually changed (prevents state → URL → state loop)
		if (currentUrl !== previousUrl) {
			previousUrl = currentUrl;
			progressionStore.initFromUrl(page.url.searchParams);
			progressionInput = storeState.progressionInput; // Sync local state
		}
	});

	// Watch for local input changes and update store + URL
	$effect(() => {
		if (progressionInput !== storeState.progressionInput) {
			progressionStore.setProgressionInput(progressionInput);
		}
	});
</script>

<div class="rounded-lg border border-border bg-card p-6 shadow-sm">
	<h2 class="mb-4 text-xl font-semibold text-foreground">Chord Progression</h2>
	<p class="mb-4 text-muted-foreground">
		Enter a sequence of chords to find optimal fingering transitions.
	</p>

	<!-- Input -->
	<Input
		bind:value={progressionInput}
		onGenerate={() => progressionStore.generate()}
		onClear={() => {
			progressionInput = '';
			progressionStore.clear();
		}}
		disabled={false}
		loading={storeState.loading}
	/>

	{#if progressionInput}
		<div class="flex justify-end">
			<ShareButton url={page.url.href} title="Share Url" />
		</div>
	{/if}

	<!-- Advanced Options -->
	<AdvancedOptionsWrapper
		activeFiltersCount={activeFilters}
		onReset={() => progressionStore.resetOptions()}
	>
		{#snippet content()}
			<AdvancedOptions
				limit={storeState.limit}
				maxDistance={storeState.maxDistance}
				capo={storeState.capo}
				context={storeState.context}
				onChange={(opts) => progressionStore.setOptions(opts)}
			/>
		{/snippet}
	</AdvancedOptionsWrapper>

	<!-- Error -->
	{#if storeState.error}
		<ErrorAlert message={storeState.error} />
	{/if}

	<!-- Results -->
	<Results sequences={storeState.results} />
</div>
