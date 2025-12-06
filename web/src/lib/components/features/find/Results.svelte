<script lang="ts">
	import FingeringCard from '$lib/components/shared/FingeringCard.svelte';
	import type { ScoredFingering } from '$lib/wasm';

	let { fingerings }: { fingerings: ScoredFingering[] } = $props();
</script>

{#if fingerings.length > 0}
	<div class="mt-8 space-y-6">
		<h3 class="font-display text-lg font-semibold text-foreground">
			Found {fingerings.length} fingering{fingerings.length === 1 ? '' : 's'}
		</h3>
		<div class="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3">
			{#each fingerings as fingering, index (fingering.tab)}
				<FingeringCard {fingering} {index} size="medium" />
			{/each}
		</div>
	</div>
{/if}
