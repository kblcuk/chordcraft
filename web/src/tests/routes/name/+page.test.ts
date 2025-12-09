/**
 * Name page integration tests
 * Focus: Page renders correctly with expected elements
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, getByTestId } from '@testing-library/svelte';
import NamePage from '../../../routes/name/+page.svelte';

// Mock SvelteKit modules
vi.mock('$app/state', () => ({
	page: {
		url: new URL('http://localhost/name'),
	},
}));

vi.mock('$app/navigation', () => ({
	goto: vi.fn(async () => {}),
}));

// Mock WASM module
vi.mock('$lib/wasm', () => ({
	initializeWasm: vi.fn().mockResolvedValue(undefined),
	analyzeChord: vi.fn().mockResolvedValue([
		{
			name: 'C',
			confidence: 100,
			explanation: 'Complete C major chord',
		},
	]),
	getInstrumentInfo: vi.fn().mockResolvedValue({
		stringCount: 6,
		stringNames: ['E', 'A', 'D', 'G', 'B', 'e'],
	}),
}));

describe('Name Page', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('should render the name page with title', async () => {
		const { getByText } = render(NamePage);
		expect(getByText('Name Chord')).toBeInTheDocument();
	});

	it('should render the visual input section', async () => {
		const { getByText } = render(NamePage);
		expect(getByText('Visual Input')).toBeInTheDocument();
	});

	it('should render the text input section', async () => {
		const { getByText } = render(NamePage);
		expect(getByText('Text Input')).toBeInTheDocument();
	});

	it('should render the tab input field', async () => {
		const { container } = render(NamePage);
		const input = getByTestId(container, 'tab-input') as HTMLInputElement;
		expect(input).toBeInTheDocument();
	});

	it('should render the capo selector', async () => {
		const { container } = render(NamePage);
		const select = container.querySelector('#capo-select') as HTMLSelectElement;
		expect(select).toBeInTheDocument();
	});
});
