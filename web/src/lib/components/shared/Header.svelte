<script lang="ts">
	import DarkModeToggle from '$lib/DarkModeToggle.svelte';
	let { wasmReady = $bindable(false) }: { wasmReady?: boolean } = $props();
</script>

<header class="border-b bg-card shadow-sm">
	<div class="mx-auto max-w-7xl px-4 py-4 sm:px-6 lg:px-8">
		<h1 class="text-3xl font-bold text-foreground">ChordCraft</h1>
		<p class="mt-1 text-sm text-muted-foreground">
			Chord-Fingering Conversion Tool
			{#if wasmReady}
				<span class="text-green-600 dark:text-green-500">● WASM Ready</span>
			{:else}
				<span class="text-yellow-600 dark:text-yellow-500">● Loading...</span>
			{/if}
		</p>
		<div class="absolute top-4 right-4">
			<DarkModeToggle />
		</div>
	</div>
</header>
