import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  // Keep the many small Pokémon sprites as files instead of embedding their
  // base64 payloads in the main JavaScript bundle.
  build: {
    assetsInlineLimit: 0,
  },
})
