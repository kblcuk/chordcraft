/**
 * Utilities for syncing state with URL query parameters
 */

import { goto } from '$app/navigation';
import { browser } from '$app/environment';
import { page } from '$app/state';

export const routes = [
	{ path: '/find', label: 'Find Fingerings' },
	{ path: '/name', label: 'Name Chord' },
	{ path: '/progression', label: 'Progression' },
] as const;

/**
 * Parse a URL search params object into a plain object
 */
export function parseSearchParams(searchParams: URLSearchParams): Record<string, string> {
	const params: Record<string, string> = {};
	searchParams.forEach((value, key) => {
		params[key] = value;
	});
	return params;
}

/**
 * Update URL query params without triggering navigation
 * Uses replaceState to avoid adding to browser history
 */
export function updateUrlParams(
	params: Record<string, string | number | boolean | null | undefined>,
	options: { replaceState?: boolean; keepFocus?: boolean } = {}
): void {
	if (!browser) return;

	const searchParams = new URLSearchParams();

	// Add all non-null, non-undefined params
	Object.entries(params).forEach(([key, value]) => {
		if (value !== null && value !== undefined && value !== '') {
			searchParams.set(key, String(value));
		}
	});

	const query = searchParams.toString();
	const pathname = page.url.pathname;
	const url = query ? `${pathname}?${query}` : pathname;

	// We can enalbe this back when goto supports query params:
	// https://github.com/sveltejs/kit/issues/14750
	// eslint-disable-next-line svelte/no-navigation-without-resolve
	goto(url, {
		replaceState: options.replaceState ?? true,
		keepFocus: options.keepFocus ?? true,
		noScroll: true,
	}).catch((e) => {
		console.error('Navigation error', e);
		// Ignore navigation errors (user might navigate away before this completes)
	});
}

/**
 * Get a typed value from URL params with a default fallback
 */
export function getParamValue<T>(
	params: URLSearchParams,
	key: string,
	defaultValue: T,
	parser?: (value: string) => T
): T {
	const value = params.get(key);
	if (value === null) return defaultValue;

	if (parser) {
		try {
			return parser(value);
		} catch {
			return defaultValue;
		}
	}

	// Auto-detect type based on defaultValue
	if (typeof defaultValue === 'number') {
		const parsed = Number(value);
		return (isNaN(parsed) ? defaultValue : parsed) as T;
	}

	if (typeof defaultValue === 'boolean') {
		return (value === 'true') as T;
	}

	return value as T;
}

/**
 * Debounce function for URL updates
 */
export function debounce<T extends (...args: unknown[]) => unknown>(
	fn: T,
	delay: number
): (...args: Parameters<T>) => void {
	let timeoutId: ReturnType<typeof setTimeout> | undefined;

	return (...args: Parameters<T>) => {
		if (timeoutId) clearTimeout(timeoutId);
		timeoutId = setTimeout(() => fn(...args), delay);
	};
}
