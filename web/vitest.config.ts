import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
	plugins: [svelte({ hot: !process.env.VITEST })],
	test: {
		globals: true,
		environment: 'happy-dom',
		setupFiles: ['./src/test/setup.ts'],
		include: ['src/**/*.{test,spec}.{js,ts}'],
		coverage: {
			provider: 'v8',
			reporter: ['text', 'html', 'json'],
			include: ['src/lib/**/*.{ts,svelte}'],
			exclude: ['src/**/*.test.ts', 'src/**/*.spec.ts'],
		},
	},
	resolve: {
		conditions: ['browser'],
	},
});
