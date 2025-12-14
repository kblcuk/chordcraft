import path from 'path';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
import { SvelteKitPWA } from '@vite-pwa/sveltekit';

// https://vite.dev/config/
// Note: Bun has native WASM support, so vite-plugin-wasm and vite-plugin-top-level-await
// are no longer needed. If WASM loading fails, we can re-add them.
export default defineConfig({
	plugins: [
		sveltekit(),
		tailwindcss(),
		SvelteKitPWA({
			registerType: 'autoUpdate',
			strategies: 'generateSW',
			injectRegister: 'auto',
			manifest: false, // ./static/manifest.webmanifest
			workbox: {
				globPatterns: [
					'client/**/*.{js,css,ico,png,svg,avif,webp,webmanifest,wasm}',
					'prerendered/**/*.{html,json}',
				],
				// Cache external resources
				runtimeCaching: [
					{
						urlPattern: /^https:\/\/fonts\.googleapis\.com\/.*/i,
						handler: 'CacheFirst',
						options: {
							cacheName: 'google-fonts-cache',
							expiration: {
								maxEntries: 10,
								maxAgeSeconds: 60 * 60 * 24 * 365, // 1 year
							},
							cacheableResponse: {
								statuses: [0, 200],
							},
						},
					},
					{
						urlPattern: /^https:\/\/fonts\.gstatic\.com\/.*/i,
						handler: 'CacheFirst',
						options: {
							cacheName: 'gstatic-fonts-cache',
							expiration: {
								maxEntries: 10,
								maxAgeSeconds: 60 * 60 * 24 * 365,
							},
							cacheableResponse: {
								statuses: [0, 200],
							},
						},
					},
				],
			},
			devOptions: {
				enabled: true,
				suppressWarnings: process.env.SUPPRESS_WARNING === 'true',
				type: 'module',
				navigateFallback: '/',
			},
		}),
	],
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
