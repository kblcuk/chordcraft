<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { nameStore } from '$lib/stores/name';
	import Input from '$lib/components/features/name/Input.svelte';
	import Results from '$lib/components/features/name/Results.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';

	// Subscribe to store
	let state = $derived($nameStore);

	// Initialize from URL on mount
	onMount(() => {
		nameStore.initFromUrl($page.url.searchParams);

		// If there's a tab in the URL, analyze immediately
		if (state.tabInput) {
			nameStore.analyze();
		}
	});
</script>

<div class="rounded-lg border border-border bg-card p-6 shadow-sm">
	<h2 class="mb-4 text-xl font-semibold text-foreground">Name Chord</h2>
	<p class="mb-4 text-muted-foreground">Enter tab notation to identify a chord.</p>

	<!-- Input -->
	<Input
		bind:value={state.tabInput}
		onAnalyze={() => nameStore.analyze()}
		onClear={() => nameStore.clear()}
		disabled={false}
		loading={state.loading}
	/>

	<!-- Error -->
	{#if state.error}
		<ErrorAlert message={state.error} />
	{/if}

	<!-- Results -->
	<Results matches={state.results} />
</div>
