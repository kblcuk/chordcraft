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
pnpm test

# Run once (CI/CD)
pnpm test:run

# With UI (visual test runner)
pnpm test:ui

# With coverage report
pnpm test:coverage
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

## Debugging Tests

```bash
# Run single test file
pnpm test ChordDiagram

# Run tests matching pattern
pnpm test "Tab Format"

# Run with UI (best for debugging)
pnpm test:ui
```

## Future Improvements

- [ ] Add visual regression tests (screenshot comparison)
- [ ] Add accessibility tests (ARIA labels, keyboard navigation)
- [ ] Add performance tests (render time < 50ms)
- [ ] Add E2E tests with Playwright (full user workflows)

---

**Remember**: Tests are for confidence, not compliance. Write tests that give you confidence users will have a great experience.
