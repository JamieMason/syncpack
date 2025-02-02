import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    coverage: {
      all: true,
      enabled: true,
      include: ['src/**/*.ts'],
      exclude: ['src/bin.ts', 'src/bin-*/index.ts', 'site', 'test'],
      extension: ['.ts'],
      provider: 'v8',
      reporter: ['lcov', 'html', 'text'],
      thresholds: {
        statements: 80,
        branches: 75,
        lines: 80,
        functions: 75,
      },
    },
    include: ['src/**/*.spec.ts'],
    onConsoleLog: () => false,
    setupFiles: ['test/setup-file.ts'],
  },
});
