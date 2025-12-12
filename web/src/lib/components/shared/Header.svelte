<script lang="ts">
	import { page } from '$app/state';
	import DarkModeToggle from '$lib/DarkModeToggle.svelte';
	import InstallButton from '$lib/components/shared/InstallButton.svelte';
	import Guitar from '@lucide/svelte/icons/guitar';
	import Ukulele from '@lucide/svelte/icons/tree-palm';
	import { updateUrl } from '$lib/utils/url-state';
	import type { Instrument } from '$lib/wasm';
	import { Button } from '$lib/components/ui/button';

	let { wasmReady = $bindable(false) }: { wasmReady?: boolean } = $props();

	// Derive instrument directly from URL
	const instrument = $derived<Instrument>(
		page.url.searchParams.get('instrument') === 'ukulele' ? 'ukulele' : 'guitar'
	);

	function toggleInstrument() {
		const newInstrument: Instrument = instrument === 'guitar' ? 'ukulele' : 'guitar';
		// Just update URL - routes will react automatically
		const currentParams = Object.fromEntries(page.url.searchParams.entries());
		updateUrl({
			...currentParams,
			instrument: newInstrument,
		});
	}
</script>

<header class="relative border-b border-border/50 bg-card shadow-warm">
	<!-- Subtle texture overlay -->
	<div class="bg-textured pointer-events-none absolute inset-0 opacity-50"></div>

	<div class="relative mx-auto max-w-7xl px-4 py-5 sm:px-6 lg:px-8">
		<div class="flex items-center justify-between">
			<!-- Logo & Title -->
			<div class="flex items-center gap-3">
				<!-- Guitar Icon with amber accent -->
				<div
					class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10 text-primary"
				>
					<Guitar class="h-5 w-5" />
				</div>

				<div>
					<h1
						class="font-display text-2xl font-bold tracking-tight text-foreground sm:text-3xl"
					>
						ChordCraft
					</h1>
					<p class="text-xs text-muted-foreground sm:text-sm">
						Chord-Fingering Conversion Tool
					</p>
				</div>
			</div>

			<!-- Right side: Instrument toggle + Status + Dark mode -->
			<div class="flex items-center gap-3 sm:gap-4">
				<!-- Instrument Toggle -->
				<Button
					onclick={toggleInstrument}
					variant="outline"
					title={`Switch to ${instrument === 'guitar' ? 'ukulele' : 'guitar'}`}
				>
					{#if instrument === 'guitar'}
						<Guitar class="h-4 w-4" />
						<span class="hidden sm:inline">Guitar</span>
					{:else}
						<Ukulele class="h-4 w-4" />
						<span class="hidden sm:inline">Ukulele</span>
					{/if}
				</Button>

				<!-- WASM Status -->
				<div class="hidden items-center gap-2 sm:flex">
					{#if wasmReady}
						<span class="flex items-center gap-1.5 text-xs text-success">
							<span class="h-1.5 w-1.5 animate-pulse rounded-full bg-success"></span>
							Ready
						</span>
					{:else}
						<span class="flex items-center gap-1.5 text-xs text-muted-foreground">
							<span
								class="animate-pulse-subtle h-1.5 w-1.5 rounded-full bg-muted-foreground"
							></span>
							Loading...
						</span>
					{/if}
				</div>

				<InstallButton />
				<DarkModeToggle />
			</div>
		</div>
	</div>
</header>
