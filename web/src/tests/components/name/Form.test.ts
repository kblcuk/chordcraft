/**
 * Form component tests
 * Focus: Validation behavior and partial input support
 */

import { describe, it, expect, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import FormTestWrapper from './FormTestWrapper.svelte';

describe('Form - Validation', () => {
	it('should allow typing partial input like "(" without propagating', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();
		expect(wrapper).toBeInTheDocument();

		// Type incomplete input
		await fireEvent.input(input!, { target: { value: '1(' } });

		// Input shows what user typed
		expect(input!.value).toBe('1(');

		// But doesn't propagate to parent (unbalanced brackets)
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('000000');
		});
	});

	it('should propagate complete valid input like "(10)"', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		// Type complete valid input
		await fireEvent.input(input!, { target: { value: '(10)' } });

		// Both input and bound value updated
		expect(input!.value).toBe('(10)');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('(10)');
		});
	});

	it('should allow typing multi-digit frets step by step', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		// Step 1: type "x"
		await fireEvent.input(input!, { target: { value: 'x' } });
		expect(input!.value).toBe('x');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('x');
		});

		// Step 2: type "x3"
		await fireEvent.input(input!, { target: { value: 'x3' } });
		expect(input!.value).toBe('x3');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('x3');
		});

		// Step 3: type "x3(" - incomplete
		await fireEvent.input(input!, { target: { value: 'x3(' } });
		expect(input!.value).toBe('x3(');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('x3');
		});

		// Step 4: type "x3(1" - still incomplete
		await fireEvent.input(input!, { target: { value: 'x3(1' } });
		expect(input!.value).toBe('x3(1');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('x3');
		});

		// Step 5: type "x3(12" - still incomplete
		await fireEvent.input(input!, { target: { value: 'x3(12' } });
		expect(input!.value).toBe('x3(12');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('x3');
		});

		// Step 6: type "x3(12)" - complete!
		await fireEvent.input(input!, { target: { value: 'x3(12)' } });
		expect(input!.value).toBe('x3(12)');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('x3(12)');
		});
	});

	it('should reject invalid characters', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		// Type invalid input with letters
		await fireEvent.input(input!, { target: { value: 'abc' } });

		// Input shows what user typed
		expect(input!.value).toBe('abc');

		// But doesn't propagate (invalid characters)
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('000000');
		});
	});

	it('should handle unbalanced brackets - closing before opening', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		await fireEvent.input(input!, { target: { value: ')10(' } });

		expect(input!.value).toBe(')10(');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('000000');
		});
	});

	it('should handle multiple bracketed numbers', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		await fireEvent.input(input!, { target: { value: 'x(12)(14)x' } });

		expect(input!.value).toBe('x(12)(14)x');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('x(12)(14)x');
		});
	});

	it('should allow empty string', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		await fireEvent.input(input!, { target: { value: '' } });

		expect(input!.value).toBe('');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('');
		});
	});
});

describe('Form - Error Display', () => {
	it('should not show error while typing invalid input', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		// Type invalid input
		await fireEvent.input(input!, { target: { value: 'abc' } });

		// Error should NOT be visible yet (no blur)
		expect(container.textContent).not.toContain('Valid tab notation');
	});

	it('should show error after blur with invalid input', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		// Type invalid input
		await fireEvent.input(input!, { target: { value: 'abc' } });

		// Blur the input
		await fireEvent.blur(input!);

		// Now error should be visible
		await vi.waitFor(() => {
			expect(container.textContent).toContain('Valid tab notation');
		});
	});

	it('should not show error after blur with valid input', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		// Type valid input
		await fireEvent.input(input!, { target: { value: 'x32010' } });

		// Blur the input
		await fireEvent.blur(input!);

		// Error should NOT be visible (input is valid)
		expect(container.textContent).not.toContain('Valid tab notation');
	});

	it('should show error for partial input (unbalanced brackets) after blur', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		// Type partial input
		await fireEvent.input(input!, { target: { value: 'x3(12' } });

		// Blur the input
		await fireEvent.blur(input!);

		// Error should be visible (unbalanced brackets)
		await vi.waitFor(() => {
			expect(container.textContent).toContain('Valid tab notation');
		});
	});
});

describe('Form - Clear Button', () => {
	it('should reset to open strings (000000) when clear button is clicked', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: 'x32010' },
		});

		const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();
		expect(input!.value).toBe('x32010');

		// Find and click clear button
		const buttons = container.querySelectorAll('button');
		const clearButton: HTMLButtonElement | undefined = Array.from(buttons).find(
			(btn) => btn.textContent === 'X'
		);

		expect(clearButton).toBeInTheDocument();
		await fireEvent.click(clearButton!);

		// Should reset to open strings
		expect(input!.value).toBe('000000');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('000000');
		});
	});

	it('should hide clear button when input is empty', () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '' },
		});

		// Clear button should not be visible
		const buttons = container.querySelectorAll('button');
		const clearButton: HTMLButtonElement | undefined = Array.from(buttons).find(
			(btn) => btn.textContent === 'X'
		);

		expect(clearButton).toBeUndefined();
	});

	it('should show clear button when input has value', () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: 'x32010' },
		});

		// Clear button should be visible
		const buttons = container.querySelectorAll('button');
		const clearButton: HTMLButtonElement | undefined = Array.from(buttons).find(
			(btn) => btn.textContent === 'X'
		);

		expect(clearButton).toBeInTheDocument();
	});
});

describe('Form - Example Buttons', () => {
	it('should set input value when example button is clicked', async () => {
		const { container } = render(FormTestWrapper, {
			props: { initialValue: '000000' },
		});

		const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
		const input: HTMLInputElement | null = container.querySelector(
			'input[data-testid="tab-input"]'
		);

		expect(input).toBeInTheDocument();

		// Find first example button (should be "x32010 (C)")
		const buttons = container.querySelectorAll('button');
		const firstExample: HTMLButtonElement | undefined = Array.from(buttons).find((btn) =>
			btn.textContent?.includes('x32010')
		);

		expect(firstExample).toBeInTheDocument();
		await fireEvent.click(firstExample!);

		// Should update both input and bound value
		expect(input!.value).toBe('x32010');
		await vi.waitFor(() => {
			expect(wrapper!.getAttribute('data-bound-value')).toBe('x32010');
		});
	});
});
