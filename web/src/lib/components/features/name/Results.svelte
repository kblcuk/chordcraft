<script lang="ts">
	import type { ChordMatch } from '$lib/wasm';

	let { matches }: { matches: ChordMatch[] } = $props();
</script>

{#if matches.length > 0}
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
