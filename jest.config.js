module.exports = {
  collectCoverage: true,
  collectCoverageFrom: ['src/**/*.ts', '!src/**/bin*.ts'],
  coverageReporters: ['html', 'lcov'],
  coverageThreshold: {
    global: {
      branches: 90,
      functions: 82,
      lines: 89,
      statements: 84,
    },
  },
  moduleFileExtensions: ['ts', 'tsx', 'js'],
  testMatch: ['<rootDir>/src/**/*.spec.(ts|tsx|js)'],
  transform: {
    '^.+\\.tsx?$': 'ts-jest',
  },
};
