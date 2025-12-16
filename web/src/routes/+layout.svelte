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
	// Register service worker for PWA (production only)

	onMount(async () => {
		const shouldRegisterServiceWorker = import.meta.env.PROD && 'serviceWorker' in navigator;
		if (!shouldRegisterServiceWorker) {
			return;
		}

		try {
			const { registerSW } = await import('virtual:pwa-register');
			registerSW({
				immediate: true,
				onNeedRefresh() {
					console.log('New version available! Please reload.');
				},
				onOfflineReady() {
					console.debug('App is ready to work offline!');
				},
			});
		} catch (error) {
			console.error('Failed to register service worker:', error);
		}
	});

	// Initialize fonts and WASM on mount
	onMount(async () => {
		// Initialize WASM in parallel
		try {
			await initializeWasm();
			wasmReady = true;
		} catch (error) {
			console.error('Failed to initialize WASM:', error);
		}
	});
</script>

<!-- Structured Data for SEO -->
<StructuredData type="WebApplication" />

<!-- Main app content -->
<div class="bg-textured flex min-h-screen flex-col bg-background">
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
