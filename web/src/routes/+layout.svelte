<script lang="ts">
	import { onMount } from 'svelte';
	import { ModeWatcher } from 'mode-watcher';
	import { initializeWasm } from '$lib/wasm';
	import Header from '$lib/components/shared/Header.svelte';
	import Navigation from '$lib/components/shared/Navigation.svelte';
	import Footer from '$lib/components/shared/Footer.svelte';
	import '../app.css';

	let { children } = $props();

	let wasmReady = $state(false);

	// Initialize WASM once at layout level
	onMount(async () => {
		try {
			await initializeWasm();
			wasmReady = true;
		} catch (error) {
			console.error('Failed to initialize WASM:', error);
		}
	});
</script>

<div class="min-h-screen bg-background">
	<Header bind:wasmReady />
	<ModeWatcher />

	<!-- Navigation -->
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<Navigation />
	</div>

	<!-- Page Content -->
	<main class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		{@render children()}
	</main>

	<Footer />
</div>
