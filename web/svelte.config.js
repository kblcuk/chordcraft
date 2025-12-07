import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// adapter-static with pre-rendering
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: undefined, // Pre-render all routes instead of SPA fallback
			precompress: false,
			strict: true,
		}),

		alias: {
			$lib: './src/lib',
		},

		prerender: {
			entries: ['*'], // Pre-render all discovered routes
			handleHttpError: 'warn', // Warn instead of failing on HTTP errors
		},
	},
};

export default config;
