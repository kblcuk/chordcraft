<script lang="ts">
	import type { ChordMatch } from '$lib/wasm';

	let { matches }: { matches: ChordMatch[] } = $props();
</script>

{#if matches.length > 0}
	<div class="mt-6 space-y-3">
		<h3 class="text-lg font-medium text-gray-900">Possible matches:</h3>
		{#each matches as match, i}
			<div class="rounded-md border border-gray-200 bg-gray-50 p-4">
				<div class="flex items-center justify-between">
					<div>
						<span class="text-xl font-bold text-gray-900">{match.name}</span>
						<span class="ml-3 text-sm text-gray-600"
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
				<p class="mt-2 text-sm text-gray-600">{match.explanation}</p>
			</div>
		{/each}
	</div>
{/if}
