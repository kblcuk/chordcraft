<script lang="ts">
	import ExampleButtons from '$lib/components/shared/ExampleButtons.svelte';
	import * as InputGroup from '$lib/components/ui/input-group';
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
			<InputGroup.Root>
				<InputGroup.Input
					id="chord-input"
					type="text"
					bind:value
					onkeydown={handleKeydown}
					onblur={onSearch}
					disabled={loading || disabled}
					placeholder="e.g., Cmaj7, Abm7, G7"
					class="flex-1"
				/>
				{#if value}
					<InputGroup.Addon align="inline-end">
						<InputGroup.Button onclick={onClear} variant="secondary"
							>X</InputGroup.Button
						>
					</InputGroup.Addon>
				{/if}
			</InputGroup.Root>
		</div>
		<p class="mt-1 text-xs text-muted-foreground">Press Enter or click away to search</p>
	</div>
</div>
