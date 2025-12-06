<script lang="ts">
	import * as InputGroup from '$lib/components/ui/input-group';
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
</script>

<div>
	<Label for="progression-input" class="mb-2">Chord Progression (space-separated)</Label>
	<div class="flex gap-2">
		<InputGroup.Root>
			<InputGroup.Input
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
				<InputGroup.Addon align="inline-end">
					<InputGroup.Button onclick={onClear} variant="secondary">X</InputGroup.Button>
				</InputGroup.Addon>
			{/if}
		</InputGroup.Root>
	</div>
	<p class="mt-1 text-xs text-muted-foreground">Press Enter or click away to generate</p>
</div>

<Button
	onclick={onGenerate}
	disabled={disabled || loading || !value.trim()}
	variant="outline"
	size="sm"
>
	{loading ? 'Generating...' : 'Generate'}
</Button>
