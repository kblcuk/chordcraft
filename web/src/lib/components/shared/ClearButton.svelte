<script lang="ts">
	import { Button } from '$lib/components/ui/button';

	let { onclick }: { onclick: () => void } = $props();
</script>

<Button {onclick} variant="outline" size="icon" title="Clear input">
	<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
		<path
			stroke-linecap="round"
			stroke-linejoin="round"
			stroke-width="2"
			d="M6 18L18 6M6 6l12 12"
		/>
	</svg>
</Button>
