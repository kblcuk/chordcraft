<script lang="ts">
	import FingeringCard from '$lib/components/shared/FingeringCard.svelte';
	import type { ScoredFingering } from '$lib/wasm';

	let {
		fingerings,
		stringCount = 6,
		stringNames = ['E', 'A', 'D', 'G', 'B', 'e'],
		loading = false,
	}: {
		fingerings: ScoredFingering[];
		stringCount?: number;
		stringNames?: string[];
		loading?: boolean;
	} = $props();
</script>

{#if loading}
	<!-- Skeleton loader to prevent CLS -->
	<div class="mt-8 space-y-6">
		<div class="h-7 w-48 animate-pulse rounded bg-muted"></div>
		<div class="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3">
			{#each Array.from(Array(6)).map((v, i) => i) as i (i)}
				<div
					class="animate-pulse rounded-xl border border-border bg-card p-4 shadow-warm"
					style="min-height: 380px;"
				>
					<!-- Diagram skeleton -->
					<div class="mb-4 flex h-[180px] items-center justify-center">
						<div class="h-40 w-32 rounded-lg bg-muted"></div>
					</div>
					<!-- Tab notation skeleton -->
					<div class="mb-3 flex justify-center">
						<div class="h-8 w-32 rounded-lg bg-muted"></div>
					</div>
					<!-- Badges skeleton -->
					<div class="mb-3 flex justify-center gap-2">
						<div class="h-5 w-16 rounded-full bg-muted"></div>
						<div class="h-5 w-20 rounded-full bg-muted"></div>
						<div class="h-5 w-16 rounded-full bg-muted"></div>
					</div>
					<!-- Notes skeleton -->
					<div class="space-y-1.5">
						<div class="mx-auto h-4 w-40 rounded bg-muted"></div>
						<div class="mx-auto h-4 w-32 rounded bg-muted"></div>
					</div>
				</div>
			{/each}
		</div>
	</div>
{:else if fingerings.length > 0}
	<div class="mt-8 space-y-6">
		<h3 class="font-display text-lg font-semibold text-foreground">
			Found {fingerings.length} fingering{fingerings.length === 1 ? '' : 's'}
		</h3>
		<div class="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3">
			{#each fingerings as fingering, index (fingering.tab)}
				<FingeringCard {fingering} {index} size="medium" {stringCount} {stringNames} />
			{/each}
		</div>
	</div>
{/if}
