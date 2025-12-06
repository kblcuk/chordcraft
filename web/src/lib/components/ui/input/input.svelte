<script lang="ts">
	import type { HTMLInputAttributes, HTMLInputTypeAttribute } from 'svelte/elements';
	import { cn, type WithElementRef } from '$lib/utils.js';

	type InputType = Exclude<HTMLInputTypeAttribute, 'file'>;

	type Props = WithElementRef<
		Omit<HTMLInputAttributes, 'type'> &
			({ type: 'file'; files?: FileList } | { type?: InputType; files?: undefined })
	>;

	let {
		ref = $bindable(null),
		value = $bindable(),
		type,
		files = $bindable(),
		class: className,
		'data-slot': dataSlot = 'input',
		...restProps
	}: Props = $props();
</script>

{#if type === 'file'}
	<input
		bind:this={ref}
		data-slot={dataSlot}
		class={cn(
			'flex h-10 w-full min-w-0 rounded-lg border border-border bg-card px-3 pt-1.5 text-sm font-medium shadow-warm-sm transition-all duration-200 outline-none selection:bg-primary/20 selection:text-foreground placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50',
			'focus-visible:border-primary focus-visible:shadow-warm focus-visible:ring-[3px] focus-visible:ring-ring/30',
			'aria-invalid:border-destructive aria-invalid:ring-destructive/20',
			className
		)}
		type="file"
		bind:files
		bind:value
		{...restProps}
	/>
{:else}
	<input
		bind:this={ref}
		data-slot={dataSlot}
		class={cn(
			'flex h-10 w-full min-w-0 rounded-lg border border-border bg-card px-3 py-2 text-base shadow-warm-sm transition-all duration-200 outline-none selection:bg-primary/20 selection:text-foreground placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
			'focus-visible:border-primary focus-visible:shadow-warm focus-visible:ring-[3px] focus-visible:ring-ring/30',
			'aria-invalid:border-destructive aria-invalid:ring-destructive/20',
			className
		)}
		{type}
		bind:value
		{...restProps}
	/>
{/if}
