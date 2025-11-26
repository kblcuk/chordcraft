<script lang="ts">
	import ChordDiagram from '$lib/ChordDiagram.svelte';
	import type { ScoredFingering } from '$lib/wasm';

	let {
		fingering,
		size = 'medium',
	}: {
		fingering: ScoredFingering;
		size?: 'small' | 'medium' | 'large';
	} = $props();
</script>

<div
	class="rounded-lg border-2 border-border bg-card p-4 transition-colors hover:border-blue-400 dark:hover:border-blue-600"
>
	<!-- Chord Diagram -->
	<div class="mb-3 flex justify-center">
		<ChordDiagram
			tab={fingering.tab}
			notes={fingering.notes}
			rootNote={fingering.notes[0] || ''}
			{size}
		/>
	</div>

	<!-- Tab Notation -->
	<div class="mb-2 text-center">
		<code class="rounded bg-muted px-3 py-1 font-mono text-lg font-bold">
			{fingering.tab}
		</code>
	</div>

	<!-- Metadata -->
	<div class="mb-2 flex flex-wrap justify-center gap-2">
		<span class="rounded bg-blue-100 px-2 py-1 text-xs font-medium text-blue-800">
			{fingering.voicingType}
		</span>
		<span class="rounded bg-muted px-2 py-1 text-xs font-medium text-foreground">
			Score: {fingering.score}
		</span>
		<span class="rounded bg-muted px-2 py-1 text-xs font-medium text-foreground">
			Fret {fingering.position}
		</span>
	</div>

	<!-- Notes and Root in Bass -->
	<div class="space-y-1 text-center text-xs text-muted-foreground">
		<div>Notes: {fingering.notes.join(', ')}</div>
		<div>
			{#if fingering.hasRootInBass}
				<span class="text-green-600 dark:text-green-500">✓ Root in bass</span>
			{:else}
				<span class="text-muted-foreground/70">No root in bass</span>
			{/if}
		</div>
	</div>
</div>
