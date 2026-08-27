import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { cpSync } from 'node:fs';

export default defineConfig({
  plugins: [svelte(), { name: 'ship-extension', closeBundle: () => cpSync('extension', 'dist/extension', { recursive: true }) }],
  build: { target: 'es2022', cssCodeSplit: false },
  server: { proxy: { '/api': 'http://localhost:8080', '/health': 'http://localhost:8080' } },
  test: { include: ['src/**/*.test.ts'] }
});
