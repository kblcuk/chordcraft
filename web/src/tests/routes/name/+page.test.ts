/**
 * Name page integration tests
 * Focus: Core user interactions for chord identification
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, fireEvent, getByTestId } from '@testing-library/svelte';
import { get } from 'svelte/store';
import NamePage from '../../../routes/name/+page.svelte';
import { nameStore } from '$lib/stores/name';

// Mock WASM module
vi.mock('$lib/wasm', () => ({
	initializeWasm: vi.fn().mockResolvedValue(undefined),
	analyzeChord: vi.fn().mockResolvedValue([
		{
			name: 'C',
			confidence: 100,
			explanation: 'Complete C major chord',
		},
		{
			name: 'Am/C',
			confidence: 85,
			explanation: 'A minor chord with C in bass',
		},
	]),
}));

// Mock SvelteKit navigation
vi.mock('$app/navigation', () => ({
	goto: vi.fn(async () => {}),
	beforeNavigate: vi.fn(),
	afterNavigate: vi.fn(),
}));

describe('Name Page - Core User Flows', () => {
	beforeEach(() => {
		nameStore.clear();
	});
	afterEach(() => {
		vi.clearAllMocks();
	});

	it('should display multiple chord interpretations with confidence scores', async () => {
		const { container } = render(NamePage);

		const input = getByTestId(container, 'tab-input') as HTMLInputElement;
		await fireEvent.input(input, { target: { value: 'x32010' } });
		await fireEvent.keyDown(input, { key: 'Enter' });

		await vi.waitFor(() => {
			const state = get(nameStore);
			expect(state.results.length).toBe(2);
			expect(state.results[0].name).toBe('C');
			expect(state.results[0].confidence).toBe(100);
			expect(state.results[1].name).toBe('Am/C');
			expect(state.results[1].confidence).toBe(85);
		});
	});

	it('should clear input and results when store.clear() is called', () => {
		// Set up some state first by updating the store
		nameStore.setTabInput('x32010');

		let state = get(nameStore);
		expect(state.tabInput).toBe('x32010');

		// Clear
		nameStore.clear();

		state = get(nameStore);
		expect(state.tabInput).toBe('');
		expect(state.results).toEqual([]);
	});
});
