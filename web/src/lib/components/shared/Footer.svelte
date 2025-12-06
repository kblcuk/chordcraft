<script lang="ts">
	import Github from '@lucide/svelte/icons/github';
	import Heart from '@lucide/svelte/icons/heart';
</script>

<footer class="mt-auto border-t border-border/50 bg-card/50">
	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<div class="flex flex-col items-center gap-4 text-center">
			<!-- Tech stack -->
			<p class="flex items-center gap-2 text-sm text-muted-foreground">
				<span>Built with</span>
				<span class="inline-flex items-center gap-1">
					<Heart class="h-3 w-3 text-destructive" />
				</span>
				<span>using Rust + WASM + Svelte</span>
			</p>

			<!-- Links -->
			<div class="flex items-center gap-6">
				<a
					href="https://github.com/kblcuk/chordcraft"
					class="group flex items-center gap-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground"
				>
					<Github class="h-4 w-4 transition-transform group-hover:scale-110" />
					<span>View on GitHub</span>
				</a>
			</div>

			<!-- Copyright -->
			<p class="text-xs text-muted-foreground/70">
				{new Date().getFullYear()} ChordCraft. Open source project.
			</p>
		</div>
	</div>
</footer>
