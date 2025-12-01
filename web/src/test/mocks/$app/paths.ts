/**
 * Mock for $app/paths
 * Provides mock path resolution functions for tests
 */

export const base = '';
export const assets = '';

export function resolve(href: string): string {
	return href;
}
