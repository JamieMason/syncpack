import { Installation } from './get-dependencies';
import { versionsMatch } from './versions-match';

const createInstallations = (name: string, ...versions: [string, string]) => ({
  a: {
    name,
    version: versions[0],
  } as Installation,
  b: {
    name,
    version: versions[1],
  } as Installation,
});

const expectVersionsMatchToReturn = (
  a: Installation,
  b: Installation,
  matchRanges: boolean,
  expectedResult: boolean,
) => {
  expect(versionsMatch(a, b, true)).toBe(expectedResult);
  expect(versionsMatch(b, a, true)).toBe(expectedResult);
};

describe('versionsMatch', () => {
  describe('matchRanges = true', () => {
    const cases: Array<[a: string, b: string, expectedResult: boolean]> = [
      ['1.2.3', '1.2.3', true],
      ['1.2.3', '1.0.0', false],
      ['1.2.3', '^1.2.3', true],
      ['1.2.3', '^1.0.0', true],
      ['1.2.3', '^2.0.0', false],
      ['^1.2.3', '^1.2.3', true],
      ['^1.2.3', '^1.4.5', true],
      ['^1.2.3', '^2.0.0', false],
      ['~1.2.3', '1.2.3', true],
      ['~1.2.3', '1.2.4', true],
      ['~1.2.3', '1.3.4', false],
      ['~1.2.3', '1.0.0', false],
      ['~1.2.3', '~1.2.3', true],
      ['~1.2.3', '~1.3.4', false],
      ['*', '1.3.4', true],
      ['*', '^1.3.4', true],
      ['*', '~1.3.4', true],

      ['npm:1.2.3', 'npm:1.2.3', true],
      ['npm:1.2.3', 'npm:1.0.0', false],
      ['npm:1.2.3', 'github:1.2.3', false],

      ['https://github.com/npm/npm.git', 'https://github.com/npm/npm.git', true],
      ['https://github.com/npm/npm.git', 'https://github.com/npm/npm-two.git', false],
    ];

    it.each(cases)('%s =~ %s == %s', (aVersion, bVersion, expectedResult) => {
      const { a, b } = createInstallations('fs', aVersion, bVersion);

      expectVersionsMatchToReturn(a, b, true, expectedResult);
    });
  });
});
