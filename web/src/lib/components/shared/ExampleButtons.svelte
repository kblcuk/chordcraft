<script lang="ts">
	import { Button } from '$lib/components/ui/button';

	type Example = string | { tab: string; label: string };

	let {
		examples,
		onSelect,
		disabled = false,
	}: {
		examples: Example[];
		onSelect: (value: string) => void;
		disabled?: boolean;
	} = $props();

	function getValue(example: Example): string {
		return typeof example === 'string' ? example : example.tab;
	}
</script>

<div>
	<p class="mb-2 text-sm font-medium text-gray-700">Quick Examples:</p>
	<div class="flex flex-wrap gap-2">
		{#each examples as example}
			<Button
				onclick={() => onSelect(getValue(example))}
				{disabled}
				variant="secondary"
				size="sm"
			>
				{#if typeof example === 'string'}
					{example}
				{:else}
					<span class="font-mono">{example.tab}</span>
					<span class="ml-1 text-gray-500">({example.label})</span>
				{/if}
			</Button>
		{/each}
	</div>
</div>
