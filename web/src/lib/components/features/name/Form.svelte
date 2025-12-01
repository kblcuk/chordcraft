<script lang="ts">
	// Essentially this is a one-field form, but it
	// makes values validation much easier even with a bit of a
	// manual sync between incoming properties and local input

	import { z } from 'zod';
	import { zod4 } from 'sveltekit-superforms/adapters';
	import { defaults, superForm } from 'sveltekit-superforms';
	import * as Form from '$lib/components/ui/form';
	import ExampleButtons from '$lib/components/shared/ExampleButtons.svelte';
	import * as InputGroup from '$lib/components/ui/input-group';
	import { exampleTabs } from '$lib/utils/examples';

	let schema = z.object({
		value: z
			.string()
			.regex(
				/^[0-9x()]{0,24}$/,
				'Valid tab notaion can contain x (for muted strings), or fret numbers. Group double-digit frets with brackets, like (11).'
			),
	});
	const form = superForm(defaults(zod4(schema)), {
		validators: zod4(schema),
		SPA: true,
		resetForm: false,
		onChange(args) {
			console.info('jojojo', args, args.get(args.paths[0]));
		},
		onUpdate: ({ form }) => {
			console.info('ASDFASDF', form.valid, form.data.value, value);
			if (!form.valid) return;
			value = form.data.value;
		},
	});

	const { form: formData, enhance, constraints } = form;
	console.info('DAFUK', $constraints.value?.pattern);

	let {
		// value = $bindable($formData.value),
		value = $bindable('000000'),
		disabled = false,
		loading = false,
	}: {
		value: string;
		disabled?: boolean;
		loading?: boolean;
	} = $props();

	function clear() {
		// Since we need form to be valid, we set everything to "muted"
		value = '000000';
	}

	// Sync form data with value if it's updated externally
	// $effect(() => {
	// 	if (value === $formData.value) return;

	// 	$formData.value = value;
	// });

	function handleExample(tab: string) {
		value = tab;
		form.submit();
	}
</script>

<div class="space-y-4">
	<ExampleButtons examples={exampleTabs} onSelect={handleExample} {disabled} />

	<p class="mt-1 text-xs text-muted-foreground">Press Enter or click away to identify</p>
	<form method="POST" use:enhance>
		<Form.Field {form} name="value">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						>Tab Notation
						<!-- bind:value={$formData.value} -->
					</Form.Label>
					<InputGroup.Root>
						<InputGroup.Input
							{...props}
							data-testid="tab-input"
							type="text"
							bind:value={$formData.value}
							{...$constraints.value}
							placeholder="e.g., x32010"
							onblur={form.submit}
							{disabled}
							class="flex-1 font-mono"
						/>
						{#if value}
							<InputGroup.Addon align="inline-end">
								<InputGroup.Button onclick={clear} variant="secondary"
									>X</InputGroup.Button
								>
							</InputGroup.Addon>
						{/if}
					</InputGroup.Root>
				{/snippet}
			</Form.Control>
			<Form.Description />
			<Form.FieldErrors />
			<Form.Button
				disabled={disabled || loading || !value.trim()}
				variant="outline"
				size="sm"
			>
				{loading ? 'Analyzing...' : 'Identify'}
			</Form.Button>
		</Form.Field>
	</form>
</div>
