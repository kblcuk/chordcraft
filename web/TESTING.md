# Testing Guide

## Philosophy

**Test user-facing behavior, not implementation details.**

We focus on:

- ✅ **What the user experiences**: Does the chord diagram render correctly?
- ✅ **Input/output behavior**: Does parsing handle various tab formats?
- ✅ **Visual elements**: Are strings, frets, and dots visible?
- ❌ **NOT internal functions**: How exactly does `calculateFretRange` work internally

## Why This Approach?

1. **Resilient to refactoring**: Tests don't break when you improve implementation
2. **Documentation**: Tests show how users interact with components
3. **Confidence**: If tests pass, users will have a good experience

## Test Structure

```
web/
├── src/
│   ├── lib/
│   │   ├── ChordDiagram.svelte          # Component
│   │   └── ChordDiagram.test.ts         # Tests (user-focused)
│   └── test/
│       └── setup.ts                     # Test setup/config
├── vitest.config.ts                     # Vitest configuration
└── TESTING.md                           # This file
```

## Running Tests

```bash
# Watch mode (runs on file changes)
bun test

# Run once (CI/CD)
bun run test:run

# With UI (visual test runner)
bun run test:ui

# With coverage report
bun run test:coverage
```

## Test Categories

### 1. Tab Format Support

Tests that different input formats all work correctly:

- `"x32010"` - Simple single-digit
- `"x(10)(12)9x"` - Mixed single/multi-digit
- `"x-3-2-0-1-0"` - With separators

**Why**: Users will input tabs in various formats from different sources.

### 2. Edge Cases

Tests boundary conditions:

- Empty strings
- All muted strings (`xxxxxx`)
- All open strings (`000000`)
- Whitespace handling

**Why**: Users make mistakes; app should handle gracefully.

### 3. Visual Elements

Tests that expected DOM elements are present:

- 6 vertical lines (strings)
- Horizontal lines (frets)
- Muted indicators (×)
- Finger dots (circles)

**Why**: These are what users actually see on screen.

### 4. Size Variants

Tests that different size props work:

- Small (120x160)
- Medium (160x200)
- Large (200x250)

**Why**: Used in different contexts (progressions vs. standalone).

### 5. Real-World Examples

Tests with actual chord fingerings:

- C major (`x32010`)
- G major (`320003`)
- F barre (`133211`)

**Why**: Ensures common use cases work perfectly.

## What We DON'T Test

### Internal Implementation

```typescript
// ❌ DON'T test internal functions directly
describe('parseTab internals', () => {
	it('should use charToFret correctly', () => {
		// Bad: testing implementation detail
	});
});

// ✅ DO test user-visible behavior
describe('ChordDiagram rendering', () => {
	it('should render tab "x32010" correctly', () => {
		// Good: testing what user experiences
	});
});
```

### Complex Pure Functions

If a function is complex enough to need its own tests, it probably should be simplified or broken down. Our philosophy: **simplify first, test second**.

Example:

```typescript
// Instead of testing this complex function:
function calculateFingerNumbersWithAdvancedAlgorithm(positions, barres, hand) {
	// 50 lines of complex logic
}

// Simplify it:
function assignFingersByFretOrder(positions) {
	// 5 lines of clear logic
}
```

## Adding New Tests

When adding a new component, ask:

1. **What does the user do?** (Props they pass, interactions)
2. **What does the user see?** (Visual elements rendered)
3. **What can go wrong?** (Edge cases, errors)

Write tests for those, **not** for internal helper functions.

## Test Utilities

We use:

- **Vitest**: Fast, Vite-native test runner
- **@testing-library/svelte**: User-centric component testing
- **@testing-library/jest-dom**: Better assertions (`toBeInTheDocument`, etc.)
- **happy-dom**: Lightweight DOM environment

## Continuous Integration

Tests run automatically on:

- Every commit (pre-commit hook - future)
- Every pull request (CI pipeline - future)
- Before deployment (CD pipeline - future)

## Coverage Goals

We aim for **high user-path coverage**, not 100% line coverage.

**Good coverage example:**

- ✅ All prop variations tested
- ✅ All user input formats tested
- ✅ All error states tested
- ❌ Internal helper function not tested (and that's okay!)

**Why**: 80% coverage of user-facing behavior is more valuable than 100% coverage including irrelevant internals.

## Writing Good Tests

### ✅ DO

```typescript
it('should render C major chord (x32010)', () => {
	const { container } = render(ChordDiagram, {
		props: { tab: 'x32010', notes: ['C', 'E', 'G'], rootNote: 'C' },
	});
	expect(container.querySelector('svg')).toBeInTheDocument();
});
```

**Why**: Clear what user is trying to do (render C major).

### ❌ DON'T

```typescript
it('should call parseTab with correct arguments', () => {
	const spy = vi.spyOn(ChordDiagram, 'parseTab');
	render(ChordDiagram, { props: { tab: 'x32010' } });
	expect(spy).toHaveBeenCalledWith('x32010');
});
```

**Why**: Testing implementation detail (that `parseTab` is called). If we refactor to use a different parsing approach, this breaks unnecessarily.

## Testing Code Standards

Follow these principles to write robust, maintainable tests:

### 1. No Random Timeouts ⛔

**❌ DON'T use arbitrary timeouts:**

```typescript
it('should update value', async () => {
	await fireEvent.input(input, { target: { value: 'x32010' } });

	// Bad: arbitrary delay, may be too short or too long
	await new Promise((resolve) => setTimeout(resolve, 100));

	expect(boundValue).toBe('x32010');
});
```

**✅ DO use `vi.waitFor()` to wait for actual conditions:**

```typescript
it('should update value', async () => {
	await fireEvent.input(input, { target: { value: 'x32010' } });

	// Good: waits for actual state change, no arbitrary delay
	await vi.waitFor(() => {
		expect(boundValue).toBe('x32010');
	});
});
```

**Why:**

- Timeouts are flaky - too short = random failures, too long = slow tests
- `vi.waitFor()` polls until condition is met (with timeout as safety)
- Makes tests faster and more reliable
- Clear intent: "wait for this specific condition"

### 2. No TypeScript Casting ⛔

**❌ DON'T use `as` casts:**

```typescript
it('should render input', () => {
	const { container } = render(Form);

	// Bad: casting bypasses type safety
	const input = container.querySelector('input') as HTMLInputElement;
	expect(input.value).toBe('000000');
});
```

**✅ DO use proper typing with null checks:**

```typescript
it('should render input', () => {
	const { container } = render(Form);

	// Good: explicit typing with proper null handling
	const input: HTMLInputElement | null = container.querySelector('input');
	expect(input).toBeInTheDocument(); // Verify element exists
	expect(input!.value).toBe('000000'); // Use ! only after verification
});
```

**Why:**

- Type casts hide potential bugs (element might not exist)
- Explicit null checks make tests more robust
- Better error messages when element is missing
- Follows TypeScript best practices

### 3. Use Proper Query Selectors

**❌ DON'T rely on fragile selectors:**

```typescript
// Bad: breaks if CSS classes change
const button = container.querySelector('.btn-primary');

// Bad: brittle position-based selector
const button = container.querySelectorAll('button')[3];
```

**✅ DO use semantic selectors:**

```typescript
// Good: data-testid attribute (most reliable)
const input: HTMLInputElement | null = container.querySelector('[data-testid="tab-input"]');

// Good: semantic HTML attributes
const submitBtn: HTMLButtonElement | null = container.querySelector('button[type="submit"]');

// Good: ARIA attributes
const dialog: HTMLElement | null = container.querySelector('[role="dialog"]');

// Good: content matching for unique text
const buttons = container.querySelectorAll('button');
const clearBtn: HTMLButtonElement | undefined = Array.from(buttons).find(
	(btn) => btn.textContent === 'Clear'
);
```

### 4. Test Async State Changes Properly

**❌ DON'T assume immediate state updates:**

```typescript
it('should propagate value', async () => {
	await fireEvent.input(input, { target: { value: 'x32010' } });
	expect(boundValue).toBe('x32010'); // May fail - effects haven't run yet
});
```

**✅ DO wait for effects to complete:**

```typescript
it('should propagate value', async () => {
	await fireEvent.input(input, { target: { value: 'x32010' } });

	await vi.waitFor(() => {
		expect(boundValue).toBe('x32010');
	});
});
```

### 5. Test Wrappers for Complex Bindings

When testing components with `$bindable()` props, create a wrapper component:

**Test Wrapper Pattern:**

```svelte
<!-- FormTestWrapper.svelte -->
<script lang="ts">
	import Form from '$lib/components/Form.svelte';

	let { initialValue = '000000' }: { initialValue?: string } = $props();
	let value = $state(initialValue);

	// Expose state via data attribute for testing
	$effect(() => {
		if (typeof document !== 'undefined') {
			const wrapper = document.querySelector('[data-test-wrapper]');
			if (wrapper) {
				wrapper.setAttribute('data-bound-value', value);
			}
		}
	});
</script>

<div data-test-wrapper data-bound-value={value}>
	<Form bind:value />
</div>
```

**Usage in tests:**

```typescript
it('should update bound value', async () => {
	const { container } = render(FormTestWrapper, {
		props: { initialValue: '000000' },
	});

	const wrapper: HTMLElement | null = container.querySelector('[data-test-wrapper]');
	const input: HTMLInputElement | null = container.querySelector('input');

	await fireEvent.input(input!, { target: { value: 'x32010' } });

	await vi.waitFor(() => {
		expect(wrapper!.getAttribute('data-bound-value')).toBe('x32010');
	});
});
```

**Why:**

- Can't use `$state()` rune in `.ts` test files
- Wrapper component provides clean way to test two-way binding
- Exposes state via DOM attributes (testable)
- Keeps test code clean and maintainable

### 6. Clear Test Descriptions

**❌ DON'T write vague test names:**

```typescript
it('works correctly', () => {
	/* ... */
});
it('handles input', () => {
	/* ... */
});
```

**✅ DO write descriptive test names:**

```typescript
it('should allow typing partial input like "(" without propagating', () => {
	/* ... */
});
it('should propagate complete valid input like "(10)"', () => {
	/* ... */
});
it('should show error after blur with invalid input', () => {
	/* ... */
});
```

**Why:**

- Test name is documentation
- Easy to understand what broke when test fails
- Helps with debugging and maintenance

### Summary: Quick Checklist

Before committing tests, verify:

- [ ] ✅ No `setTimeout()` or arbitrary delays
- [ ] ✅ Use `vi.waitFor()` for async assertions
- [ ] ✅ No `as` type casts
- [ ] ✅ Proper TypeScript typing with null checks
- [ ] ✅ Semantic selectors (`data-testid`, ARIA roles)
- [ ] ✅ Clear, descriptive test names
- [ ] ✅ Test wrappers for complex component bindings

## Debugging Tests

```bash
# Run single test file
bun test ChordDiagram

# Run tests matching pattern
bun test "Tab Format"

# Run with UI (best for debugging)
bun run test:ui
```

## Future Improvements

- [ ] Add visual regression tests (screenshot comparison)
- [ ] Add accessibility tests (ARIA labels, keyboard navigation)
- [ ] Add performance tests (render time < 50ms)
- [ ] Add E2E tests with Playwright (full user workflows)

---

**Remember**: Tests are for confidence, not compliance. Write tests that give you confidence users will have a great experience.
