<script lang="ts">
	import * as Alert from '$lib/components/ui/alert';

	let { message }: { message: string } = $props();
</script>

<Alert.Root variant="destructive" class="mt-6">
	<Alert.Description>{message}</Alert.Description>
</Alert.Root>
