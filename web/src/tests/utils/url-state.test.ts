/**
 * URL State utilities tests
 * Tests parsing and building URL params for all routes
 */

import { describe, it, expect } from 'vitest';
import {
	parseFindParams,
	parseNameParams,
	parseProgressionParams,
	buildFindParams,
	buildNameParams,
	buildProgressionParams,
	countFindFilters,
	countProgressionFilters,
	FIND_DEFAULTS,
	NAME_DEFAULTS,
	PROGRESSION_DEFAULTS,
} from '$lib/utils/url-state';

describe('URL State - Find Params', () => {
	describe('parseFindParams', () => {
		it('should return defaults for empty params', () => {
			const params = new URLSearchParams();
			const result = parseFindParams(params);

			expect(result.chord).toBe('');
			expect(result.limit).toBe(10);
			expect(result.capo).toBe(0);
			expect(result.voicing).toBe('all');
			expect(result.position).toBeNull();
			expect(result.context).toBe('solo');
			expect(result.instrument).toBe('guitar');
		});

		it('should parse all params correctly', () => {
			const params = new URLSearchParams({
				chord: 'Cmaj7',
				limit: '20',
				capo: '3',
				voicing: 'jazzy',
				position: '5',
				context: 'band',
				instrument: 'ukulele',
			});
			const result = parseFindParams(params);

			expect(result.chord).toBe('Cmaj7');
			expect(result.limit).toBe(20);
			expect(result.capo).toBe(3);
			expect(result.voicing).toBe('jazzy');
			expect(result.position).toBe(5);
			expect(result.context).toBe('band');
			expect(result.instrument).toBe('ukulele');
		});

		it('should handle invalid number values', () => {
			const params = new URLSearchParams({
				limit: 'invalid',
				capo: 'abc',
				position: 'xyz',
			});
			const result = parseFindParams(params);

			expect(result.limit).toBe(10); // default
			expect(result.capo).toBe(0); // default
			expect(result.position).toBeNull(); // default
		});

		it('should handle invalid voicing values', () => {
			const params = new URLSearchParams({ voicing: 'invalid' });
			const result = parseFindParams(params);
			expect(result.voicing).toBe('all');
		});

		it('should handle invalid context values', () => {
			const params = new URLSearchParams({ context: 'invalid' });
			const result = parseFindParams(params);
			expect(result.context).toBe('solo');
		});

		it('should handle invalid instrument values', () => {
			const params = new URLSearchParams({ instrument: 'omnichord' });
			const result = parseFindParams(params);
			expect(result.instrument).toBe('guitar');
		});
	});

	describe('buildFindParams', () => {
		it('should omit default values', () => {
			const state = {
				...FIND_DEFAULTS,
				chord: 'C',
				instrument: 'guitar' as const,
			};
			const result = buildFindParams(state);

			expect(result.chord).toBe('C');
			expect(result.limit).toBeUndefined();
			expect(result.capo).toBeUndefined();
			expect(result.voicing).toBeUndefined();
			expect(result.position).toBeNull();
			expect(result.context).toBeUndefined();
		});

		it('should include non-default values', () => {
			const state = {
				chord: 'Am7',
				limit: 20,
				capo: 3,
				voicing: 'jazzy' as const,
				position: 7,
				context: 'band' as const,
				instrument: 'ukulele' as const,
			};
			const result = buildFindParams(state);

			expect(result.chord).toBe('Am7');
			expect(result.limit).toBe(20);
			expect(result.capo).toBe(3);
			expect(result.voicing).toBe('jazzy');
			expect(result.position).toBe(7);
			expect(result.context).toBe('band');
			expect(result.instrument).toBe('ukulele');
		});
	});

	describe('countFindFilters', () => {
		it('should return 0 for default state', () => {
			const state = { ...FIND_DEFAULTS, chord: '', instrument: 'guitar' as const };
			expect(countFindFilters(state)).toBe(0);
		});

		it('should count non-default filters', () => {
			const state = {
				chord: 'C',
				limit: 20, // non-default
				capo: 3, // non-default
				voicing: 'jazzy' as const, // non-default
				position: 5, // non-default
				context: 'band' as const, // non-default
				instrument: 'guitar' as const,
			};
			expect(countFindFilters(state)).toBe(5);
		});
	});
});

describe('URL State - Name Params', () => {
	describe('parseNameParams', () => {
		it('should return defaults for empty params', () => {
			const params = new URLSearchParams();
			const result = parseNameParams(params);

			expect(result.tab).toBe('');
			expect(result.capo).toBe(0);
			expect(result.startFret).toBe(0);
			expect(result.instrument).toBe('guitar');
		});

		it('should parse all params correctly', () => {
			const params = new URLSearchParams({
				tab: 'x32010',
				capo: '2',
				startFret: '5',
				instrument: 'ukulele',
			});
			const result = parseNameParams(params);

			expect(result.tab).toBe('x32010');
			expect(result.capo).toBe(2);
			expect(result.startFret).toBe(5);
			expect(result.instrument).toBe('ukulele');
		});

		it('should handle ukulele tab notation (4 chars)', () => {
			const params = new URLSearchParams({
				tab: '0003',
				instrument: 'ukulele',
			});
			const result = parseNameParams(params);

			expect(result.tab).toBe('0003');
			expect(result.instrument).toBe('ukulele');
		});
	});

	describe('buildNameParams', () => {
		it('should omit default values', () => {
			const state = {
				...NAME_DEFAULTS,
				tab: 'x32010',
				instrument: 'guitar' as const,
			};
			const result = buildNameParams(state);

			expect(result.tab).toBe('x32010');
			expect(result.capo).toBeUndefined();
			expect(result.startFret).toBeUndefined();
		});

		it('should include non-default values', () => {
			const state = {
				tab: '022100',
				capo: 2,
				startFret: 3,
				instrument: 'ukulele' as const,
			};
			const result = buildNameParams(state);

			expect(result.tab).toBe('022100');
			expect(result.capo).toBe(2);
			expect(result.startFret).toBe(3);
			expect(result.instrument).toBe('ukulele');
		});
	});
});

describe('URL State - Progression Params', () => {
	describe('parseProgressionParams', () => {
		it('should return defaults for empty params', () => {
			const params = new URLSearchParams();
			const result = parseProgressionParams(params);

			expect(result.chords).toBe('');
			expect(result.limit).toBe(3);
			expect(result.maxDistance).toBe(3);
			expect(result.capo).toBe(0);
			expect(result.context).toBe('solo');
			expect(result.instrument).toBe('guitar');
		});

		it('should parse all params correctly', () => {
			const params = new URLSearchParams({
				chords: 'C G Am F',
				limit: '5',
				maxDistance: '5',
				capo: '2',
				context: 'band',
				instrument: 'ukulele',
			});
			const result = parseProgressionParams(params);

			expect(result.chords).toBe('C G Am F');
			expect(result.limit).toBe(5);
			expect(result.maxDistance).toBe(5);
			expect(result.capo).toBe(2);
			expect(result.context).toBe('band');
			expect(result.instrument).toBe('ukulele');
		});
	});

	describe('buildProgressionParams', () => {
		it('should omit default values', () => {
			const state = {
				...PROGRESSION_DEFAULTS,
				chords: 'C G Am F',
				instrument: 'guitar' as const,
			};
			const result = buildProgressionParams(state);

			expect(result.chords).toBe('C G Am F');
			expect(result.limit).toBeUndefined();
			expect(result.maxDistance).toBeUndefined();
			expect(result.capo).toBeUndefined();
			expect(result.context).toBeUndefined();
		});

		it('should include non-default values', () => {
			const state = {
				chords: 'Dm7 G7 Cmaj7',
				limit: 5,
				maxDistance: 5,
				capo: 2,
				context: 'band' as const,
				instrument: 'ukulele' as const,
			};
			const result = buildProgressionParams(state);

			expect(result.chords).toBe('Dm7 G7 Cmaj7');
			expect(result.limit).toBe(5);
			expect(result.maxDistance).toBe(5);
			expect(result.capo).toBe(2);
			expect(result.context).toBe('band');
			expect(result.instrument).toBe('ukulele');
		});
	});

	describe('countProgressionFilters', () => {
		it('should return 0 for default state', () => {
			const state = { ...PROGRESSION_DEFAULTS, chords: '', instrument: 'guitar' as const };
			expect(countProgressionFilters(state)).toBe(0);
		});

		it('should count non-default filters', () => {
			const state = {
				chords: 'C G Am F',
				limit: 5, // non-default
				maxDistance: 5, // non-default
				capo: 2, // non-default
				context: 'band' as const, // non-default
				instrument: 'guitar' as const,
			};
			expect(countProgressionFilters(state)).toBe(4);
		});
	});
});
