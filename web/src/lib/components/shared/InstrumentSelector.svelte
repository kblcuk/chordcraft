<script lang="ts">
	import { page } from '$app/state';
	import { updateUrl } from '$lib/utils/url-state';
	import { INSTRUMENT_METADATA, INSTRUMENT_CATEGORIES, type Instrument } from '$lib/wasm';
	import {
		Select,
		SelectContent,
		SelectGroup,
		SelectItem,
		SelectLabel,
		SelectTrigger,
	} from '$lib/components/ui/select';
	import Guitar from '@lucide/svelte/icons/guitar';
	import Flask from '@lucide/svelte/icons/flask-round';

	// Derive current instrument from URL
	const currentInstrument = $derived<Instrument>(
		(page.url.searchParams.get('instrument') as Instrument) || 'guitar'
	);

	const isStandard = $derived(
		INSTRUMENT_CATEGORIES.standard.includes(
			currentInstrument as (typeof INSTRUMENT_CATEGORIES.standard)[number]
		)
	);

	// Get display value for current instrument
	const currentLabel = $derived(
		INSTRUMENT_METADATA[currentInstrument]?.label || 'Guitar (Standard)'
	);

	function handleChange(newValue: string | undefined) {
		if (!newValue) return;

		// Update URL with new instrument
		const currentParams = Object.fromEntries(page.url.searchParams.entries());
		updateUrl({
			...currentParams,
			instrument: newValue as Instrument,
		});
	}
</script>

<Select value={currentInstrument} onValueChange={handleChange} type="single">
	<SelectTrigger>
		<div class="flex items-center gap-2">
			{#if isStandard}
				<Guitar class="h-4 w-4" />
			{:else}
				<Flask class="h-4 w-4" />
			{/if}
			<span class="hidden sm:inline">{currentLabel}</span>
			<span class="inline sm:hidden">
				{currentLabel.split(' ')[0]}
			</span>
		</div>
	</SelectTrigger>

	<SelectContent>
		<!-- Standard Instruments Group -->
		<SelectGroup>
			<SelectLabel class="text-xs font-semibold">Standard Instruments</SelectLabel>
			{#each INSTRUMENT_CATEGORIES.standard as inst (inst)}
				{@const meta = INSTRUMENT_METADATA[inst]}
				<SelectItem value={inst} class="data-highlighted:bg-primary/10">
					<div class="flex flex-col">
						<span class="text-foreground">{meta.label}</span>
						<span
							class="text-xs text-muted-foreground data-highlighted:text-foreground/70"
						>
							{meta.tuning}
						</span>
					</div>
				</SelectItem>
			{/each}
		</SelectGroup>

		<!-- Alternate Tunings Group -->
		<SelectGroup>
			<SelectLabel class="text-xs font-semibold">Alternate Guitar Tunings</SelectLabel>
			{#each INSTRUMENT_CATEGORIES['alternate-tuning'] as inst (inst)}
				{@const meta = INSTRUMENT_METADATA[inst]}
				<SelectItem value={inst} class="data-highlighted:bg-primary/10">
					<div class="flex flex-col">
						<span>{meta.label}</span>
						<span
							class="text-xs text-muted-foreground data-highlighted:text-foreground/70"
						>
							{meta.tuning}
						</span>
					</div>
				</SelectItem>
			{/each}
		</SelectGroup>
	</SelectContent>
</Select>
