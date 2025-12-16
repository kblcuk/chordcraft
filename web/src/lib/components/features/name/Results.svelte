<script lang="ts">
	import type { ChordMatch } from '$lib/wasm';

	let {
		matches,
		loading = false,
	}: {
		matches: ChordMatch[];
		loading?: boolean;
	} = $props();
</script>

{#if loading}
	<!-- Skeleton loader to prevent CLS -->
	<div class="mt-6 space-y-3">
		<div class="h-6 w-40 animate-pulse rounded bg-muted"></div>
		{#each Array.from(Array(3)).map((v, i) => i) as i (i)}
			<div class="animate-pulse rounded-md border border-border bg-background p-4">
				<div class="flex items-center justify-between">
					<div class="flex-1">
						<div class="h-7 w-32 rounded bg-muted"></div>
						<div class="mt-1 h-4 w-40 rounded bg-muted"></div>
					</div>
					{#if i === 0}
						<div class="h-6 w-24 rounded bg-muted"></div>
					{/if}
				</div>
				<div class="mt-2 space-y-1">
					<div class="h-4 w-full rounded bg-muted"></div>
					<div class="h-4 w-3/4 rounded bg-muted"></div>
				</div>
			</div>
		{/each}
	</div>
{:else if matches.length > 0}
	<div class="mt-6 space-y-3">
		<h3 class="text-lg font-medium text-foreground">Possible matches:</h3>
		{#each matches as match, i (match.name)}
			<div class="rounded-md border border-border bg-background p-4">
				<div class="flex items-center justify-between">
					<div>
						<span class="text-xl font-bold text-foreground">{match.name}</span>
						<span class="ml-3 text-sm text-muted-foreground"
							>{match.confidence}% confidence</span
						>
					</div>
					{#if i === 0}
						<span
							class="rounded bg-green-100 px-2 py-1 text-xs font-medium text-green-800"
						>
							Best Match
						</span>
					{/if}
				</div>
				<p class="mt-2 text-sm text-muted-foreground">{match.explanation}</p>
			</div>
		{/each}
	</div>
{/if}
