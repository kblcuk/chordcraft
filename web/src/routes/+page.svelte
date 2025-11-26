<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	// Redirect to /find on mount
	onMount(() => {
		goto(resolve('/find'), { replaceState: true });
	});
</script>

<div class="flex min-h-screen items-center justify-center">
	<p class="text-muted-foreground">Redirecting...</p>
</div>
