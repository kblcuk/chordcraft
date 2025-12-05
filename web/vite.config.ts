import path from 'path';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

// https://vite.dev/config/
// Note: Bun has native WASM support, so vite-plugin-wasm and vite-plugin-top-level-await
// are no longer needed. If WASM loading fails, we can re-add them.
export default defineConfig({
	plugins: [sveltekit(), tailwindcss()],
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
