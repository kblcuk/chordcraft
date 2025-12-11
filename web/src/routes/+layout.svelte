<script lang="ts">
	import { onMount } from 'svelte';
	import { ModeWatcher } from 'mode-watcher';
	import { initializeWasm } from '$lib/wasm';
	import Header from '$lib/components/shared/Header.svelte';
	import Navigation from '$lib/components/shared/Navigation.svelte';
	import Footer from '$lib/components/shared/Footer.svelte';
	import StructuredData from '$lib/components/seo/StructuredData.svelte';
	import '../app.css';

	let { children } = $props();

	let wasmReady = $state(false);
	let fontsReady = $state(false);
	let appReady = $derived(fontsReady);

	// Initialize fonts and WASM on mount
	onMount(async () => {
		// Wait for fonts to load (with timeout fallback)
		const fontLoadPromise = (async () => {
			await document.fonts.ready;
			fontsReady = true;
		})();

		// Fallback: show content after 1.5s even if fonts aren't loaded
		const timeoutPromise = new Promise<void>((resolve) => {
			setTimeout(() => {
				fontsReady = true;
				resolve();
			}, 1500);
		});

		// Initialize WASM in parallel
		const wasmLoadPromise = (async () => {
			try {
				await initializeWasm();
				wasmReady = true;
			} catch (error) {
				console.error('Failed to initialize WASM:', error);
			}
		})();

		// Wait for fonts (or timeout) - WASM can continue loading in background
		await Promise.all([wasmLoadPromise, Promise.race([fontLoadPromise, timeoutPromise])]);
	});
</script>

<!-- Structured Data for SEO -->
<StructuredData type="WebApplication" />

<!-- Loading screen shown until fonts are ready -->
{#if !appReady}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-background"
		aria-label="Loading"
	>
		<div class="flex flex-col items-center gap-4">
			<!-- Simple loading spinner -->
			<div
				class="h-8 w-8 animate-spin rounded-full border-4 border-primary/20 border-t-primary"
			></div>
			<p class="text-sm text-muted-foreground">Loading ChordCraft...</p>
		</div>
	</div>
{/if}

<!-- Main app content -->
<div
	class="bg-textured flex min-h-screen flex-col bg-background transition-opacity duration-300"
	class:opacity-0={!appReady}
	class:opacity-100={appReady}
>
	<Header bind:wasmReady />
	<ModeWatcher />

	<!-- Navigation -->
	<div class="mx-auto w-full max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<Navigation />
	</div>

	<!-- Page Content -->
	<main class="mx-auto w-full max-w-7xl flex-1 px-4 pb-8 sm:px-6 lg:px-8">
		{@render children()}
	</main>

	<Footer />
</div>
