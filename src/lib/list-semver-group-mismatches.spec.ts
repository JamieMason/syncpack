import 'expect-more-jest';

// @TODO
const listSemverGroupMismatches: any = () => null;

describe.skip('listSemverGroupMismatches', () => {
  describe('when semver range should be "~"', () => {
    it('returns dependencies with invalid semver ranges', () => {
      expect(
        listSemverGroupMismatches({
          range: '~',
          instances: [
            { name: 'foo', version: '~0.1.4' },
            { name: 'bar', version: '2.2.6' },
            { name: 'baz', version: '^1.0.0' },
          ],
        } as any),
      ).toEqual([
        { name: 'bar', version: '2.2.6' },
        { name: 'baz', version: '^1.0.0' },
      ]);
    });
  });
  describe('when semver range should be "*"', () => {
    it('returns dependencies with invalid semver ranges', () => {
      expect(
        listSemverGroupMismatches({
          range: '*',
          instances: [
            { name: 'foo', version: '~0.1.4' },
            { name: 'bar', version: '2.2.6' },
            { name: 'baz', version: '^1.0.0' },
          ],
        } as any),
      ).toEqual([
        { name: 'bar', version: '2.2.6' },
        { name: 'baz', version: '^1.0.0' },
        { name: 'foo', version: '~0.1.4' },
      ]);
    });
  });
  describe('when semver range should be ""', () => {
    it('returns dependencies with invalid semver ranges', () => {
      expect(
        listSemverGroupMismatches({
          range: '',
          instances: [
            { name: 'foo', version: '~0.1.4' },
            { name: 'bar', version: '2.2.6' },
            { name: 'baz', version: '^1.0.0' },
          ],
        } as any),
      ).toEqual([
        { name: 'baz', version: '^1.0.0' },
        { name: 'foo', version: '~0.1.4' },
      ]);
    });
  });
});
