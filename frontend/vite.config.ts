import { defineConfig } from 'vite'

export default defineConfig({
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
})
