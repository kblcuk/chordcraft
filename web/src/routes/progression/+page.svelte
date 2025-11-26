<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { progressionStore, activeProgressionFilters } from '$lib/stores/progression';
	import Input from '$lib/components/features/progression/Input.svelte';
	import AdvancedOptions from '$lib/components/features/progression/AdvancedOptions.svelte';
	import Results from '$lib/components/features/progression/Results.svelte';
	import AdvancedOptionsWrapper from '$lib/components/shared/AdvancedOptionsWrapper.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';

	// Subscribe to store
	let state = $derived($progressionStore);
	let activeFilters = $derived($activeProgressionFilters);

	// Initialize from URL on mount
	onMount(() => {
		progressionStore.initFromUrl($page.url.searchParams);

		// If there's a progression in the URL, generate immediately
		if (state.progressionInput) {
			progressionStore.generate();
		}
	});
</script>

<div class="rounded-lg border bg-white p-6 shadow-sm">
	<h2 class="mb-4 text-xl font-semibold text-gray-900">Chord Progression</h2>
	<p class="mb-4 text-gray-600">
		Enter a sequence of chords to find optimal fingering transitions.
	</p>

	<!-- Input -->
	<Input
		bind:value={state.progressionInput}
		onGenerate={() => progressionStore.generate()}
		onClear={() => progressionStore.clear()}
		disabled={false}
		loading={state.loading}
	/>

	<!-- Advanced Options -->
	<AdvancedOptionsWrapper
		activeFiltersCount={activeFilters}
		onReset={() => progressionStore.resetOptions()}
	>
		{#snippet content()}
			<AdvancedOptions
				limit={state.limit}
				maxDistance={state.maxDistance}
				capo={state.capo}
				context={state.context}
				onChange={(opts) => progressionStore.setOptions(opts)}
			/>
		{/snippet}
	</AdvancedOptionsWrapper>

	<!-- Error -->
	{#if state.error}
		<ErrorAlert message={state.error} />
	{/if}

	<!-- Results -->
	<Results sequences={state.results} />
</div>
