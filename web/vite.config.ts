import path from 'path';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
import topLevelAwait from 'vite-plugin-top-level-await';
import wasm from 'vite-plugin-wasm';

// https://vite.dev/config/
export default defineConfig({
	plugins: [svelte(), wasm(), topLevelAwait(), tailwindcss()],
	resolve: {
		alias: {
			$lib: path.resolve('./src/lib'),
		},
	},
});
