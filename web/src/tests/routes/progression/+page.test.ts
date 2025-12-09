/**
 * Progression page integration tests
 * Focus: Page renders correctly with expected elements
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render } from '@testing-library/svelte';
import ProgressionPage from '../../../routes/progression/+page.svelte';

// Mock SvelteKit modules
vi.mock('$app/state', () => ({
	page: {
		url: new URL('http://localhost/progression'),
	},
}));

vi.mock('$app/navigation', () => ({
	goto: vi.fn(async () => {}),
}));

// Mock WASM module
vi.mock('$lib/wasm', () => ({
	initializeWasm: vi.fn().mockResolvedValue(undefined),
	generateProgression: vi.fn().mockResolvedValue([]),
	getInstrumentInfo: vi.fn().mockResolvedValue({
		stringCount: 6,
		stringNames: ['E', 'A', 'D', 'G', 'B', 'e'],
	}),
}));

describe('Progression Page', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('should render the progression page with title', async () => {
		const { getByText } = render(ProgressionPage);
		expect(getByText('Chord Progression')).toBeInTheDocument();
	});

	it('should render the progression input field', async () => {
		const { container } = render(ProgressionPage);
		const input = container.querySelector('#progression-input') as HTMLInputElement;
		expect(input).toBeInTheDocument();
		expect(input.placeholder).toContain('Cmaj7');
	});

	it('should render common progression buttons', async () => {
		const { getByText } = render(ProgressionPage);
		expect(getByText('I-IV-V in C')).toBeInTheDocument();
		expect(getByText('I-V-vi-IV')).toBeInTheDocument();
	});

	it('should render advanced options section', async () => {
		const { getByText } = render(ProgressionPage);
		expect(getByText('Advanced Options')).toBeInTheDocument();
	});
});
