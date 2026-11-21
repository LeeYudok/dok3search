import { defineConfig, loadEnv } from 'vite'

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, '.', '')
  const apiTarget = env.DOK3_API_URL || 'http://127.0.0.1:8080'
  return {
    build: {
      rolldownOptions: {
        output: {
          codeSplitting: {
            groups: [{
              name: 'sample-documents',
              test: /[\\/]src[\\/]samples[\\/]/,
              entriesAware: true,
              maxSize: 400 * 1024,
            }],
          },
        },
      },
    },
    server: {
      host: '127.0.0.1',
      proxy: {
        '/api': apiTarget,
        '/health': apiTarget,
      },
    },
  }
})
