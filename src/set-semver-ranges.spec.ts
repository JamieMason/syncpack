import { readJsonSync } from 'fs-extra';
import mock = require('mock-fs');
import { getFixture, getMockCommander } from '../test/helpers';
import {
  RANGE_ANY,
  RANGE_EXACT,
  RANGE_GT,
  RANGE_GTE,
  RANGE_LOOSE,
  RANGE_LT,
  RANGE_LTE,
  RANGE_MINOR,
  RANGE_PATCH
} from './constants';
import { run } from './set-semver-ranges';

describe('set-semver-ranges', () => {
  const ranges = [
    [RANGE_ANY, 'any'],
    [RANGE_EXACT, 'exact'],
    [RANGE_GT, 'gt'],
    [RANGE_GTE, 'gte'],
    [RANGE_LOOSE, 'loose'],
    [RANGE_LT, 'lt'],
    [RANGE_LTE, 'lte'],
    [RANGE_MINOR, 'minor'],
    [RANGE_PATCH, 'patch']
  ].map(([range, name]) => ({
    data: getFixture(name).data[0],
    filePath: `/path/${name}/package.json`,
    range
  }));
  const unsupported = [RANGE_ANY, RANGE_LOOSE];
  const sources = ranges.map(({ filePath }) => filePath);
  const filesystem = ranges.reduce(
    (obj, { filePath, data }) => ({ ...obj, [filePath]: JSON.stringify(data) }),
    {}
  );

  ranges.forEach(({ data: expectedData, range: targetRange }) => {
    ranges
      .filter(({ range: sourceRange }) => sourceRange !== targetRange)
      .filter(({ range: sourceRange }) => !unsupported.includes(sourceRange))
      .forEach(({ filePath, range: sourceRange }) => {
        it(`sets "${sourceRange}" semver ranges to the "${targetRange}" format`, async () => {
          const program = getMockCommander(sources, '^((?!ignore).)*$');
          program.semverRange = targetRange;
          mock(filesystem);
          const noop = () => undefined;
          const spyConsole = jest
            .spyOn(console, 'log')
            .mockImplementation(noop);
          await run(program);
          expect(readJsonSync(filePath)).toEqual(expectedData);
          mock.restore();
        });
      });
  });
});
