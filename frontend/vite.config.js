import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
    root: './react',
    build: {
        outDir: './dist', // Output folder relative to the root
        emptyOutDir: true, // Ensure the output folder is cleared before build
    },
    plugins: [react()],
    server: {
        port: 3000,
    },
});
