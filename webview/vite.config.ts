import { defineConfig, loadEnv, type ConfigEnv } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import fs from 'fs';
import path from 'path';

const getPackageVersion = (): string => {
  const packageJsonPath = path.resolve(process.cwd(), 'package.json');
  try {
    if (fs.existsSync(packageJsonPath)) {
      const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf-8'));
      return JSON.stringify(packageJson.version);
    }
  } catch { }
  return '0.0.0';
}

export default defineConfig((props: ConfigEnv) => {
  const { mode } = props;
  const env = loadEnv(mode, process.cwd())

  const allEntries = {
    main: resolve(__dirname, 'index.html'),
    app: resolve(__dirname, 'app.html'),
  }

  const mainEntry = {
    main: resolve(__dirname, 'app.html'),
  }

  const getEntry = (): Record<string, string> => {
    if (env.VITE_BUILD_ALL === 'true') return allEntries
    if (env.VITE_BUILD_SOCKET === 'true') {
      return {
        index: resolve(__dirname, 'ui.html'),
      };
    }
    if (env.VITE_BUILD_GUI === 'true') {
      return {
        index: resolve(__dirname, 'gui.html'),
        gate: resolve(__dirname, 'loading.html'),
      };
    }
    if (env.VITE_BUILD_VSCODE === 'true') {
      return mainEntry;
    }
    return allEntries;
  }

  const getOutput = (): string => {
    if (env.VITE_BUILD_ALL === 'true') return '../dist/web'
    if (env.VITE_BUILD_VSCODE === 'true') {
      return './../plugin/dist/web';
    }
    if (env.VITE_BUILD_GUI === 'true') {
      return './../dist/gui';
    }
    if (env.VITE_BUILD_SOCKET == 'true') {
      return './../dist/socket';
    }
    return './dist';
  }
  
  return {
    define: {
      'import.meta.env.VITE_APP_VERSION': getPackageVersion(),
    },
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
    server: {
      proxy: {
        '/api': {
          target: 'http://127.0.0.1:3000',
          changeOrigin: true,
        },
      }
    },
    optimizeDeps: {
      exclude: ['rshark']
    },
    base: '',
    assetsInclude: ['**/*.ttf'],
    build: {
      outDir: getOutput(),
      emptyOutDir: true,
      rollupOptions: {
        input: getEntry(),
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
