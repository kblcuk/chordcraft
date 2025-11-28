<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { nameStore } from '$lib/stores/name';
	import Input from '$lib/components/features/name/Input.svelte';
	import Results from '$lib/components/features/name/Results.svelte';
	import ErrorAlert from '$lib/components/shared/ErrorAlert.svelte';
	import InteractiveChordDiagram from '$lib/components/features/name/InteractiveChordDiagram.svelte';
	import FingerCountBadge from '$lib/components/features/name/FingerCountBadge.svelte';
	import { Label } from '$lib/components/ui/label';
	import { debounce } from '$lib/utils/url-state';

	// Subscribe to store
	let state = $derived($nameStore);

	// Initialize from URL on mount
	onMount(() => {
		nameStore.initFromUrl(page.url.searchParams);

		// If there's a tab in the URL, analyze immediately
		if (state.tabInput) {
			nameStore.analyze();
		}
	});

	// Debounced auto-analyze (200ms for snappy feel)
	const debouncedAnalyze = debounce(() => {
		if (state.tabInput.trim()) {
			nameStore.analyze();
		}
	}, 200);

	// Watch for tab changes and auto-analyze
	$effect(() => {
		if (state.tabInput) {
			debouncedAnalyze();
		}
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
						bind:value={state.capo}
						onchange={(e) => nameStore.setCapo(Number(e.currentTarget.value))}
						class="rounded-md border border-border bg-background px-3 py-1.5 text-sm"
					>
						<option value="0">None</option>
						{#each Array(12)
							.fill(0)
							.map((v, i) => i + 1) as fretNum (fretNum)}
							<option value={fretNum}>Fret {fretNum}</option>
						{/each}
					</select>
					{#if state.capo > 0}
						<span class="text-xs text-muted-foreground">
							(+{state.capo} semitones)
						</span>
					{/if}
				</div>
			</div>

			<!-- Interactive Fretboard -->
			<div class="flex justify-center">
				<InteractiveChordDiagram
					bind:value={state.tabInput}
					bind:startFret={state.startFret}
					capo={state.capo}
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
			<Input
				bind:value={state.tabInput}
				onAnalyze={() => nameStore.analyze()}
				onClear={() => nameStore.clear()}
				disabled={false}
				loading={state.loading}
			/>
		</div>
	</div>

	<!-- Error -->
	{#if state.error}
		<div class="mt-4">
			<ErrorAlert message={state.error} />
		</div>
	{/if}

	<!-- Results -->
	{#if state.results.length > 0}
		<div class="mt-6 space-y-4">
			<!-- Finger Count Badge -->
			<div class="flex justify-center">
				<FingerCountBadge tab={state.tabInput} />
			</div>

			<Results matches={state.results} />
		</div>
	{/if}
</div>
