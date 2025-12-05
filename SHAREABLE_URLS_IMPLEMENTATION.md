# Shareable URLs - Implementation Instructions

**Goal:** Make all three routes (Find, Name, Progression) fully shareable via URL query parameters with browser navigation support.

## Quick Summary

Add URL state synchronization to enable:
- Sharing links that preserve all user settings
- Browser back/forward button support
- Manual URL editing reactivity
- One-click "Share" buttons

**Note:** The project already uses `$app/state` (Svelte 5) instead of the deprecated `$app/stores`. All routes already import `page` from `$app/state` and use it in `onMount()`. We just need to add reactivity via `$effect()`.

## Implementation Checklist

### ✅ Step 1: Fix Name Route URL Sync
**File:** `web/src/lib/stores/name.ts`

**What to do:**
1. Update `initFromUrl()` method to parse `capo` and `startFret` from URL
2. Update `setCapo()` to call `debouncedUrlUpdate()`
3. Update `setStartFret()` to call `debouncedUrlUpdate()`
4. Update `debouncedUrlUpdate()` to include `capo` and `startFret` params
5. Ensure `tabInput` changes also trigger URL updates

**Code snippets:**

```typescript
// In initFromUrl()
initFromUrl(searchParams: URLSearchParams) {
  store.update((state) => ({
    ...state,
    tabInput: getParamValue(searchParams, 'tab', ''),
    capo: getParamValue(searchParams, 'capo', 0, Number),
    startFret: getParamValue(searchParams, 'startFret', 0, Number),
  }));
}

// In setCapo()
setCapo(capo: number) {
  store.update((state) => ({ ...state, capo }));
  debouncedUrlUpdate(); // ← Add this
}

// In setStartFret()
setStartFret(startFret: number) {
  store.update((state) => ({ ...state, startFret }));
  debouncedUrlUpdate(); // ← Add this
}

// Update debouncedUrlUpdate() function
const debouncedUrlUpdate = debounce(() => {
  const state = get(store);
  updateUrlParams({
    tab: state.tabInput || undefined,
    capo: state.capo > 0 ? state.capo.toString() : undefined,
    startFret: state.startFret > 0 ? state.startFret.toString() : undefined,
  });
}, 300);
```

**Also ensure `tabInput` changes trigger URL updates** - look for where `tabInput` is updated and add `debouncedUrlUpdate()` call if not already present.

---

### ✅ Step 2: Add URL Reactivity to Find Route
**File:** `web/src/routes/find/+page.svelte`

**What to do:**
The route already imports `page` from `$app/state` (Svelte 5). We just need to add `$effect()` to watch URL changes.

**Code to add:**

```typescript
// page is already imported at the top
// import { page } from '$app/state';

// Add this state variable (near other state declarations)
let previousUrl = $state(page.url.href);

// Add this effect (after onMount or near other effects)
// React to URL changes (browser navigation, manual edits)
$effect(() => {
  const currentUrl = page.url.href;

  // Only sync if URL actually changed (prevents state → URL → state loop)
  if (currentUrl !== previousUrl) {
    previousUrl = currentUrl;
    findStore.initFromUrl(page.url.searchParams);
  }
});
```

**Note:** Access `page.url.href` directly (not `$page.url.href`). The `page` object from `$app/state` is already reactive.

---

### ✅ Step 3: Add URL Reactivity to Name Route
**File:** `web/src/routes/name/+page.svelte`

**What to do:** Same as Step 2, but use `nameStore` instead of `findStore`

**Code to add:**

```typescript
// page is already imported
let previousUrl = $state(page.url.href);

$effect(() => {
  const currentUrl = page.url.href;

  if (currentUrl !== previousUrl) {
    previousUrl = currentUrl;
    nameStore.initFromUrl(page.url.searchParams);
  }
});
```

---

### ✅ Step 4: Add URL Reactivity to Progression Route
**File:** `web/src/routes/progression/+page.svelte`

**What to do:** Same as Step 2, but use `progressionStore` instead of `findStore`

**Code to add:**

```typescript
// page is already imported
let previousUrl = $state(page.url.href);

$effect(() => {
  const currentUrl = page.url.href;

  if (currentUrl !== previousUrl) {
    previousUrl = currentUrl;
    progressionStore.initFromUrl(page.url.searchParams);
  }
});
```

---

### ✅ Step 5: Add Share Button to Find Route
**File:** `web/src/routes/find/+page.svelte`

**What to do:** Add a share button near the search button

**Code to add:**

```svelte
<button
  class="rounded bg-gray-600 px-4 py-2 text-white hover:bg-gray-700"
  onclick={() => {
    navigator.clipboard.writeText(window.location.href)
      .then(() => {
        alert('Share link copied to clipboard!');
      })
      .catch((err) => {
        console.error('Failed to copy:', err);
        alert('Failed to copy link. Please copy from address bar.');
      });
  }}
>
  📋 Share
</button>
```

**Adjust Tailwind classes** to match existing button styles in the app.

---

### ✅ Step 6: Add Share Button to Name Route
**File:** `web/src/routes/name/+page.svelte`

**What to do:** Same as Step 5, add share button near analyze button

---

### ✅ Step 7: Add Share Button to Progression Route
**File:** `web/src/routes/progression/+page.svelte`

**What to do:** Same as Step 5, add share button near generate button

---

### ✅ Step 8 (Optional): Add Circular Update Guards
**Files:** `web/src/lib/stores/find.ts`, `web/src/lib/stores/name.ts`, `web/src/lib/stores/progression.ts`

**What to do:** Add safety flag to prevent URL updates while syncing from URL

**Code pattern:**

```typescript
// Add at top of store creator function
let isUpdatingFromUrl = false;

// Wrap initFromUrl() body
function initFromUrl(searchParams: URLSearchParams) {
  isUpdatingFromUrl = true;
  store.update((state) => ({
    // ... existing update logic
  }));
  isUpdatingFromUrl = false;
}

// Add check to debouncedUrlUpdate
const debouncedUrlUpdate = debounce(() => {
  if (isUpdatingFromUrl) return; // ← Add this guard

  const state = get(store);
  updateUrlParams({
    // ... existing params
  });
}, 300);
```

---

## Testing After Implementation

### Manual Testing (Do These First!)

1. **Find Route:**
   - Enter "Cmaj7", set capo to 3, voicing to "core"
   - Copy URL from address bar
   - Open in new tab → Should see same chord and settings
   - Click browser back/forward → UI should update

2. **Name Route:**
   - Enter "x32010", set capo to 2
   - Copy URL
   - Open in new tab → Should see same tab and capo
   - Click share button → URL should copy

3. **Progression Route:**
   - Enter "C Am F G", set max distance to 5
   - Copy URL
   - Open in new tab → Should auto-generate same progression

4. **Browser Navigation:**
   - Make changes on any route
   - Click browser back button → Should revert changes
   - Click forward button → Should restore changes

### Automated Tests

Create test files in `web/src/tests/stores/`:

**Test file structure:**
```typescript
// name.test.ts
import { describe, it, expect } from 'vitest';
import { nameStore } from '$lib/stores/name';

describe('nameStore URL sync', () => {
  it('should parse capo from URL params', () => {
    const params = new URLSearchParams('tab=x32010&capo=3');
    nameStore.initFromUrl(params);
    // Assert state has capo: 3
  });

  it('should parse startFret from URL params', () => {
    const params = new URLSearchParams('startFret=5');
    nameStore.initFromUrl(params);
    // Assert state has startFret: 5
  });

  it('should omit default values from URL', () => {
    // Set capo to 0, verify URL doesn't include capo param
  });
});
```

**Run tests:**
```bash
cd web
bun test
```

---

## Files Modified Summary

### Core Changes (Required):
1. `web/src/lib/stores/name.ts` - Add capo/startFret URL sync
2. `web/src/routes/find/+page.svelte` - Add URL reactivity
3. `web/src/routes/name/+page.svelte` - Add URL reactivity
4. `web/src/routes/progression/+page.svelte` - Add URL reactivity

### UI Enhancements (Required):
5. Add share buttons to all three route pages

### Safety Measures (Optional):
6. `web/src/lib/stores/find.ts` - Add circular update guard
7. `web/src/lib/stores/progression.ts` - Add circular update guard

### Tests (Required):
8. Create `web/src/tests/stores/name.test.ts`
9. Create `web/src/tests/stores/find.test.ts`
10. Create `web/src/tests/stores/progression.test.ts`

---

## Key Technical Details

### SvelteKit $app/state (Svelte 5)

**Important:** This project uses Svelte 5, which means we use `$app/state` instead of the deprecated `$app/stores`:

```typescript
// ✅ Correct (Svelte 5)
import { page } from '$app/state';
const url = page.url.href;  // Access directly, no $ prefix

// ❌ Deprecated (will be removed in SvelteKit 3)
import { page } from '$app/stores';
const url = $page.url.href;  // Uses $ prefix
```

The `page` object from `$app/state`:
- Is reactive using Svelte 5 runes (`$state.raw` internally)
- Has fine-grained reactivity (changes to `page.state` don't invalidate `page.data`)
- Works automatically in `$effect()` and other rune contexts
- Accessed without the `$` prefix (e.g., `page.url`, not `$page.url`)

### How URL Sync Works

**State → URL Flow:**
1. User types or changes options
2. Store state updates
3. Debounced function (300ms) calls `updateUrlParams()`
4. URL updates without page reload (using SvelteKit's `goto()`)

**URL → State Flow:**
1. URL changes (browser navigation, manual edit)
2. `$effect()` detects `page.url` change (automatically tracked)
3. Calls `store.initFromUrl(page.url.searchParams)`
4. Store state updates from URL params

**Circular Update Prevention:**
- `previousUrl` tracking in route components
- Optional `isUpdatingFromUrl` flag in stores
- Debouncing prevents rapid updates

### Default Value Handling

Only include non-default values in URL for cleaner links:
- `capo`: Only if > 0
- `limit`: Only if != default (10 for find, 3 for progression)
- `voicing`: Only if != 'all'
- `context`: Only if != 'solo'
- `position`: Always include if set
- `maxDistance`: Only if != 3

---

## Troubleshooting

**Problem:** Infinite loop of URL updates
- **Solution:** Ensure `previousUrl` check is in place
- **Solution:** Add `isUpdatingFromUrl` guard to stores

**Problem:** URL doesn't update when typing
- **Solution:** Ensure `debouncedUrlUpdate()` is called after state changes
- **Solution:** Check that debounce is imported and configured (300ms)

**Problem:** Browser back button doesn't work
- **Solution:** Ensure `$effect()` is watching `page.url` (it auto-tracks)
- **Solution:** Ensure `page` is imported from `$app/state` (not `$app/stores`)

**Problem:** Share button doesn't copy
- **Solution:** Check browser clipboard permissions
- **Solution:** Add error handling for clipboard API failures

**Problem:** `$page is not defined`
- **Solution:** Use `page.url` not `$page.url` with `$app/state`
- **Solution:** The `$` prefix is only for the deprecated `$app/stores`

---

## References

- [SvelteKit $app/state documentation](https://svelte.dev/docs/kit/$app-state)
- [SvelteKit $app/state tutorial](https://svelte.dev/tutorial/kit/page-state)
- [Migrating from $app/stores](https://svelte.dev/docs/kit/$app-stores)
- [Svelte 5 $effect documentation](https://svelte.dev/docs/svelte/$effect)
- [GitHub Issue: page.url reactivity](https://github.com/sveltejs/kit/issues/13187)

---

## Success Criteria

✅ All three routes have shareable URLs
✅ Browser back/forward buttons work
✅ Manual URL edits update the UI
✅ Name route includes capo and startFret
✅ Share buttons copy current URL
✅ No infinite loops
✅ No URL spam during typing (debouncing works)
✅ All tests pass

---

## Quick Start Command

```bash
cd web
# Make the code changes above
bun test           # Run tests
bun run dev        # Test manually
```

---

**Last updated:** 2025-11-28
**Status:** Ready to implement
**Estimated time:** 1-2 hours

**Sources:**
- [SvelteKit $app/stores documentation](https://svelte.dev/docs/kit/$app-stores)
- [SvelteKit $app/state documentation](https://svelte.dev/docs/kit/$app-state)
- [Svelte 5 $effect documentation](https://svelte.dev/docs/svelte/$effect)
- [SvelteKit $app/state tutorial](https://svelte.dev/tutorial/kit/page-state)
