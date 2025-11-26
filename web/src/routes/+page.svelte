<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	// Redirect to /find on mount
	onMount(() => {
		goto('/find', { replaceState: true });
	});
</script>

<div class="flex min-h-screen items-center justify-center">
	<p class="text-gray-500">Redirecting...</p>
</div>
