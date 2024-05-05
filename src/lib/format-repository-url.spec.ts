import { describe, expect, test } from 'vitest';
import { formatRepositoryUrl } from './format-repository-url.js';

describe('formatRepositoryUrl', () => {
  test.each([
    {
      actual: 'https://github.com/npm/json-parse-even-better-errors',
      expected: 'https://github.com/npm/json-parse-even-better-errors',
    },
    {
      actual: 'https://github.com/Microsoft/TypeScript.git',
      expected: 'https://github.com/Microsoft/TypeScript',
    },
    // if there is no provider domain supplied, we assume it's github
    {
      actual: 'chalk/ansi-regex',
      expected: 'https://github.com/chalk/ansi-regex',
    },
    {
      actual: 'git://github.com/juliangruber/balanced-match.git',
      expected: 'https://github.com/juliangruber/balanced-match',
    },
    {
      actual: 'git@github.com:colorjs/color-name.git',
      expected: 'https://github.com/colorjs/color-name',
    },
    {
      actual: 'git+https://github.com/dubzzz/pure-rand.git',
      expected: 'https://github.com/dubzzz/pure-rand',
    },
  ])('formats "$actual" as "$expected" url correctly', ({ actual, expected }) => {
    expect(formatRepositoryUrl(actual)).toEqual(expected);
  });
});
