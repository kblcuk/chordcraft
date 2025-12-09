<script lang="ts">
	import ChordDiagram from '$lib/ChordDiagram.svelte';
	import Check from '@lucide/svelte/icons/check';
	import X from '@lucide/svelte/icons/x';
	import type { ScoredFingering } from '$lib/wasm';

	let {
		fingering,
		size = 'medium',
		index = 0,
		stringCount = 6,
	}: {
		fingering: ScoredFingering;
		size?: 'small' | 'medium' | 'large';
		index?: number;
		stringCount?: number;
	} = $props();

	// Stagger animation class
	let staggerClass = $derived(`stagger-${Math.min(index + 1, 6)}`);
</script>

<div
	class="card-lift animate-fade-in-up rounded-xl border border-border bg-card p-4 opacity-0 shadow-warm transition-all duration-200 hover:border-primary/30 {staggerClass}"
>
	<!-- Chord Diagram -->
	<div class="mb-4 flex justify-center">
		<ChordDiagram
			tab={fingering.tab}
			notes={fingering.notes}
			rootNote={fingering.notes[0] || ''}
			{size}
			{stringCount}
		/>
	</div>

	<!-- Tab Notation -->
	<div class="mb-3 text-center">
		<code
			class="inline-block rounded-lg bg-secondary px-4 py-1.5 text-base font-semibold tracking-wide text-foreground"
		>
			{fingering.tab}
		</code>
	</div>

	<!-- Metadata Badges -->
	<div class="mb-3 flex flex-wrap justify-center gap-2">
		<!-- Voicing Type -->
		<span
			class="rounded-full bg-primary/10 px-2.5 py-0.5 text-xs font-medium text-primary capitalize"
		>
			{fingering.voicingType}
		</span>

		<!-- Score -->
		<span class="rounded-full bg-secondary px-2.5 py-0.5 text-xs font-medium text-foreground">
			Score: {fingering.score}
		</span>

		<!-- Position -->
		<span class="rounded-full bg-secondary px-2.5 py-0.5 text-xs font-medium text-foreground">
			Fret {fingering.position}
		</span>
	</div>

	<!-- Notes & Root in Bass -->
	<div class="space-y-1.5 text-center text-xs">
		<!-- Notes list -->
		<div class="text-muted-foreground">
			Notes: <span class="font-medium text-foreground">{fingering.notes.join(', ')}</span>
		</div>

		<!-- Root in bass indicator -->
		<div>
			{#if fingering.hasRootInBass}
				<span class="inline-flex items-center gap-1 text-success">
					<Check class="h-3 w-3" />
					Root in bass
				</span>
			{:else}
				<span class="inline-flex items-center gap-1 text-muted-foreground/60">
					<X class="h-3 w-3" />
					No root in bass
				</span>
			{/if}
		</div>
	</div>
</div>
