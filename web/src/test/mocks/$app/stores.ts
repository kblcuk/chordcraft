import { readable } from 'svelte/store';

// Mock page store for tests
export const page = readable({
	url: new URL('http://localhost:5173'),
	params: {},
	route: { id: null },
	status: 200,
	error: null,
	data: {},
	state: {},
	form: undefined,
});
