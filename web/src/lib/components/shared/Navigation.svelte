<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { routes } from '$lib/utils/url-state';

	function isActive(path: string): boolean {
		return page.url.pathname === path;
	}
</script>

<nav class="flex space-x-2 rounded-lg border border-border bg-card p-1 shadow-sm">
	{#each routes as route (route.label)}
		<a
			href={resolve(route.path)}
			class="flex-1 rounded-md px-4 py-2 text-center text-sm font-medium transition-colors {isActive(
				route.path
			)
				? 'bg-blue-600 text-white dark:bg-blue-700'
				: 'text-muted-foreground hover:bg-muted'}"
		>
			{route.label}
		</a>
	{/each}
</nav>
