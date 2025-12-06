<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { routes } from '$lib/utils/url-state';
	import Search from '@lucide/svelte/icons/search';
	import Music from '@lucide/svelte/icons/music';
	import ListMusic from '@lucide/svelte/icons/list-music';

	const icons = {
		'/find': Search,
		'/name': Music,
		'/progression': ListMusic,
	} as const;

	function isActive(path: string): boolean {
		return page.url.pathname === path;
	}
</script>

<nav class="relative">
	<!-- Fret markers (decorative dots at positions 3 and 5) -->
	<div
		class="pointer-events-none absolute -top-2 left-1/4 h-1.5 w-1.5 rounded-full bg-border/50"
	></div>
	<div
		class="pointer-events-none absolute -top-2 right-1/4 h-1.5 w-1.5 rounded-full bg-border/50"
	></div>

	<!-- Navigation container styled like a fretboard section -->
	<div class="relative rounded-xl border border-border bg-card p-1.5 shadow-warm">
		<!-- Horizontal fret lines (subtle) -->
		<div class="pointer-events-none absolute inset-x-2 top-0 h-px bg-border/30"></div>
		<div class="pointer-events-none absolute inset-x-2 bottom-0 h-px bg-border/30"></div>

		<div class="flex gap-1.5">
			{#each routes as route (route.label)}
				{@const Icon = icons[route.path as keyof typeof icons]}
				{@const active = isActive(route.path)}
				<a
					href={resolve(route.path)}
					class="group relative flex flex-1 items-center justify-center gap-2 rounded-lg px-4 py-2.5 text-sm font-medium transition-all duration-200
						{active
						? 'bg-primary text-primary-foreground shadow-warm-sm'
						: 'text-muted-foreground hover:bg-secondary hover:text-foreground'}"
				>
					{#if Icon}
						<Icon
							class="h-4 w-4 transition-transform duration-200 group-hover:scale-110"
						/>
					{/if}
					<span>{route.label}</span>

					<!-- Active indicator dot (like a fret marker) -->
					{#if active}
						<span
							class="absolute -bottom-3 left-1/2 h-1 w-1 -translate-x-1/2 rounded-full bg-primary"
						></span>
					{/if}
				</a>
			{/each}
		</div>
	</div>
</nav>
