import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { svelteSVG } from "rollup-plugin-svelte-svg";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
	svelte(),
	svelteSVG({
		// optional SVGO options
		// pass empty object to enable defaults
		svgo: {},
		// vite-specific
		// https://vitejs.dev/guide/api-plugin.html#plugin-ordering
		// enforce: 'pre' | 'post'
		enforce: "pre",
	}),
]
})
