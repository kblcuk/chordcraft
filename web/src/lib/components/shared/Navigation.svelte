<script lang="ts">
	import { page } from '$app/stores';

	const routes = [
		{ path: '/find', label: 'Find Fingerings' },
		{ path: '/name', label: 'Name Chord' },
		{ path: '/progression', label: 'Progression' },
	];

	function isActive(path: string): boolean {
		return $page.url.pathname === path;
	}
</script>

<nav class="flex space-x-2 rounded-lg border bg-white p-1 shadow-sm">
	{#each routes as route}
		<a
			href={route.path}
			class="flex-1 rounded-md px-4 py-2 text-center text-sm font-medium transition-colors {isActive(
				route.path
			)
				? 'bg-blue-600 text-white'
				: 'text-gray-600 hover:bg-gray-100'}"
		>
			{route.label}
		</a>
	{/each}
</nav>
