<script lang="ts">
	import * as Collapsible from '$lib/components/ui/collapsible';
	import { Button } from '$lib/components/ui/button';
	import type { Snippet } from 'svelte';

	let {
		activeFiltersCount = 0,
		onReset,
		content,
	}: {
		activeFiltersCount: number;
		onReset: () => void;
		content: Snippet;
	} = $props();

	let open = $state(false);
</script>

<div class="flex gap-3">
	<Collapsible.Root bind:open>
		<Collapsible.Trigger
			class="inline-flex items-center justify-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium whitespace-nowrap ring-offset-background transition-colors hover:bg-accent hover:text-accent-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50"
		>
			<span>Advanced</span>
			{#if activeFiltersCount > 0}
				<span class="rounded-full bg-blue-600 px-2 py-0.5 text-xs font-semibold text-white">
					{activeFiltersCount}
				</span>
			{/if}
			<svg
				class="h-4 w-4 transition-transform {open ? 'rotate-180' : ''}"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M19 9l-7 7-7-7"
				/>
			</svg>
		</Collapsible.Trigger>

		<Collapsible.Content class="mt-6">
			<div class="space-y-6 rounded-lg border border-gray-200 bg-gray-50 p-6">
				<div class="mb-4 flex items-center justify-between">
					<h3 class="text-lg font-semibold text-gray-900">Advanced Options</h3>
					<Button variant="ghost" size="sm" onclick={onReset} class="text-sm underline">
						Reset to defaults
					</Button>
				</div>

				{@render content()}
			</div>
		</Collapsible.Content>
	</Collapsible.Root>
</div>
