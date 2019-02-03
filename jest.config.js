module.exports = {
  collectCoverage: true,
  collectCoverageFrom: ['src/**/*.ts', '!src/**/typings.ts', '!src/**/bin*.ts'],
  coverageReporters: ['html', 'lcov'],
  coverageThreshold: {
    global: {
      branches: 95,
      functions: 95,
      lines: 95,
      statements: 95
    }
  },
  moduleFileExtensions: ['ts', 'tsx', 'js'],
  setupFilesAfterEnv: ['<rootDir>/test/setup.ts'],
  testMatch: ['<rootDir>/src/**/*.spec.(ts|tsx|js)'],
  transform: {
    '^.+\\.tsx?$': 'ts-jest'
  }
};
