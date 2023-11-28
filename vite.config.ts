import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [react()],
    clearScreen: false,
    server: {
        port: 1919,
        strictPort: true,
    },
    resolve: {
        alias: {
            '@': resolve(__dirname, 'src'),
            '@mui/styled-engine': resolve(
                __dirname,
                'node_modules',
                '@mui',
                'styled-engine-sc'
            ),
        },
    },
}))
