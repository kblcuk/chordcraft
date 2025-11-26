<script lang="ts">
	import { Slider } from '$lib/components/ui/slider';
	import { Label } from '$lib/components/ui/label';

	type PlayingContext = 'solo' | 'band';

	let {
		limit,
		maxDistance,
		capo,
		context,
		onChange,
	}: {
		limit: number;
		maxDistance: number;
		capo: number;
		context: PlayingContext;
		onChange: (
			options: Partial<{
				limit: number;
				maxDistance: number;
				capo: number;
				context: PlayingContext;
			}>
		) => void;
	} = $props();
</script>

<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
	<!-- Number of Alternatives -->
	<div>
		<Label for="prog-limit" class="mb-2">
			Number of Alternatives: {limit}
		</Label>
		<Slider
			id="prog-limit"
			value={[limit]}
			min={1}
			max={10}
			step={1}
			onValueChange={(v) => onChange({ limit: v[0] })}
		/>
		<div class="mt-1 flex justify-between text-xs text-gray-500">
			<span>1</span>
			<span>10</span>
		</div>
	</div>

	<!-- Max Fret Distance -->
	<div>
		<Label for="prog-distance" class="mb-2">
			Max Fret Distance: {maxDistance}
		</Label>
		<Slider
			id="prog-distance"
			value={[maxDistance]}
			min={1}
			max={12}
			step={1}
			onValueChange={(v) => onChange({ maxDistance: v[0] })}
		/>
		<div class="mt-1 flex justify-between text-xs text-gray-500">
			<span>1 fret</span>
			<span>12 frets</span>
		</div>
		<p class="mt-2 text-xs text-gray-500">Maximum fret jump between consecutive fingerings</p>
	</div>

	<!-- Capo Selector -->
	<div>
		<Label for="prog-capo" class="mb-2">Capo Position</Label>
		<select
			id="prog-capo"
			value={capo}
			onchange={(e) => onChange({ capo: Number(e.currentTarget.value) })}
			class="w-full rounded-md border border-gray-300 px-4 py-2 focus:border-blue-500 focus:ring-2 focus:ring-blue-500"
		>
			<option value={0}>No capo</option>
			{#each Array(12).fill(0) as _, i}
				<option value={i + 1}>Fret {i + 1}</option>
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
				<span class="ml-2 text-sm text-gray-700">
					Solo
					<span class="text-gray-500">(full bass)</span>
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
				<span class="ml-2 text-sm text-gray-700">
					Band
					<span class="text-gray-500">(lighter)</span>
				</span>
			</label>
		</div>
	</div>
</div>
