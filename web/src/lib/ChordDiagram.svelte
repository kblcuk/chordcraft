<script lang="ts">
	/**
	 * SVG-based chord diagram component
	 * Displays fingerings visually on a fretboard
	 *
	 * Supports multiple instruments with variable string counts:
	 * - Guitar: 6 strings
	 * - Ukulele: 4 strings
	 *
	 * Workshop Warmth Design - styled like a vintage chord chart
	 */

	import {
		VISIBLE_FRETS,
		DIMENSIONS,
		MARGIN_BOTTOM,
		MARGIN_SIDE,
		COLORS,
	} from '$lib/utils/fretboard-constants';

	// ============================================================================
	// Props
	// ============================================================================

	let {
		tab,
		notes = [],
		rootNote = '',
		size = 'medium',
		stringCount = 6,
		stringNames = ['E', 'A', 'D', 'G', 'B', 'e'],
	}: {
		tab: string;
		notes?: string[];
		rootNote?: string;
		size?: 'small' | 'medium' | 'large';
		stringCount?: number;
		stringNames?: string[];
	} = $props();

	// ============================================================================
	// Types
	// ============================================================================

	interface Position {
		string: number; // 0-based index (0 = lowest pitched string)
		fret: number; // 0 = open, 1-24 = fretted, -1 = muted
	}

	interface FingerPosition extends Position {
		note: string;
		isRoot: boolean;
		fingerNumber?: number;
	}

	interface Barre {
		fret: number;
		fromString: number;
		toString: number;
	}

	// ============================================================================
	// Computed Layout
	// ============================================================================

	let { width, height, dotRadius, marginTop } = $derived(DIMENSIONS[size]);
	let fretboardWidth = $derived(width - MARGIN_SIDE * 2);
	let fretboardHeight = $derived(height - marginTop - MARGIN_BOTTOM);
	let stringSpacing = $derived(fretboardWidth / (stringCount - 1));
	let fretSpacing = $derived(fretboardHeight / VISIBLE_FRETS);

	// ============================================================================
	// Pure Functions
	// ============================================================================

	/**
	 * Parse tab notation into positions
	 *
	 * Supports multiple formats:
	 * - Simple: "x32010" (single digit frets)
	 * - Parentheses: "x(10)(12)9(11)x" (multi-digit frets in parens)
	 * - With separators: "x-3-2-0-1-0" or "x 3 2 0 1 0"
	 *
	 * Format matches Rust backend implementation.
	 */
	const parseTab = (tab: string, maxStrings: number): Position[] => {
		const trimmed = tab.trim();
		if (!trimmed) return [];

		const positions: Position[] = [];
		let i = 0;
		let stringIndex = 0;

		while (i < trimmed.length && stringIndex < maxStrings) {
			const char = trimmed[i];

			// Muted string
			if (char === 'x' || char === 'X') {
				positions.push({ string: stringIndex, fret: -1 });
				stringIndex++;
				i++;
			}
			// Multi-digit fret in parentheses
			else if (char === '(') {
				const closeIndex = trimmed.indexOf(')', i);
				if (closeIndex === -1) break; // Invalid format

				const numStr = trimmed.substring(i + 1, closeIndex);
				const fret = parseInt(numStr, 10);

				if (!isNaN(fret)) {
					positions.push({ string: stringIndex, fret });
					stringIndex++;
				}
				i = closeIndex + 1;
			}
			// Single digit fret (0-9)
			else if (char >= '0' && char <= '9') {
				const fret = parseInt(char, 10);
				positions.push({ string: stringIndex, fret });
				stringIndex++;
				i++;
			}
			// Separators (space, dash, comma) - skip
			else if (char === ' ' || char === '-' || char === ',') {
				i++;
			}
			// Invalid character - skip
			else {
				i++;
			}
		}

		return positions;
	};

	/**
	 * Filter to fretted positions (fret > 0)
	 */
	const getFrettedPositions = (positions: Position[]): Position[] =>
		positions.filter((p) => p.fret > 0);

	/**
	 * Calculate the fret range to display
	 */
	const calculateFretRange = (positions: Position[]): [number, number] => {
		const fretted = getFrettedPositions(positions);

		if (fretted.length === 0) {
			return [0, VISIBLE_FRETS];
		}

		const frets = fretted.map((p) => p.fret);
		const minFret = Math.min(...frets);
		const maxFret = Math.max(...frets);

		return maxFret <= VISIBLE_FRETS
			? [0, VISIBLE_FRETS]
			: [Math.max(1, minFret - 1), Math.max(1, minFret - 1) + VISIBLE_FRETS];
	};

	/**
	 * Group positions by fret number
	 */
	const groupByFret = (positions: Position[]): Map<number, number[]> =>
		positions.reduce((acc, pos) => {
			const strings = acc.get(pos.fret) || [];
			acc.set(pos.fret, [...strings, pos.string]);
			return acc;
		}, new Map<number, number[]>());

	/**
	 * Check if a barre is continuous (no gaps)
	 */
	const isContinuousBarre = (
		positions: Position[],
		fret: number,
		fromString: number,
		toString: number
	): boolean =>
		Array.from({ length: toString - fromString + 1 }, (_, i) => fromString + i).every((s) => {
			const pos = positions.find((p) => p.string === s);
			return !pos || pos.fret === fret || pos.fret === -1;
		});

	/**
	 * Detect barre chords (functional style)
	 */
	const detectBarres = (positions: Position[]): Barre[] => {
		const fretted = getFrettedPositions(positions);
		const byFret = groupByFret(fretted);

		return Array.from(byFret.entries())
			.filter(([, strings]) => strings.length >= 2)
			.map(([fret, strings]) => {
				const sorted = [...strings].sort((a, b) => a - b);
				return {
					fret,
					fromString: sorted[0],
					toString: sorted[sorted.length - 1],
				};
			})
			.filter(
				(barre) =>
					barre.toString - barre.fromString >= 1 &&
					isContinuousBarre(positions, barre.fret, barre.fromString, barre.toString)
			);
	};

	/**
	 * Assign finger numbers (1-4) based on fret order
	 */
	const assignFingerNumbers = (positions: Position[]): Map<string, number> => {
		const fretted = getFrettedPositions(positions);
		const uniqueFrets = [...new Set(fretted.map((p) => p.fret))].sort((a, b) => a - b);

		return new Map(
			uniqueFrets.flatMap((fret, index) =>
				fretted
					.filter((p) => p.fret === fret)
					.map(
						(p) => [`${p.string}-${p.fret}`, Math.min(index + 1, 4)] as [string, number]
					)
			)
		);
	};

	/**
	 * Build finger positions with note information
	 */
	const buildFingerPositions = (
		positions: Position[],
		notes: string[],
		rootNote: string,
		fingerNumbers: Map<string, number>
	): FingerPosition[] =>
		positions
			.map((pos, index) => ({
				...pos,
				note: notes[index] || '',
				isRoot: notes[index] === rootNote,
				fingerNumber: fingerNumbers.get(`${pos.string}-${pos.fret}`),
			}))
			.filter((p) => p.fret >= 0); // Exclude muted strings

	/**
	 * Convert string index and fret to SVG coordinates
	 */
	const getCoordinates =
		(minFret: number, marginTop: number) =>
		(stringIndex: number, fret: number): { x: number; y: number } => ({
			x: MARGIN_SIDE + stringIndex * stringSpacing,
			y: marginTop + (fret - minFret) * fretSpacing,
		});

	/**
	 * Get Y position for a fret line
	 */
	const getFretY =
		(minFret: number, marginTop: number) =>
		(fret: number): number =>
			marginTop + (fret - minFret) * fretSpacing;

	// ============================================================================
	// Reactive Computations
	// ============================================================================

	let positions = $derived(parseTab(tab, stringCount));
	let [minFret] = $derived(calculateFretRange(positions));
	let isHighPosition = $derived(minFret > 0);
	let barres = $derived(detectBarres(positions));
	let fingerNumbers = $derived(assignFingerNumbers(positions));
	let fingerPositions = $derived(buildFingerPositions(positions, notes, rootNote, fingerNumbers));
	let getPosition = $derived(getCoordinates(minFret, marginTop));
	let getFretYPos = $derived(getFretY(minFret, marginTop));

	// ============================================================================
	// Helper Functions for Rendering
	// ============================================================================

	const getStringStrokeWidth = (stringIndex: number, totalStrings: number): number =>
		stringIndex === 0 || stringIndex === totalStrings - 1 ? 2 : 1.5;

	const getFretStrokeWidth = (isNut: boolean): number => (isNut ? 4 : 1.5);

	const getDisplayY = (fret: number, y: number): number =>
		fret === 0 ? marginTop - 12 : y - fretSpacing / 2;
</script>

<svg
	{width}
	{height}
	viewBox="0 0 {width} {height}"
	class="bg-diagram rounded-lg"
	xmlns="http://www.w3.org/2000/svg"
>
	<!-- Subtle wood grain texture (decorative) -->
	<defs>
		<pattern id="woodGrain" patternUnits="userSpaceOnUse" width="100" height="100">
			<rect width="100" height="100" fill="none" />
			<path
				d="M0 20 Q25 18 50 20 T100 20"
				stroke={COLORS.woodGrain}
				stroke-width="0.5"
				fill="none"
				opacity="0.5"
			/>
			<path
				d="M0 40 Q25 42 50 40 T100 40"
				stroke={COLORS.woodGrain}
				stroke-width="0.5"
				fill="none"
				opacity="0.5"
			/>
			<path
				d="M0 60 Q25 58 50 60 T100 60"
				stroke={COLORS.woodGrain}
				stroke-width="0.5"
				fill="none"
				opacity="0.5"
			/>
			<path
				d="M0 80 Q25 82 50 80 T100 80"
				stroke={COLORS.woodGrain}
				stroke-width="0.5"
				fill="none"
				opacity="0.5"
			/>
		</pattern>
	</defs>
	<rect {width} {height} fill="url(#woodGrain)" rx="8" />

	{#if isHighPosition}
		<text
			x="5"
			y={marginTop + fretSpacing / 2}
			class="text-xs font-medium select-none"
			text-anchor="start"
			dominant-baseline="central"
			fill={COLORS.fretNumber}
		>
			{minFret + 1}fr
		</text>
	{/if}

	{#each [...Array(stringCount).keys()] as stringIndex (stringIndex)}
		{@const x = MARGIN_SIDE + stringIndex * stringSpacing}
		<line
			x1={x}
			y1={marginTop}
			x2={x}
			y2={marginTop + fretboardHeight}
			stroke={COLORS.string}
			stroke-width={getStringStrokeWidth(stringIndex, stringCount)}
			stroke-linecap="round"
		/>
	{/each}

	{#each [...Array(VISIBLE_FRETS + 1).keys()] as fretIndex (fretIndex)}
		{@const y = getFretYPos(minFret + fretIndex)}
		{@const isNut = fretIndex === 0 && minFret === 0}
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

	{#each barres as barre (`${barre.fret}-${barre.fromString}-${barre.toString}`)}
		{@const y = getPosition(0, barre.fret).y - fretSpacing / 2}
		{@const x1 = MARGIN_SIDE + barre.fromString * stringSpacing}
		{@const x2 = MARGIN_SIDE + barre.toString * stringSpacing}
		<line
			{x1}
			y1={y}
			{x2}
			y2={y}
			stroke={COLORS.barre}
			stroke-width="6"
			stroke-linecap="round"
			opacity="0.7"
		/>
	{/each}

	{#each fingerPositions as pos (pos)}
		{@const { x, y } = getPosition(pos.string, pos.fret)}
		{@const displayY = getDisplayY(pos.fret, y)}
		{@const isOpen = pos.fret === 0}
		{@const strokeColor = pos.isRoot ? COLORS.rootOpenString : COLORS.openString}
		{@const fillColor = pos.isRoot ? COLORS.rootDot : COLORS.fingerDot}

		{#if isOpen}
			<circle
				cx={x}
				cy={displayY}
				r={dotRadius - 2}
				fill="none"
				stroke={strokeColor}
				stroke-width="2"
			/>
		{:else}
			<circle cx={x} cy={displayY + 1} r={dotRadius} fill="rgba(0,0,0,0.15)" />
			<circle cx={x} cy={displayY} r={dotRadius} fill={fillColor} />
			{#if pos.fingerNumber}
				<text
					{x}
					y={displayY}
					class="pointer-events-none font-bold select-none"
					text-anchor="middle"
					dominant-baseline="central"
					fill={COLORS.dotText}
					font-size={dotRadius * 1.2}
				>
					{pos.fingerNumber}
				</text>
			{/if}
		{/if}
	{/each}

	{#each positions.filter((p) => p.fret === -1) as pos (pos)}
		{@const x = MARGIN_SIDE + pos.string * stringSpacing}
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
	{/each}

	<!-- String names at bottom -->
	{#each [...Array(stringCount).keys()] as stringIndex (stringIndex)}
		{@const x = MARGIN_SIDE + stringIndex * stringSpacing}
		{@const stringName = stringNames[stringIndex] || `${stringIndex + 1}`}
		<text
			{x}
			y={height - 8}
			class="text-xs select-none"
			text-anchor="middle"
			fill={COLORS.fretNumber}
			font-size="10"
		>
			{stringName}
		</text>
	{/each}
</svg>
