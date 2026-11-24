import { defineConfig } from 'vitest/config'

export default defineConfig({
  server: {
    fs: { allow: ['.', '../docs/manual'] },
  },
  test: {
    environment: 'jsdom',
    setupFiles: ['./src/test-setup.ts'],
    globals: false,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json-summary', 'lcov'],
      include: ['src/**/*.{ts,tsx}'],
      exclude: [
        'src/samples/**',
        'src/cargo-book.ts',
        'src/comprehensive-rust.ts',
        'src/go-reference.ts',
        'src/rust-book.ts',
        'src/rust-crates.ts',
        'src/rust-reference.ts',
        'src/rust-std.ts',
        'src/rustc-error-codes.ts',
        'src/test-setup.ts',
        'src/**/*.test.{ts,tsx}',
      ],
      thresholds: {
        lines: 58,
        statements: 55,
        functions: 52,
        branches: 50,
      },
    },
  },
})
