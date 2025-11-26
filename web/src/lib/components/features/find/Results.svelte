<script lang="ts">
	import FingeringCard from '$lib/components/shared/FingeringCard.svelte';
	import type { ScoredFingering } from '$lib/wasm';

	let { fingerings }: { fingerings: ScoredFingering[] } = $props();
</script>

{#if fingerings.length > 0}
	<div class="mt-6 space-y-6">
		<h3 class="text-lg font-medium text-foreground">
			Found {fingerings.length} fingerings:
		</h3>
		<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
			{#each fingerings as fingering (fingering.tab)}
				<FingeringCard {fingering} size="medium" />
			{/each}
		</div>
	</div>
{/if}
