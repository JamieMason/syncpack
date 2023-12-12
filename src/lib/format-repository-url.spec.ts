import { formatRepositoryUrl } from './format-repository-url';

describe('formatRepositoryUrl', () => {
  test('formats url correctly', () => {
    const cases = [
      ['https://github.com/npm/json-parse-even-better-errors', 'https://github.com/npm/json-parse-even-better-errors'],
      ['https://github.com/Microsoft/TypeScript.git', 'https://github.com/Microsoft/TypeScript'],
      // if there is no provider domain supplied, we assume it's github
      ['chalk/ansi-regex', 'https://github.com/chalk/ansi-regex'],
      ['git://github.com/juliangruber/balanced-match.git', 'https://github.com/juliangruber/balanced-match'],
      ['git@github.com:colorjs/color-name.git', 'https://github.com/colorjs/color-name'],
      ['git+https://github.com/dubzzz/pure-rand.git', 'https://github.com/dubzzz/pure-rand'],
    ] as const;

    cases.forEach(([string, expected]) => expect(formatRepositoryUrl(string)).toEqual(expected));
  });
});
