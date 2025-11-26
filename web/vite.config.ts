import path from 'path';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
import topLevelAwait from 'vite-plugin-top-level-await';
import wasm from 'vite-plugin-wasm';

// https://vite.dev/config/
export default defineConfig({
	plugins: [sveltekit(), wasm(), topLevelAwait(), tailwindcss()],
	resolve: {
		alias: {
			$lib: path.resolve('./src/lib'),
		},
	},
	server: {
		fs: {
			// Allow serving files from the workspace root (for WASM package)
			allow: ['..'],
		},
	},
});
