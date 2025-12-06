<script lang="ts">
	import Form from '$lib/components/features/name/Form.svelte';

	let {
		initialValue = $bindable('000000'),
	}: {
		initialValue?: string;
	} = $props();

	// Reactive state for testing
	let value = $state(initialValue);

	// Expose value for testing (via data attribute)
	$effect(() => {
		// Update DOM attribute so tests can read it
		if (typeof document !== 'undefined') {
			const wrapper = document.querySelector('[data-test-wrapper]');
			if (wrapper) {
				wrapper.setAttribute('data-bound-value', value);
			}
		}
	});
</script>

<div data-test-wrapper data-bound-value={value}>
	<Form bind:value />
</div>
