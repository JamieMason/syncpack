module.exports = {
  collectCoverage: true,
  collectCoverageFrom: ['src/**/*.ts', '!src/**/bin*/index.ts'],
  coverageReporters: ['html', 'lcov'],
  coverageThreshold: {
    global: {
      branches: 69,
      functions: 84,
      lines: 81,
      statements: 80,
    },
  },
  moduleFileExtensions: ['ts', 'js'],
  testMatch: ['<rootDir>/src/**/*.spec.(ts|js)'],
  transform: {
    '^.+\\.ts$': 'ts-jest',
  },
};
