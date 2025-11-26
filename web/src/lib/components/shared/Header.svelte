<script lang="ts">
	let { wasmReady = $bindable(false) }: { wasmReady?: boolean } = $props();
</script>

<header class="border-b bg-white shadow-sm">
	<div class="mx-auto max-w-7xl px-4 py-4 sm:px-6 lg:px-8">
		<h1 class="text-3xl font-bold text-gray-900">ChordCraft</h1>
		<p class="mt-1 text-sm text-gray-500">
			Chord-Fingering Conversion Tool
			{#if wasmReady}
				<span class="text-green-600">● WASM Ready</span>
			{:else}
				<span class="text-yellow-600">● Loading...</span>
			{/if}
		</p>
	</div>
</header>
