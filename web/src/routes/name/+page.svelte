<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { nameStore } from '$lib/stores/name';
	import Form from '$lib/components/features/name/Form.svelte';
	import Results from '$lib/components/features/name/Results.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';
	import InteractiveChordDiagram from '$lib/components/features/name/InteractiveChordDiagram.svelte';
	import FingerCountBadge from '$lib/components/features/name/FingerCountBadge.svelte';
	import { Label } from '$lib/components/ui/label';
	import ShareButton from '$lib/components/shared/ShareButton.svelte';

	// Subscribe to store
	let storeState = $derived($nameStore);

	// Local input values (controlled component pattern)
	let tabInput: string = $state('000000');
	let startFret: number = $state(0);

	// Track previous URL to detect changes
	let previousUrl = '';

	// Initialize from URL on mount
	onMount(() => {
		nameStore.initFromUrl(page.url.searchParams);
		tabInput = storeState.tabInput;
		startFret = storeState.startFret;

		// If there's a tab in the URL, analyze immediately
		if (storeState.tabInput) {
			nameStore.analyze();
		}
	});

	// React to URL changes (browser navigation, manual edits)
	$effect(() => {
		const currentUrl = page.url.href;
		// Only sync if URL actually changed (prevents state → URL → state loop)
		if (currentUrl === previousUrl) return;

		previousUrl = currentUrl;
		nameStore.initFromUrl(page.url.searchParams);
		tabInput = storeState.tabInput; // Sync local state
		startFret = storeState.startFret;
	});

	// Watch for startFret changes and update store + URL
	$effect(() => {
		if (startFret === storeState.startFret) return;
		nameStore.setStartFret(startFret);
	});

	// Watch for tab changes and auto-analyze
	$effect(() => {
		if (tabInput === storeState.tabInput) return;

		nameStore.setTabInput(tabInput);
		nameStore.analyze();
	});
</script>

<div class="rounded-lg border border-border bg-card p-6 shadow-sm">
	<h2 class="mb-4 text-xl font-semibold text-foreground">Name Chord</h2>
	<p class="mb-4 text-muted-foreground">
		Identify a chord by entering tab notation or building it visually on the fretboard.
	</p>

	<div class="space-y-6">
		<!-- Visual Input Section -->
		<div class="space-y-4">
			<div class="flex items-center justify-between">
				<h3 class="text-sm font-medium text-foreground">Visual Input</h3>

				<!-- Capo Selector -->
				<div class="flex items-center gap-2">
					<Label for="capo-select" class="text-sm font-medium">Capo:</Label>
					<select
						id="capo-select"
						value={storeState.capo}
						onchange={(e) => nameStore.setCapo(Number(e.currentTarget.value))}
						class="rounded-md border border-border bg-background px-3 py-1.5 text-sm"
					>
						<option value="0">None</option>
						{#each Array.from({ length: 12 }, (v, i) => i + 1) as fretNum (fretNum)}
							<option value={fretNum}>Fret {fretNum}</option>
						{/each}
					</select>
					{#if storeState.capo > 0}
						<span class="text-xs text-muted-foreground">
							(+{storeState.capo} semitones)
						</span>
					{/if}
				</div>
			</div>

			<!-- Interactive Fretboard -->
			<div class="flex justify-center">
				<InteractiveChordDiagram
					bind:value={tabInput}
					bind:startFret
					capo={storeState.capo}
					size="large"
				/>
			</div>
		</div>

		<!-- Divider -->
		<div class="relative">
			<div class="absolute inset-0 flex items-center">
				<div class="w-full border-t border-border"></div>
			</div>
			<div class="relative flex justify-center text-xs uppercase">
				<span class="bg-card px-2 text-muted-foreground">Or</span>
			</div>
		</div>

		<!-- Text Input Section -->
		<div class="space-y-2">
			<h3 class="text-sm font-medium text-foreground">Text Input</h3>
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
	{#if storeState.error}
		<div class="mt-4">
			<ErrorAlert message={storeState.error} />
		</div>
	{/if}

	<!-- Results -->
	{#if storeState.results.length > 0}
		<div class="mt-6 space-y-4">
			<!-- Finger Count Badge -->
			<div class="flex justify-center">
				<FingerCountBadge tab={storeState.tabInput} />
			</div>

			<Results matches={storeState.results} />
		</div>
	{/if}
</div>
