<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import CheckIcon from '@lucide/svelte/icons/check';
	import Copy from '@lucide/svelte/icons/copy';
	import { onDestroy } from 'svelte';

	let {
		url,
		title = url.split('/').at(-1),
	}: {
		/** URL to share. */
		url: string;

		/** Share message title. */
		title?: string;
	} = $props();

	let complete = $state(false);

	let timeout: number | NodeJS.Timeout;

	async function share() {
		const shareData: ShareData = { url, title };

		if (navigator.canShare && navigator.canShare(shareData)) {
			await navigator.share(shareData);
		} else {
			await navigator.clipboard.writeText(url);
			complete = true;
			timeout = setTimeout(() => {
				complete = false;
			}, 1500);
		}
	}
	onDestroy(() => {
		clearTimeout(timeout);
	});
</script>

<Button onclick={share} variant="outline" size="sm" title="Share link">
	{#if complete}
		<CheckIcon class="h-5 w-5 text-green-500" />
		Copied!
	{:else}
		<Copy class="h-5 w-5" />
		Share url
	{/if}
</Button>
