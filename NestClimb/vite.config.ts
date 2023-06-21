import { VitePWA } from 'vite-plugin-pwa'
import type { UserConfig } from 'vite'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig(async () => {

	const config: UserConfig = {
		plugins: [
			VitePWA({
				registerType: 'autoUpdate',
				injectRegister: 'auto',
				includeAssets: ['favicon.ico', 'apple-touch-icon.png'],
				injectManifest: {
					globPatterns: ['**/*.{js,html,wasm,png}'],
				},
				manifest: {
					start_url: 'index.html?fullscreen=true',
					display: 'fullscreen',
					orientation: 'landscape',
					name: 'Nest Climb',
					short_name: 'NestClimb',
					description: 'Nest Climb',
					theme_color: '#000000',
					icons: [
						{
							src: 'pwa-192x192.png',
							sizes: '192x192',
							type: 'image/png',
						},
						{
							src: 'pwa-512x512.png',
							sizes: '512x512',
							type: 'image/png',
							purpose: 'any',
						},
					],
				},
				devOptions: {
					enabled: true,
				},
			}),
		],
		base: '/',
		build: {
			target: 'esnext',
			rollupOptions: {
				output: {
					entryFileNames: 'assets/[name].js',
				},
			},
		},


	}

	return config
})