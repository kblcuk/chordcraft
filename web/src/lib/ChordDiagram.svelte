<script lang="ts">
	/**
	 * SVG-based chord diagram component
	 * Displays guitar fingerings visually on a fretboard
	 */

	// ============================================================================
	// Props
	// ============================================================================

	export let tab: string; // e.g., "x32010"
	export let notes: string[] = []; // e.g., ["C", "E", "G", "C", "E"]
	export let rootNote: string = ''; // e.g., "C"
	export let size: 'small' | 'medium' | 'large' = 'medium';

	// ============================================================================
	// Types
	// ============================================================================

	interface Position {
		string: number; // 0-5 (low E to high E)
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
	// Constants
	// ============================================================================

	const STRING_COUNT = 6;
	const VISIBLE_FRETS = 5;

	// Size-based dimensions using Tailwind scale
	const DIMENSIONS = {
		small: { width: 120, height: 160, dotRadius: 6, marginTop: 30 },
		medium: { width: 160, height: 200, dotRadius: 8, marginTop: 35 },
		large: { width: 200, height: 250, dotRadius: 10, marginTop: 40 },
	} as const;

	const MARGIN_BOTTOM = 20;
	const MARGIN_SIDE = 25; // Increased from 20 to prevent overflow

	// Tailwind color mappings for SVG
	const COLORS = {
		string: '#1f2937', // gray-800
		fret: '#1f2937', // gray-800
		nut: '#111827', // gray-900
		fingerDot: '#1f2937', // gray-800
		rootDot: '#2563eb', // blue-600
		openString: '#1f2937', // gray-800
		rootOpenString: '#2563eb', // blue-600
		barre: '#4b5563', // gray-600
		mutedString: '#9ca3af', // gray-400
		fretNumber: '#6b7280', // gray-500
	} as const;

	// ============================================================================
	// Computed Layout
	// ============================================================================

	$: ({ width, height, dotRadius, marginTop } = DIMENSIONS[size]);
	$: fretboardWidth = width - MARGIN_SIDE * 2;
	$: fretboardHeight = height - marginTop - MARGIN_BOTTOM;
	$: stringSpacing = fretboardWidth / (STRING_COUNT - 1);
	$: fretSpacing = fretboardHeight / VISIBLE_FRETS;

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
	const parseTab = (tab: string): Position[] => {
		const trimmed = tab.trim();
		if (!trimmed) return [];

		const positions: Position[] = [];
		let i = 0;
		let stringIndex = 0;

		while (i < trimmed.length && stringIndex < STRING_COUNT) {
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
			.filter(([_, strings]) => strings.length >= 2)
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

	$: positions = parseTab(tab);
	$: [minFret, maxFret] = calculateFretRange(positions);
	$: isHighPosition = minFret > 0;
	$: barres = detectBarres(positions);
	$: fingerNumbers = assignFingerNumbers(positions);
	$: fingerPositions = buildFingerPositions(positions, notes, rootNote, fingerNumbers);
	$: getPosition = getCoordinates(minFret, marginTop);
	$: getFretYPos = getFretY(minFret, marginTop);

	// ============================================================================
	// Helper Functions for Rendering
	// ============================================================================

	const getStringStrokeWidth = (stringIndex: number): number =>
		stringIndex === 0 || stringIndex === 5 ? 2 : 1.5;

	const getFretStrokeWidth = (isNut: boolean): number => (isNut ? 4 : 1.5);

	const getDisplayY = (fret: number, y: number): number =>
		fret === 0 ? marginTop - 12 : y - fretSpacing / 2;
</script>

<svg
	{width}
	{height}
	viewBox="0 0 {width} {height}"
	class="bg-white rounded-lg border border-gray-200 p-1"
	xmlns="http://www.w3.org/2000/svg"
>
	<!-- Fret number indicator (for high positions) -->
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

	<!-- Strings (vertical lines) -->
	{#each Array(STRING_COUNT).fill(0) as _, stringIndex}
		{@const x = MARGIN_SIDE + stringIndex * stringSpacing}
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
	{#each Array(VISIBLE_FRETS + 1).fill(0) as _, fretIndex}
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

	<!-- Barres (lines behind finger dots) -->
	{#each barres as barre}
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
			opacity="0.6"
		/>
	{/each}

	<!-- Finger positions (dots) -->
	{#each fingerPositions as pos}
		{@const { x, y } = getPosition(pos.string, pos.fret)}
		{@const displayY = getDisplayY(pos.fret, y)}
		{@const isOpen = pos.fret === 0}
		{@const strokeColor = pos.isRoot ? COLORS.rootOpenString : COLORS.openString}
		{@const fillColor = pos.isRoot ? COLORS.rootDot : COLORS.fingerDot}

		{#if isOpen}
			<!-- Open string circle -->
			<circle
				cx={x}
				cy={displayY}
				r={dotRadius - 2}
				fill="none"
				stroke={strokeColor}
				stroke-width="2"
				class="drop-shadow-sm"
			/>
		{:else}
			<!-- Fretted position dot -->
			<circle cx={x} cy={displayY} r={dotRadius} fill={fillColor} class="drop-shadow-md" />
			{#if pos.fingerNumber}
				<text
					{x}
					y={displayY}
					class="select-none pointer-events-none font-bold"
					text-anchor="middle"
					dominant-baseline="central"
					fill="white"
					font-size={dotRadius * 1.2}
				>
					{pos.fingerNumber}
				</text>
			{/if}
		{/if}
	{/each}

	<!-- Muted string indicators (X) at top -->
	{#each positions.filter((p) => p.fret === -1) as pos}
		{@const x = MARGIN_SIDE + pos.string * stringSpacing}
		<text
			{x}
			y={marginTop - 14}
			class="select-none font-bold"
			text-anchor="middle"
			font-size="14"
			fill={COLORS.mutedString}
		>
			×
		</text>
	{/each}
</svg>
