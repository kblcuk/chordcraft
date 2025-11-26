<script lang="ts">
	import ClearButton from '$lib/components/shared/ClearButton.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import { commonProgressions } from '$lib/utils/examples';

	let {
		value = $bindable(''),
		onGenerate,
		onClear,
		disabled = false,
		loading = false,
	}: {
		value: string;
		onGenerate: () => void;
		onClear: () => void;
		disabled?: boolean;
		loading?: boolean;
	} = $props();

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			onGenerate();
		}
	}

	function handleExample(chords: string) {
		value = chords;
		onGenerate();
	}
</script>

<div class="space-y-4">
	<!-- Common Progressions -->
	<div>
		<p class="mb-2 text-sm font-medium text-gray-700">Common Progressions:</p>
		<div class="flex flex-wrap gap-2">
			{#each commonProgressions as progression}
				<Button
					onclick={() => handleExample(progression.chords)}
					{disabled}
					variant="secondary"
					size="sm"
				>
					{progression.name}
				</Button>
			{/each}
		</div>
	</div>

	<div>
		<Label for="progression-input" class="mb-2">Chord Progression (space-separated)</Label>
		<div class="flex gap-2">
			<Input
				id="progression-input"
				type="text"
				bind:value
				onkeydown={handleKeydown}
				onblur={onGenerate}
				placeholder="e.g., Cmaj7 Am7 Dm7 G7"
				{disabled}
				class="flex-1"
			/>
			{#if value}
				<ClearButton onclick={onClear} />
			{/if}
		</div>
		<p class="mt-1 text-xs text-gray-500">Press Enter or click away to generate</p>
	</div>

	<Button
		onclick={onGenerate}
		disabled={disabled || loading || !value.trim()}
		variant="outline"
		size="sm"
	>
		{loading ? 'Generating...' : 'Generate'}
	</Button>
</div>
