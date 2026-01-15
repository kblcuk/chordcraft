/**
 * InstrumentSelector Component Tests
 * Tests the instrument dropdown selector UI
 */

import { describe, it, expect, vi } from 'vitest';
import { render } from '@testing-library/svelte';
import InstrumentSelector from '$lib/components/shared/InstrumentSelector.svelte';
import { INSTRUMENT_METADATA, INSTRUMENT_CATEGORIES } from '$lib/wasm';

// Mock the page store
vi.mock('$app/state', () => ({
	page: {
		url: {
			pathname: '/find',
			searchParams: new URLSearchParams(),
		},
	},
}));

// Mock updateUrl
vi.mock('$lib/utils/url-state', () => ({
	updateUrl: vi.fn(),
}));

describe('InstrumentSelector', () => {
	it('should render without crashing', () => {
		const { container } = render(InstrumentSelector);
		expect(container).toBeTruthy();
	});

	it('should display guitar icon for standard instruments', () => {
		const { container } = render(InstrumentSelector);
		// Guitar icon should be present by default
		const icon = container.querySelector('svg');
		expect(icon).toBeTruthy();
	});

	it('should have all 11 instruments defined in metadata', () => {
		const allInstruments = [
			...INSTRUMENT_CATEGORIES.standard,
			...INSTRUMENT_CATEGORIES['alternate-tuning'],
		];

		expect(allInstruments).toHaveLength(11);

		allInstruments.forEach((inst) => {
			const meta = INSTRUMENT_METADATA[inst];
			expect(meta).toBeDefined();
			expect(meta.label).toBeTruthy();
			expect(meta.tuning).toBeTruthy();
			expect(meta.stringCount).toBeGreaterThan(0);
		});
	});

	it('should have 7 standard instruments', () => {
		expect(INSTRUMENT_CATEGORIES.standard).toHaveLength(7);
		expect(INSTRUMENT_CATEGORIES.standard).toEqual([
			'guitar',
			'ukulele',
			'baritone-ukulele',
			'bass',
			'bass-5',
			'mandolin',
			'banjo',
		]);
	});

	it('should have 4 alternate tunings', () => {
		expect(INSTRUMENT_CATEGORIES['alternate-tuning']).toHaveLength(4);
		expect(INSTRUMENT_CATEGORIES['alternate-tuning']).toEqual([
			'drop-d',
			'open-g',
			'dadgad',
			'guitar-7',
		]);
	});

	it('should display correct tuning for each instrument', () => {
		const expectedTunings: Record<string, string> = {
			guitar: 'EADGBE',
			ukulele: 'GCEA',
			'baritone-ukulele': 'DGBE',
			bass: 'EADG',
			'bass-5': 'BEADG',
			mandolin: 'GDAE',
			banjo: 'gDGBD',
			'guitar-7': 'BEADGBE',
			'drop-d': 'DADGBE',
			'open-g': 'DGDGBD',
			dadgad: 'DADGAD',
		};

		Object.entries(expectedTunings).forEach(([instrument, tuning]) => {
			expect(INSTRUMENT_METADATA[instrument as keyof typeof INSTRUMENT_METADATA].tuning).toBe(
				tuning
			);
		});
	});
});
