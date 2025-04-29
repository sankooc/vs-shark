import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

// https://vite.dev/config/
export default defineConfig(({ command, mode }) => {
  // 加载环境变量
  const env = loadEnv(mode, process.cwd())
  
  // 所有入口配置
  const allEntries = {
    main: resolve(__dirname, 'index.html'),
    admin: resolve(__dirname, 'admin.html'),
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
    plugins: [react()],
    build: {
      rollupOptions: {
        input: getEntries(),
        output: {
          entryFileNames: 'js/[name].js',
          chunkFileNames: 'js/[name]-chunk.js',
          assetFileNames: 'assets/[name][extname]',
        },
      },
    },
  }
})
