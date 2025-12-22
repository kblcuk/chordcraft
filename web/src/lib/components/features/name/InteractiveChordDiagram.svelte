<script lang="ts">
	/**
	 * Interactive chord diagram for building fingerings visually
	 * Allows clicking on fretboard to select finger positions
	 *
	 * Supports multiple instruments with variable string counts:
	 * - Guitar: 6 strings
	 * - Ukulele: 4 strings
	 *
	 * Features:
	 * - Subtle CSS animations for visual feedback
	 * - Mobile-optimized touch targets
	 * - Accessible keyboard navigation
	 */

	import { VISIBLE_FRETS, DIMENSIONS, MARGIN_SIDE, COLORS } from '$lib/utils/fretboard-constants';
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

	// ============================================================================
	// Computed Layout
	// ============================================================================

	let { width, height, dotRadius, marginTop, marginBottom } = $derived(DIMENSIONS[size]);
	let fretboardWidth = $derived(width - MARGIN_SIDE * 2);
	let fretboardHeight = $derived(height - marginTop - marginBottom);
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
	 * Track the last value we generated ourselves in the fingering→value effect.
	 * This helps us distinguish between:
	 * - Parent echoing back our generated value (should skip)
	 * - User actually typing a new value (should parse)
	 */
	let lastGeneratedValue: string | undefined;

	/**
	 * When value prop changes (from text input), parse into fingering
	 * Only runs when value actually changes (not on every fingering change)
	 *
	 * IMPORTANT: When capo changes, the parent's bound value becomes "stale"
	 * (it was generated with the OLD capo). We must detect this and NOT
	 * re-parse the stale value into fingering, which would corrupt it.
	 */
	$effect(() => {
		if (!value || value === previousValue) {
			return;
		}

		// If this value matches what we ourselves last generated, skip.
		// This handles the case where parent re-renders with stale value after
		// we already generated a new one - we don't want to parse our own output.
		if (lastGeneratedValue !== undefined && value !== lastGeneratedValue) {
			// Check if this is an old value we generated (parent echoing stale binding)
			// by seeing if we can regenerate it with current fingering + any capo
			const couldBeStale =
				generateTabNotation(fingering, capo) !== value && // Not current
				previousValue === lastGeneratedValue; // We just generated something else

			if (couldBeStale) {
				// Parent is echoing a stale value - ignore it, let fingering→value fix it
				previousValue = value;
				return;
			}
		}

		// User actually typed something new - parse it
		const parsed = parseTabNotation(value);
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

		lastGeneratedValue = newTab;
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

	// Touch target size (larger for easier tapping)
	let touchTargetRadius = $derived(dotRadius + 6);

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

<div class="interactive-fretboard flex flex-col items-center space-y-3">
	<!-- Position slider -->
	<div class="flex items-center gap-3">
		<label for="position-slider" class="text-sm font-medium whitespace-nowrap">
			Position: {startFret === 0 ? 'Open' : `Frets ${startFret + 1}-${endFret}`}
		</label>
		<input
			id="position-slider"
			type="range"
			min="0"
			max="19"
			bind:value={startFret}
			class="h-8 flex-1 touch-pan-x"
		/>
	</div>

	<!-- SVG Fretboard -->
	<svg
		{width}
		{height}
		viewBox="0 0 {width} {height}"
		class="bg-diagram rounded-lg select-none"
		style="touch-action: manipulation;"
		xmlns="http://www.w3.org/2000/svg"
	>
		<!-- Fret number indicator (for high positions) -->
		{#if !isOpenPosition}
			<text
				x={startFret > 8 ? 0 : 5}
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

		<!-- Finger positions (selected dots) -->
		{#each fingering as fret, stringIndex (`finger-${stringIndex}-${fret}`)}
			{@const x = getStringX(stringIndex)}
			{#if fret > 0}
				{@const y = getDotY(fret)}
				<circle cx={x} cy={y} r={dotRadius} fill={COLORS.selectedDot} class="finger-dot" />
			{:else if fret === 0}
				{@const y = marginTop - 12}
				<circle
					cx={x}
					cy={y}
					r={dotRadius - 2}
					fill="none"
					stroke={COLORS.openString}
					stroke-width="2"
					class="open-string"
				/>
			{/if}
		{/each}

		<!-- Muted string indicators (X) -->
		{#each fingering as fret, stringIndex (`muted-${stringIndex}-${fret}`)}
			{#if fret === -1}
				{@const x = getStringX(stringIndex)}
				<text
					{x}
					y={marginTop - 14}
					class="muted-marker select-none"
					text-anchor="middle"
					font-size="14"
					font-weight="bold"
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
						r={touchTargetRadius}
						class="hit-zone focus-visible:outline-2 focus-visible:outline-(--diagram-selected)"
						role="button"
						tabindex="0"
						aria-label="Set {stringName} string to fret {fret}"
						onclick={handler}
						onkeydown={handleKeyPress(handler)}
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
						r={touchTargetRadius}
						class="hit-zone focus-visible:outline-2 focus-visible:outline-(--diagram-selected)"
						role="button"
						tabindex="0"
						aria-label="Set {stringName} string to fret {fret}"
						onclick={handler}
						onkeydown={handleKeyPress(handler)}
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
				r={14}
				class="hit-zone focus-visible:outline-2 focus-visible:outline-(--diagram-selected)"
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
		class="text-sm text-muted-foreground underline transition-colors hover:text-foreground"
	>
		Clear all
	</button>

	<!-- Current tab notation display -->
	<div class="w-full rounded border border-border bg-muted p-2 text-center font-mono text-sm">
		{value || 'Click on the fretboard to build a chord'}
	</div>
</div>

<style>
	/* Custom animation for finger dot appearance - scale + fade not in Tailwind */
	@keyframes dot-appear {
		from {
			transform: scale(0.7);
			opacity: 0.6;
		}
	}

	/* Finger dots */
	:global(.interactive-fretboard .finger-dot),
	:global(.interactive-fretboard .open-string),
	:global(.interactive-fretboard .muted-marker) {
		animation: dot-appear 0.12s ease-out;
		transform-origin: center;
		transform-box: fill-box;
	}

	:global(.interactive-fretboard .finger-dot) {
		filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.15));
	}

	/* Hit zones - hover/active via CSS */
	:global(.interactive-fretboard .hit-zone) {
		cursor: pointer;
		fill: transparent;
	}

	:global(.interactive-fretboard .hit-zone:hover) {
		fill: var(--diagram-hover);
		fill-opacity: 0.25;
	}

	:global(.interactive-fretboard .hit-zone:active) {
		fill: var(--diagram-hover);
		fill-opacity: 0.4;
	}

	/* Slider thumb - needs vendor prefixes, can't use Tailwind */
	:global(.interactive-fretboard input[type='range']) {
		-webkit-appearance: none;
		appearance: none;
		background: transparent;
	}

	:global(.interactive-fretboard input[type='range']::-webkit-slider-runnable-track) {
		height: 6px;
		background: var(--muted);
		border-radius: 50%;
	}

	:global(.interactive-fretboard input[type='range']::-webkit-slider-thumb) {
		-webkit-appearance: none;
		width: 24px;
		height: 24px;
		background: var(--diagram-selected);
		border-radius: 50%;
		margin-top: -9px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
	}

	:global(.interactive-fretboard input[type='range']::-moz-range-track) {
		height: 6px;
		background: var(--muted);
		border-radius: 50%;
	}

	:global(.interactive-fretboard input[type='range']::-moz-range-thumb) {
		width: 24px;
		height: 24px;
		background: var(--diagram-selected);
		border-radius: 50%;
		border: none;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
	}

	@media (pointer: coarse) {
		:global(.interactive-fretboard input[type='range']::-webkit-slider-thumb) {
			width: 28px;
			height: 28px;
			margin-top: -11px;
		}

		:global(.interactive-fretboard input[type='range']::-moz-range-thumb) {
			width: 28px;
			height: 28px;
		}
	}
</style>
