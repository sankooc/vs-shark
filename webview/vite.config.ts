import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig(({ command, mode }) => {
  const env = loadEnv(mode, process.cwd())
  
  const allEntries = {
    main: resolve(__dirname, 'index.html'),
    app: resolve(__dirname, 'app.html'),
  }

  const mainEntry = {
    main: resolve(__dirname, 'index.html'),
  }

  const getEntries = () => {
    if (command === 'serve') return allEntries
    if (env.VITE_BUILD_ALL === 'true') return allEntries
    return mainEntry
  }

  return {
    plugins: [
      react(),
      wasm(),
      topLevelAwait()
    ],
    resolve: {
      alias: {
        '@': resolve(__dirname, 'src'),
        '@assets': resolve(__dirname, 'src/assets')
      },
    },
    optimizeDeps: {
      exclude: ['rshark']
    },
    base: '',
    assetsInclude: ['**/*.ttf'],
    build: {
      outDir: './../plugin/dist/web',
      emptyOutDir: true,
      rollupOptions: {
        input: getEntries(),
        output: {
          entryFileNames: 'js/[name].js',
          chunkFileNames: 'js/[name]-chunk.js',
          assetFileNames: (assetInfo) => {
            const info = assetInfo.name ?? '';
            if (info.endsWith('.ttf')) {
              return 'assets/font/[name][extname]';
            }
            return 'assets/[name][extname]';
          }
        },
      },
      target: 'esnext',
      assetsInlineLimit: 0,
    },
  }
})
