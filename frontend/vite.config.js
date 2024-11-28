import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
    root: './react',
    build: {
      outDir: '../react/dist',
      emptyOutDir: true,
    },
    plugins: [react()],
    base: './', // Ensures relative paths with HashRouter
    server: {
      port: 3000,
    },
  });