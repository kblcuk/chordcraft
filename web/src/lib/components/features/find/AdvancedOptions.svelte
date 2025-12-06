<script lang="ts">
	import { Slider } from '$lib/components/ui/slider';
	import { Label } from '$lib/components/ui/label';

	type VoicingType = 'all' | 'core' | 'full' | 'jazzy';
	type PlayingContext = 'solo' | 'band';

	let {
		limit,
		capo,
		voicing,
		position,
		context,
		onChange,
	}: {
		limit: number;
		capo: number;
		voicing: VoicingType;
		position: number | null;
		context: PlayingContext;
		onChange: (
			options: Partial<{
				limit: number;
				capo: number;
				voicing: VoicingType;
				position: number | null;
				context: PlayingContext;
			}>
		) => void;
	} = $props();
</script>

<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
	<!-- Limit Slider -->
	<div>
		<Label for="find-limit" class="mb-2">
			Number of Fingerings: {limit}
		</Label>
		<Slider
			id="find-limit"
			type="single"
			value={limit}
			min={5}
			max={50}
			step={5}
			onValueChange={(v: number) => onChange({ limit: v })}
		/>
		<div class="mt-1 flex justify-between text-xs text-muted-foreground">
			<span>5</span>
			<span>50</span>
		</div>
	</div>

	<!-- Capo Selector -->
	<div>
		<Label for="find-capo" class="mb-2">Capo Position</Label>
		<select
			id="find-capo"
			value={capo}
			onchange={(e) => onChange({ capo: Number(e.currentTarget.value) })}
			class="w-full rounded-md border border-border px-4 py-2 focus:ring-2"
		>
			<option value={0}>No capo</option>
			{#each [...Array(12).keys()] as i (i)}
				<option value={i + 1}>Fret {i + 1}</option>
			{/each}
		</select>
	</div>

	<!-- Position Preference -->
	<div>
		<Label for="find-position" class="mb-2">Preferred Position</Label>
		<select
			id="find-position"
			value={position ?? ''}
			onchange={(e) => {
				const val = e.currentTarget.value;
				onChange({ position: val === '' ? null : Number(val) });
			}}
			class="w-full rounded-md border border-border px-4 py-2 focus:border-blue-500 focus:ring-2 focus:ring-blue-500"
		>
			<option value={null}>Any position</option>
			<option value={0}>Open position (0-5)</option>
			{#each [...Array(12).keys()] as i (i)}
				<option value={i + 1}>Around fret {i + 1}</option>
			{/each}
		</select>
	</div>

	<!-- Playing Context -->
	<div>
		<Label class="mb-2">Playing Context</Label>
		<div class="flex gap-4">
			<label class="flex cursor-pointer items-center">
				<input
					type="radio"
					checked={context === 'solo'}
					value="solo"
					onchange={() => onChange({ context: 'solo' })}
					class="h-4 w-4 text-blue-600 focus:ring-blue-500"
				/>
				<span class="ml-2 text-sm text-foreground">
					Solo
					<span class="text-muted-foreground">(full bass)</span>
				</span>
			</label>
			<label class="flex cursor-pointer items-center">
				<input
					type="radio"
					checked={context === 'band'}
					value="band"
					onchange={() => onChange({ context: 'band' })}
					class="h-4 w-4 text-blue-600 focus:ring-blue-500"
				/>
				<span class="ml-2 text-sm text-foreground">
					Band
					<span class="text-muted-foreground">(lighter)</span>
				</span>
			</label>
		</div>
	</div>
</div>

<!-- Voicing Filter -->
<div>
	<Label class="mb-3">Voicing Type</Label>
	<div class="grid grid-cols-2 gap-2">
		<label class="flex cursor-pointer items-center">
			<input
				type="radio"
				checked={voicing === 'all'}
				value="all"
				onchange={() => onChange({ voicing: 'all' })}
				class="h-4 w-4 text-blue-600 focus:ring-blue-500"
			/>
			<span class="ml-2 text-sm text-foreground">
				All
				<span class="text-muted-foreground">(show everything)</span>
			</span>
		</label>
		<label class="flex cursor-pointer items-center">
			<input
				type="radio"
				checked={voicing === 'core'}
				value="core"
				onchange={() => onChange({ voicing: 'core' })}
				class="h-4 w-4 text-blue-600 focus:ring-blue-500"
			/>
			<span class="ml-2 text-sm text-foreground">
				Core
				<span class="text-muted-foreground">(essential)</span>
			</span>
		</label>
		<label class="flex cursor-pointer items-center">
			<input
				type="radio"
				checked={voicing === 'full'}
				value="full"
				onchange={() => onChange({ voicing: 'full' })}
				class="h-4 w-4 text-blue-600 focus:ring-blue-500"
			/>
			<span class="ml-2 text-sm text-foreground">
				Full
				<span class="text-muted-foreground">(complete)</span>
			</span>
		</label>
		<label class="flex cursor-pointer items-center">
			<input
				type="radio"
				checked={voicing === 'jazzy'}
				value="jazzy"
				onchange={() => onChange({ voicing: 'jazzy' })}
				class="h-4 w-4 text-blue-600 focus:ring-blue-500"
			/>
			<span class="ml-2 text-sm text-foreground">
				Jazzy
				<span class="text-muted-foreground">(extended)</span>
			</span>
		</label>
	</div>
</div>
