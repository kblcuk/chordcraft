<script lang="ts">
	import ExampleButtons from '$lib/components/shared/ExampleButtons.svelte';
	import ClearButton from '$lib/components/shared/ClearButton.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import { exampleTabs } from '$lib/utils/examples';

	let {
		value = $bindable(''),
		onAnalyze,
		onClear,
		disabled = false,
		loading = false,
	}: {
		value: string;
		onAnalyze: () => void;
		onClear: () => void;
		disabled?: boolean;
		loading?: boolean;
	} = $props();

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			onAnalyze();
		}
	}

	function handleExample(tab: string) {
		value = tab;
		onAnalyze();
	}
</script>

<div class="space-y-4">
	<ExampleButtons examples={exampleTabs} onSelect={handleExample} {disabled} />

	<div>
		<Label for="tab-input" class="mb-2">Tab Notation</Label>
		<div class="flex gap-2">
			<Input
				id="tab-input"
				type="text"
				bind:value
				onkeydown={handleKeydown}
				onblur={onAnalyze}
				placeholder="e.g., x32010"
				{disabled}
				class="flex-1 font-mono"
			/>
			{#if value}
				<ClearButton onclick={onClear} />
			{/if}
		</div>
		<p class="mt-1 text-xs text-muted-foreground">Press Enter or click away to identify</p>
	</div>

	<Button
		onclick={onAnalyze}
		disabled={disabled || loading || !value.trim()}
		variant="outline"
		size="sm"
	>
		{loading ? 'Analyzing...' : 'Identify'}
	</Button>
</div>
