<script lang="ts">
	import ExampleButtons from '$lib/components/shared/ExampleButtons.svelte';
	import * as InputGroup from '$lib/components/ui/input-group';
	import { Label } from '$lib/components/ui/label';
	import { exampleTabs } from '$lib/utils/examples';

	// Validation pattern and error message
	const PATTERN = /^[0-9x()]{0,24}$/;
	const ERROR_MESSAGE =
		'Valid tab notation can contain x (for muted strings), or fret numbers. Group double-digit frets with brackets, like (11).';

	// Check if tab notation is complete and valid
	function isValidTabNotation(str: string): boolean {
		// First check basic pattern
		if (!PATTERN.test(str)) return false;

		// Empty string is valid
		if (str === '') return true;

		// Check for balanced brackets
		let openCount = 0;
		for (let char of str) {
			if (char === '(') openCount++;
			if (char === ')') {
				openCount--;
				if (openCount < 0) return false; // Closing before opening
			}
		}

		// All brackets must be closed
		return openCount === 0;
	}

	let {
		value = $bindable('000000'),
		disabled = false,
	}: {
		value: string;
		disabled?: boolean;
	} = $props();

	// Local state - what user actually types (can be invalid during typing)
	let localInput = $state(value);

	// Validation state - only show errors after blur
	let showError = $state(false);
	let isValid = $state(true);

	// Track previous value to detect external changes
	let previousValue = $state(value);

	// Sync DOWN: when parent changes value (e.g., diagram click), update local input
	$effect(() => {
		// Only sync if value changed externally (not from our own update)
		if (value !== previousValue) {
			localInput = value;
			previousValue = value;
			showError = false;
		}
	});

	// Sync UP: when local input is valid AND complete, propagate to parent
	$effect(() => {
		isValid = isValidTabNotation(localInput);
		if (isValid && localInput !== value) {
			value = localInput;
			previousValue = value; // Update tracker so DOWN effect doesn't trigger
		}
	});

	function handleBlur() {
		// Show validation errors only after user finishes typing
		showError = true;
	}

	function clear() {
		localInput = '000000';
		value = '000000';
		previousValue = '000000';
		showError = false;
	}

	function handleExample(tab: string) {
		localInput = tab;
		value = tab;
		previousValue = tab;
		showError = false;
	}
</script>

<div class="space-y-4">
	<ExampleButtons examples={exampleTabs} onSelect={handleExample} {disabled} />

	<div class="space-y-2">
		<Label for="tab-input">Tab Notation</Label>
		<InputGroup.Root>
			<InputGroup.Input
				id="tab-input"
				data-testid="tab-input"
				type="text"
				bind:value={localInput}
				onblur={handleBlur}
				placeholder="e.g., x32010"
				{disabled}
				class="flex-1 font-mono"
				aria-invalid={showError && !isValid}
			/>
			{#if localInput}
				<InputGroup.Addon align="inline-end">
					<InputGroup.Button onclick={clear} variant="secondary">X</InputGroup.Button>
				</InputGroup.Addon>
			{/if}
		</InputGroup.Root>

		<!-- Error message - only show after blur -->
		{#if showError && !isValid}
			<p class="text-sm text-destructive">{ERROR_MESSAGE}</p>
		{/if}

		<!-- Helper text -->
		<p class="text-xs text-muted-foreground">Type to identify the chord automatically</p>
	</div>
</div>
