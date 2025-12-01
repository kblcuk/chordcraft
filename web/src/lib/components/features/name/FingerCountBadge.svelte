<script lang="ts">
	import { parseTabNotation } from '$lib/utils/tab-notation';

	/**
	 * Displays required finger count for a chord with creative labels
	 */

	let { tab }: { tab: string } = $props();

	// ============================================================================
	// Types
	// ============================================================================

	interface FingerCountInfo {
		count: number;
		label: string;
		emoji: string;
		color: string;
	}

	// ============================================================================
	// Finger Count Labels
	// ============================================================================

	const FINGER_LABELS: Record<number, { label: string; emoji: string; color: string }> = {
		0: {
			label: 'Open Strings Only',
			emoji: '🎵',
			color: 'bg-blue-100 text-blue-800 border-blue-200',
		},
		1: {
			label: 'One Finger Wonder',
			emoji: '☝️',
			color: 'bg-green-100 text-green-800 border-green-200',
		},
		2: {
			label: 'Ninja Turtle',
			emoji: '🐢',
			color: 'bg-green-100 text-green-800 border-green-200',
		},
		3: {
			label: 'Easy Peasy',
			emoji: '👌',
			color: 'bg-yellow-100 text-yellow-800 border-yellow-200',
		},
		4: {
			label: 'Human Standard',
			emoji: '🎸',
			color: 'bg-orange-100 text-orange-800 border-orange-200',
		},
		5: {
			label: 'Advanced (with thumb)',
			emoji: '🤘',
			color: 'bg-red-100 text-red-800 border-red-200',
		},
	};

	const AI_AUGMENTED = {
		label: 'AI-Augmented Human',
		emoji: '🤖',
		color: 'bg-purple-100 text-purple-800 border-purple-200',
	};

	// ============================================================================
	// Finger Count Algorithm
	// ============================================================================

	/**
	 * Find consecutive runs in sorted string indices
	 * Example: [0, 1, 2, 5, 6] → [[0, 1, 2], [5, 6]]
	 */
	function findConsecutiveRuns(strings: number[]): number[][] {
		if (strings.length === 0) return [];

		return strings.reduce<number[][]>((runs, str, i) => {
			if (i === 0 || str !== strings[i - 1] + 1) {
				// Start new run
				runs.push([str]);
			} else {
				// Continue current run
				runs[runs.length - 1].push(str);
			}
			return runs;
		}, []);
	}

	/**
	 * Count fingers for lowest fret (potential barre position)
	 * Uses span heuristic: if span ≥ 3, likely a barre
	 */
	function countFingersForLowestFret(strings: number[]): number {
		if (strings.length < 2) return strings.length;

		const span = strings[strings.length - 1] - strings[0] + 1;
		return span >= 3 ? 1 : strings.length; // Barre if span ≥ 3, else individual fingers
	}

	/**
	 * Count fingers for higher frets (consecutive run logic)
	 * Consecutive runs of 3+ strings = barre (1 finger)
	 */
	function countFingersForHigherFret(strings: number[]): number {
		const runs = findConsecutiveRuns(strings);
		return runs.reduce((total, run) => total + (run.length >= 3 ? 1 : run.length), 0);
	}

	/**
	 * Count required fingers for a chord
	 * Functional approach with map-reduce pattern
	 */
	function countFingers(tab: string): number {
		const positions = parseTabNotation(tab);
		if (positions.length === 0) return 0;

		// Group strings by fret using reduce
		const fretGroups = positions.reduce(
			(groups, fret, stringIndex) => {
				if (fret > 0) {
					if (!groups[fret]) groups[fret] = [];
					groups[fret].push(stringIndex);
				}
				return groups;
			},
			{} as Record<number, number[]>
		);

		const sortedFrets = Object.keys(fretGroups)
			.map(Number)
			.sort((a, b) => a - b);

		if (sortedFrets.length === 0) return 0;

		const lowestFret = sortedFrets[0];

		// Map each fret to finger count, then reduce to sum
		return sortedFrets.reduce((total, fret) => {
			const strings = fretGroups[fret].sort((a, b) => a - b);
			const fingerCount =
				fret === lowestFret
					? countFingersForLowestFret(strings)
					: countFingersForHigherFret(strings);
			return total + fingerCount;
		}, 0);
	}

	/**
	 * Get finger count info with label and styling
	 */
	function getFingerCountInfo(tab: string): FingerCountInfo {
		const count = countFingers(tab);

		if (count >= 6) {
			return { count, ...AI_AUGMENTED };
		}

		return {
			count,
			...(FINGER_LABELS[count] || FINGER_LABELS[4]), // Default to "Human Standard" if not found
		};
	}

	// ============================================================================
	// Reactive Computations
	// ============================================================================

	let fingerInfo = $derived(getFingerCountInfo(tab));

	// Check if there are any playable strings (not all muted)
	let hasPlayableStrings = $derived.by(() => {
		if (!tab.trim()) return false;
		const positions = parseTabNotation(tab);
		return positions.some((pos) => pos >= 0);
	});
</script>

{#if hasPlayableStrings}
	<div class="inline-flex items-center gap-2 rounded-lg border px-3 py-2 {fingerInfo.color}">
		<span class="text-2xl" role="img" aria-label={fingerInfo.label}>
			{fingerInfo.emoji}
		</span>
		<div class="flex flex-col">
			<span class="text-xs font-medium tracking-wide uppercase opacity-75">
				{fingerInfo.count}
				{fingerInfo.count === 1 ? 'Finger' : 'Fingers'}
			</span>
			<span class="font-semibold">{fingerInfo.label}</span>
		</div>
	</div>
{/if}
