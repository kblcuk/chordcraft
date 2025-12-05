<script lang="ts">
	import ChordDiagram from '$lib/ChordDiagram.svelte';
	import type { ProgressionSequence } from '$lib/wasm';
	import ArrowRightIcon from '@lucide/svelte/icons/arrow-right';

	let { sequences }: { sequences: ProgressionSequence[] } = $props();
</script>

{#if sequences.length > 0}
	<div class="mt-6 space-y-8">
		{#each sequences as sequence, i (sequence.fingerings.map((f) => f.tab).join('-'))}
			<div class="rounded-lg border-2 border-border bg-card p-6">
				<div class="mb-6 flex items-center justify-between">
					<h3 class="text-xl font-bold text-foreground">
						Alternative #{i + 1}
					</h3>
					<div class="text-sm">
						<span class="text-muted-foreground">Avg Transition:</span>
						<span class="ml-1 font-semibold text-blue-600">
							{sequence.avgTransitionScore.toFixed(1)}
						</span>
					</div>
				</div>

				<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
					{#each sequence.fingerings as fingering, j (fingering.tab + j)}
						<div class="relative">
							<div class="rounded-lg border-2 border-border bg-background p-4">
								<!-- Chord Name -->
								<div class="mb-3 text-center">
									<h4 class="text-lg font-bold text-foreground">
										{sequence.chords[j]}
									</h4>
								</div>

								<!-- Chord Diagram -->
								<div class="mb-3 flex justify-center">
									<ChordDiagram
										tab={fingering.tab}
										notes={fingering.notes}
										rootNote={fingering.notes[0] || ''}
										size="small"
									/>
								</div>

								<!-- Tab Notation -->
								<div class="mb-2 text-center">
									<code
										class="rounded border border-border bg-card px-2 py-1 font-mono text-sm"
									>
										{fingering.tab}
									</code>
								</div>

								<!-- Metadata -->
								<div class="flex flex-wrap justify-center gap-1 text-xs">
									<span class="rounded bg-blue-100 px-2 py-0.5 text-blue-800">
										{fingering.voicingType}
									</span>
									<span class="rounded bg-muted px-2 py-0.5 text-foreground">
										Fret {fingering.position}
									</span>
								</div>
							</div>

							<!-- Transition Arrow (Desktop) -->
							{#if j < sequence.transitions.length}
								<div
									class="absolute top-1/2 -right-11 z-10 hidden -translate-y-1/2 lg:block"
								>
									<div
										class="flex justify-center rounded-full border-2 border-green-500 bg-card p-2 shadow-md dark:border-green-600"
									>
										<ArrowRightIcon
											class="h-6 w-6 text-green-500 dark:text-green-600"
										/>
									</div>
									<div class="mt-1 text-center">
										<div
											class="text-xs font-semibold text-green-600 dark:text-green-500"
										>
											Score: {sequence.transitions[j].score}
										</div>
										<div class="text-xs text-muted-foreground">
											{sequence.transitions[j].fingerMovements} move{sequence
												.transitions[j].fingerMovements !== 1
												? 's'
												: ''}
										</div>
									</div>
								</div>

								<!-- Mobile Transition Info -->
								<div
									class="mt-3 rounded border border-green-200 bg-green-50 p-2 text-center lg:hidden"
								>
									<div class="text-sm text-green-700">
										→ Next: {sequence.transitions[j].fingerMovements} finger{sequence
											.transitions[j].fingerMovements !== 1
											? 's'
											: ''} move, {sequence.transitions[j].commonAnchors} anchor{sequence
											.transitions[j].commonAnchors !== 1
											? 's'
											: ''}
										<span class="font-semibold"
											>(score: {sequence.transitions[j].score})</span
										>
									</div>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
{/if}
