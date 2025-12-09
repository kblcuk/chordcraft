/**
 * Find page integration tests
 * Focus: Page renders correctly with expected elements
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render } from '@testing-library/svelte';
import FindPage from '../../../routes/find/+page.svelte';

// Mock SvelteKit modules
vi.mock('$app/state', () => ({
	page: {
		url: new URL('http://localhost/find'),
	},
}));

vi.mock('$app/navigation', () => ({
	goto: vi.fn(async () => {}),
}));

// Mock WASM module
vi.mock('$lib/wasm', () => ({
	initializeWasm: vi.fn().mockResolvedValue(undefined),
	findFingerings: vi.fn().mockResolvedValue([
		{
			tab: 'x32010',
			score: 85,
			voicingType: 'full',
			hasRootInBass: true,
			position: 0,
			notes: ['C', 'E', 'G', 'C', 'E'],
		},
	]),
	getInstrumentInfo: vi.fn().mockResolvedValue({
		stringCount: 6,
		stringNames: ['E', 'A', 'D', 'G', 'B', 'e'],
	}),
}));

describe('Find Page', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('should render the find page with title', async () => {
		const { getByText } = render(FindPage);
		expect(getByText('Find Fingerings')).toBeInTheDocument();
	});

	it('should render the chord input field', async () => {
		const { container } = render(FindPage);
		const input = container.querySelector('#chord-input') as HTMLInputElement;
		expect(input).toBeInTheDocument();
		expect(input.placeholder).toContain('Cmaj7');
	});

	it('should render example chord buttons', async () => {
		const { getByText } = render(FindPage);
		expect(getByText('C')).toBeInTheDocument();
		expect(getByText('Cmaj7')).toBeInTheDocument();
	});

	it('should render advanced options section', async () => {
		const { getByText } = render(FindPage);
		expect(getByText('Advanced Options')).toBeInTheDocument();
	});
});
