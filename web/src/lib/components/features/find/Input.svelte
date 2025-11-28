<script lang="ts">
	import ExampleButtons from '$lib/components/shared/ExampleButtons.svelte';
	import ClearButton from '$lib/components/shared/ClearButton.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { exampleChords } from '$lib/utils/examples';

	let {
		value = $bindable(''),
		onSearch,
		onClear,
		disabled = false,
		loading = false,
	}: {
		value: string;
		onSearch: () => void;
		onClear: () => void;
		disabled?: boolean;
		loading?: boolean;
	} = $props();

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			onSearch();
		}
	}

	function handleExample(chord: string) {
		value = chord;
		onSearch();
	}
</script>

<div class="space-y-4">
	<ExampleButtons examples={exampleChords} onSelect={handleExample} {disabled} />

	<div>
		<Label for="chord-input" class="mb-2">Chord Name</Label>
		<div class="flex gap-2">
			<Input
				id="chord-input"
				type="text"
				bind:value
				onkeydown={handleKeydown}
				onblur={onSearch}
				{loading}
				placeholder="e.g., Cmaj7, Abm7, G7"
				{disabled}
				class="flex-1"
			/>
			{#if value}
				<ClearButton onclick={onClear} />
			{/if}
		</div>
		<p class="mt-1 text-xs text-muted-foreground">Press Enter or click away to search</p>
	</div>
</div>
