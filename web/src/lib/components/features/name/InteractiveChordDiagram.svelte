<script lang="ts">
	/**
	 * Interactive chord diagram for building fingerings visually
	 * Allows clicking on fretboard to select finger positions
	 *
	 * Supports multiple instruments with variable string counts:
	 * - Guitar: 6 strings
	 * - Ukulele: 4 strings
	 */

	import {
		VISIBLE_FRETS,
		DIMENSIONS,
		MARGIN_BOTTOM,
		MARGIN_SIDE,
		COLORS,
	} from '$lib/utils/fretboard-constants';
	import {
		parseTabNotation,
		generateTabNotation,
		transposeFingeringToNewPosition,
		detectBarres,
	} from '$lib/utils/tab-notation';

	// ============================================================================
	// Props
	// ============================================================================

	let {
		value = $bindable('000000'),
		size = 'medium',
		startFret = $bindable(0),
		capo = 0,
		stringCount = 6,
		stringNames = ['E', 'A', 'D', 'G', 'B', 'e'],
	}: {
		value?: string;
		size?: 'small' | 'medium' | 'large';
		startFret?: number;
		capo?: number;
		stringCount?: number;
		stringNames?: string[];
	} = $props();

	// ============================================================================
	// State
	// ============================================================================

	// Fingering state: -1 = muted, 0 = open, 1-24 = fret number
	// Indexed from low string (0) to high string (stringCount-1)
	let fingering = $state<number[]>(parseTabNotation(value));

	// Hover state for visual feedback
	let hoveredPosition = $state<{ string: number; fret: number } | null>(null);

	// ============================================================================
	// Computed Layout
	// ============================================================================

	let { width, height, dotRadius, marginTop } = $derived(DIMENSIONS[size]);
	let fretboardWidth = $derived(width - MARGIN_SIDE * 2);
	let fretboardHeight = $derived(height - marginTop - MARGIN_BOTTOM);
	let stringSpacing = $derived(fretboardWidth / (stringCount - 1));
	let fretSpacing = $derived(fretboardHeight / VISIBLE_FRETS);
	let endFret = $derived(startFret + VISIBLE_FRETS);
	let isOpenPosition = $derived(startFret === 0);

	// ============================================================================
	// Bidirectional Synchronization
	// ============================================================================

	/**
	 * Track previous position for transposition detection
	 */
	let previousStartFret = $state(startFret);

	/**
	 * Track previous value for change detection
	 */
	let previousValue = $state(value);

	/**
	 * Track previous string count to detect instrument changes (not initial render)
	 */
	let previousStringCount: number | undefined;

	/**
	 * When stringCount changes (instrument switch), reset fingering to match new string count.
	 * The old fingering doesn't make sense on a different instrument.
	 * Uses $effect.pre to track previous value; skips initial mount via undefined check.
	 */
	$effect.pre(() => {
		if (previousStringCount !== undefined && stringCount !== previousStringCount) {
			// Reset to all open strings for the new instrument
			fingering = Array(stringCount).fill(0);

			// Reset position to open
			startFret = 0;
			previousStartFret = 0;
		}
		previousStringCount = stringCount;
	});

	/**
	 * When startFret changes, transpose the fingering to match new position
	 */
	$effect(() => {
		if (startFret === previousStartFret) return;
		fingering = transposeFingeringToNewPosition(fingering, previousStartFret, startFret);
		previousStartFret = startFret;
	});

	/**
	 * When value prop changes (from text input), parse into fingering
	 * Only runs when value actually changes (not on every fingering change)
	 */
	$effect(() => {
		if (!value || value === previousValue) return;

		const parsed = parseTabNotation(value);
		// Only update fingering if it would produce a different result
		const currentGenerated = generateTabNotation(fingering, capo);
		if (currentGenerated !== value) {
			fingering = parsed;
		}
		previousValue = value;
	});

	/**
	 * When fingering changes (from visual interaction), generate tab notation
	 * Equality check prevents infinite loop
	 */
	$effect(() => {
		const newTab = generateTabNotation(fingering, capo);
		if (newTab === value) return;

		value = newTab;
		previousValue = newTab;
	});

	function handleFretClick(stringIndex: number, fret: number) {
		const currentValue = fingering[stringIndex];

		// If clicking the same position, cycle: fret -> open
		const isSameFret = currentValue === fret;
		fingering[stringIndex] = isSameFret ? 0 : fret;
	}

	function handleStringMarkerClick(stringIndex: number) {
		const currentValue = fingering[stringIndex];

		// Not set or fretted -> open
		const isMutedOrFretted = currentValue > 0 || currentValue === -1;
		fingering[stringIndex] = isMutedOrFretted ? 0 : -1;
	}

	export function clear() {
		fingering = Array(stringCount).fill(0);
	}

	const handleKeyPress = (handler: () => void) => (event: KeyboardEvent) => {
		if (event.key !== 'Enter' && event.key !== ' ') return;
		event.preventDefault();
		return handler();
	};

	// ============================================================================
	// Coordinate Calculations
	// ============================================================================

	function getStringX(stringIndex: number): number {
		return MARGIN_SIDE + stringIndex * stringSpacing;
	}

	function getFretY(fret: number): number {
		return marginTop + (fret - startFret) * fretSpacing;
	}

	function getDotY(fret: number): number {
		if (fret === 0) {
			return marginTop - 12; // Above fretboard for open strings
		}
		return getFretY(fret) - fretSpacing / 2;
	}

	function getStringStrokeWidth(stringIndex: number): number {
		return stringIndex === 0 || stringIndex === stringCount - 1 ? 2 : 1.5;
	}

	function getFretStrokeWidth(isNut: boolean): number {
		return isNut ? 4 : 1.5;
	}

	let barres = $derived(detectBarres(fingering));
</script>

<div class="space-y-3">
	<!-- Position slider -->
	<div class="flex items-center gap-3">
		<label for="position-slider" class="text-sm font-medium">
			Position: {startFret === 0 ? 'Open' : `Frets ${startFret + 1}-${endFret}`}
		</label>
		<input
			id="position-slider"
			type="range"
			min="0"
			max="19"
			bind:value={startFret}
			class="flex-1"
		/>
	</div>

	<!-- SVG Fretboard -->
	<svg
		{width}
		{height}
		viewBox="0 0 {width} {height}"
		class="bg-diagram rounded-lg"
		xmlns="http://www.w3.org/2000/svg"
	>
		<!-- Fret number indicator (for high positions) -->
		{#if !isOpenPosition}
			<text
				x="5"
				y={marginTop + fretSpacing / 2}
				class="text-xs font-medium select-none"
				text-anchor="start"
				dominant-baseline="central"
				fill={COLORS.fretNumber}
			>
				{startFret + 1}fr
			</text>
		{/if}

		<!-- Strings (vertical lines) -->
		{#each Array(stringCount)
			.fill(0)
			.map((_, i) => i) as stringIndex (stringIndex)}
			{@const x = getStringX(stringIndex)}
			<line
				x1={x}
				y1={marginTop}
				x2={x}
				y2={marginTop + fretboardHeight}
				stroke={COLORS.string}
				stroke-width={getStringStrokeWidth(stringIndex)}
				stroke-linecap="round"
			/>
		{/each}

		<!-- Frets (horizontal lines) -->
		{#each Array(VISIBLE_FRETS + 1)
			.fill(0)
			.map((v, i) => i) as fretIndex (fretIndex)}
			{@const fret = startFret + fretIndex}
			{@const y = getFretY(fret)}
			{@const isNut = fretIndex === 0 && startFret === 0}
			<line
				x1={MARGIN_SIDE}
				y1={y}
				x2={MARGIN_SIDE + fretboardWidth}
				y2={y}
				stroke={isNut ? COLORS.nut : COLORS.fret}
				stroke-width={getFretStrokeWidth(isNut)}
				stroke-linecap="butt"
			/>
		{/each}

		<!-- Barres (auto-detected) -->
		{#each barres as barre (`${barre.fret}-${barre.fromString}-${barre.toString}`)}
			{@const y = getDotY(barre.fret)}
			{@const x1 = getStringX(barre.fromString)}
			{@const x2 = getStringX(barre.toString)}
			<line
				{x1}
				y1={y}
				{x2}
				y2={y}
				stroke={COLORS.barre}
				stroke-width="6"
				stroke-linecap="round"
				opacity="0.6"
			/>
		{/each}

		<!-- Hover indicators -->
		{#if hoveredPosition && hoveredPosition.fret >= startFret && hoveredPosition.fret <= endFret}
			{@const x = getStringX(hoveredPosition.string)}
			{@const y = getDotY(hoveredPosition.fret)}
			<circle
				cx={x}
				cy={y}
				r={dotRadius}
				fill={COLORS.hoverDot}
				opacity="0.3"
				class="pointer-events-none"
			/>
		{/if}

		<!-- Finger positions (selected dots) -->
		{#each fingering as fret, stringIndex (stringIndex)}
			{@const x = getStringX(stringIndex)}
			{#if fret > 0}
				{@const y = getDotY(fret)}
				<circle
					cx={x}
					cy={y}
					r={dotRadius}
					fill={COLORS.selectedDot}
					class="drop-shadow-md"
				/>
			{:else if fret === 0}
				{@const y = marginTop - 12}
				<circle
					cx={x}
					cy={y}
					r={dotRadius - 2}
					fill="none"
					stroke={COLORS.openString}
					stroke-width="2"
					class="drop-shadow-sm"
				/>
			{/if}
		{/each}

		<!-- Muted string indicators (X) -->
		{#each fingering as fret, stringIndex (stringIndex)}
			{#if fret === -1}
				{@const x = getStringX(stringIndex)}
				<text
					{x}
					y={marginTop - 14}
					class="font-bold select-none"
					text-anchor="middle"
					font-size="14"
					fill={COLORS.mutedString}
				>
					×
				</text>
			{/if}
		{/each}

		<!-- Interactive click zones for frets -->
		{#if isOpenPosition}
			<!-- Open position: allow clicking frets 1-5 -->
			{#each Array(VISIBLE_FRETS)
				.fill(0)
				.map((_, i) => i) as fretIndex (fretIndex)}
				{@const fret = fretIndex + 1}
				{#each Array(stringCount)
					.fill(0)
					.map((_, i) => i) as stringIndex (`${fretIndex}-${stringIndex}`)}
					{@const x = getStringX(stringIndex)}
					{@const y = getDotY(fret)}
					{@const stringName = stringNames[stringIndex] || `String ${stringIndex + 1}`}
					{@const handler = () => handleFretClick(stringIndex, fret)}
					<circle
						cx={x}
						cy={y}
						r={dotRadius + 4}
						fill="transparent"
						class="cursor-pointer focus:ring-2 focus:ring-blue-500 focus:outline-none"
						role="button"
						tabindex="0"
						aria-label="Set {stringName} string to fret {fret}"
						onclick={handler}
						onkeydown={handleKeyPress(handler)}
						onmouseenter={() => (hoveredPosition = { string: stringIndex, fret })}
						onmouseleave={() => (hoveredPosition = null)}
					/>
				{/each}
			{/each}
		{:else}
			<!-- Higher position: allow clicking visible frets -->
			{#each Array(VISIBLE_FRETS)
				.fill(0)
				.map((_, i) => i) as fretIndex (fretIndex)}
				{@const fret = startFret + fretIndex + 1}
				{#each Array(stringCount)
					.fill(0)
					.map((_, i) => i) as stringIndex (`${fretIndex}-${stringIndex}`)}
					{@const x = getStringX(stringIndex)}
					{@const y = getDotY(fret)}
					{@const stringName = stringNames[stringIndex] || `String ${stringIndex + 1}`}
					{@const handler = () => handleFretClick(stringIndex, fret)}
					<circle
						cx={x}
						cy={y}
						r={dotRadius + 4}
						fill="transparent"
						class="cursor-pointer focus:ring-2 focus:ring-blue-500 focus:outline-none"
						role="button"
						tabindex="0"
						aria-label="Set {stringName} string to fret {fret}"
						onclick={handler}
						onkeydown={handleKeyPress(handler)}
						onmouseenter={() => (hoveredPosition = { string: stringIndex, fret })}
						onmouseleave={() => (hoveredPosition = null)}
					/>
				{/each}
			{/each}
		{/if}

		<!-- Interactive zones for string markers (open/muted toggle) -->
		{#each Array(stringCount)
			.fill(0)
			.map((_, i) => i) as stringIndex (stringIndex)}
			{@const x = getStringX(stringIndex)}
			{@const stringName = stringNames[stringIndex] || `String ${stringIndex + 1}`}
			{@const handler = () => handleStringMarkerClick(stringIndex)}
			<circle
				cx={x}
				cy={marginTop - 14}
				r={12}
				fill="transparent"
				class="cursor-pointer focus:ring-2 focus:ring-blue-500 focus:outline-none"
				role="button"
				tabindex="0"
				aria-label="Toggle {stringName} string: open, muted, or clear"
				onclick={handler}
				onkeydown={handleKeyPress(handler)}
			/>
		{/each}
	</svg>

	<!-- Clear button -->
	<button
		data-testid="clear-button"
		onclick={clear}
		class="text-sm text-muted-foreground underline hover:text-foreground"
	>
		Clear all
	</button>

	<!-- Current tab notation display -->
	<div class="rounded border border-border bg-muted p-2 text-center font-mono text-sm">
		{value || 'Click on the fretboard to build a chord'}
	</div>
</div>
