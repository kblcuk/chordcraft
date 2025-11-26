<script lang="ts">
	import FingeringCard from '$lib/components/shared/FingeringCard.svelte';
	import ChordDiagram from '$lib/ChordDiagram.svelte';
	import type { ProgressionSequence } from '$lib/wasm';

	let { sequences }: { sequences: ProgressionSequence[] } = $props();
</script>

{#if sequences.length > 0}
	<div class="mt-6 space-y-8">
		{#each sequences as sequence, i}
			<div class="rounded-lg border-2 border-gray-300 bg-white p-6">
				<div class="mb-6 flex items-center justify-between">
					<h3 class="text-xl font-bold text-gray-900">
						Alternative #{i + 1}
					</h3>
					<div class="text-sm">
						<span class="text-gray-600">Avg Transition:</span>
						<span class="ml-1 font-semibold text-blue-600">
							{sequence.avgTransitionScore.toFixed(1)}
						</span>
					</div>
				</div>

				<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
					{#each sequence.fingerings as fingering, j}
						<div class="relative">
							<div class="rounded-lg border-2 border-gray-200 bg-gray-50 p-4">
								<!-- Chord Name -->
								<div class="mb-3 text-center">
									<h4 class="text-lg font-bold text-gray-900">
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
										class="rounded border bg-white px-2 py-1 font-mono text-sm"
									>
										{fingering.tab}
									</code>
								</div>

								<!-- Metadata -->
								<div class="flex flex-wrap justify-center gap-1 text-xs">
									<span class="rounded bg-blue-100 px-2 py-0.5 text-blue-800">
										{fingering.voicingType}
									</span>
									<span class="rounded bg-gray-200 px-2 py-0.5 text-gray-700">
										Fret {fingering.position}
									</span>
								</div>
							</div>

							<!-- Transition Arrow (Desktop) -->
							{#if j < sequence.transitions.length}
								<div
									class="absolute top-1/2 -right-3 z-10 hidden -translate-y-1/2 lg:block"
								>
									<div
										class="rounded-full border-2 border-green-500 bg-white p-2 shadow-md"
									>
										<svg
											class="h-5 w-5 text-green-600"
											fill="none"
											stroke="currentColor"
											viewBox="0 0 24 24"
										>
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M13 7l5 5m0 0l-5 5m5-5H6"
											/>
										</svg>
									</div>
									<div class="mt-1 text-center">
										<div class="text-xs font-semibold text-green-600">
											Score: {sequence.transitions[j].score}
										</div>
										<div class="text-xs text-gray-500">
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
