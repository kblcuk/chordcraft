/**
 * Mock for $app/state
 * Provides a mock page state for tests
 */

export const page = {
	url: new URL('http://localhost:5173'),
	params: {},
	route: { id: null },
	status: 200,
	error: null,
	data: {},
	form: undefined,
};
