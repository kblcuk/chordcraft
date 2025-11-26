<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { findStore, activeFindFilters } from '$lib/stores/find';
	import Input from '$lib/components/features/find/Input.svelte';
	import AdvancedOptions from '$lib/components/features/find/AdvancedOptions.svelte';
	import Results from '$lib/components/features/find/Results.svelte';
	import AdvancedOptionsWrapper from '$lib/components/shared/AdvancedOptionsWrapper.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';

	// Subscribe to store
	let state = $derived($findStore);
	let activeFilters = $derived($activeFindFilters);

	// Initialize from URL on mount
	onMount(() => {
		findStore.initFromUrl($page.url.searchParams);

		// If there's a chord in the URL, search immediately
		if (state.chordInput) {
			findStore.search();
		}
	});
</script>

<div class="rounded-lg border border-border bg-card p-6 shadow-sm">
	<h2 class="mb-4 text-xl font-semibold text-foreground">Find Fingerings</h2>
	<p class="mb-4 text-muted-foreground">Enter a chord name to see all possible fingerings.</p>

	<!-- Input -->
	<Input
		bind:value={state.chordInput}
		onSearch={() => findStore.search()}
		onClear={() => findStore.clear()}
		disabled={false}
		loading={state.loading}
	/>

	<!-- Advanced Options -->
	<AdvancedOptionsWrapper
		activeFiltersCount={activeFilters}
		onReset={() => findStore.resetOptions()}
	>
		{#snippet content()}
			<AdvancedOptions
				limit={state.limit}
				capo={state.capo}
				voicing={state.voicing}
				position={state.position}
				context={state.context}
				onChange={(opts) => findStore.setOptions(opts)}
			/>
		{/snippet}
	</AdvancedOptionsWrapper>

	<!-- Error -->
	{#if state.error}
		<ErrorAlert message={state.error} />
	{/if}

	<!-- Results -->
	<Results fingerings={state.results} />
</div>
