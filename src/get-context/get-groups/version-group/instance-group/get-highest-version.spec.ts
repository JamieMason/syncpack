import { R } from '@mobily/ts-belt';
import 'expect-more-jest';
import { getHighestVersion } from './get-highest-version';

const shuffle = (array: string[]): string[] => {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
  return array;
};

describe('getHighestVersion', () => {
  it('returns the newest version from an array of versions', () => {
    const a = ['<1.0.0'];
    const b = shuffle([...a, '<=1.0.0']);
    const c = shuffle([...b, '1']);
    const d = shuffle([...c, '1.0.0']);
    const e = shuffle([...d, '~1.0.0']);
    const f = shuffle([...e, '1.x.x']);
    const g = shuffle([...f, '^1.0.0']);
    const h = shuffle([...g, '>=1.0.0']);
    const i = shuffle([...h, '>1.0.0']);
    const j = shuffle([...i, '*']);
    // valid semver
    expect(getHighestVersion(a)).toEqual(R.Ok('<1.0.0'));
    expect(getHighestVersion(b)).toEqual(R.Ok('<=1.0.0'));
    expect(getHighestVersion(c)).toEqual(R.Ok('1'));
    expect(getHighestVersion(d)).toEqual(
      // "1" and "1.0.0" are equal and first match wins
      R.Ok(expect.stringMatching(/^(1|1\.0\.0)$/)),
    );
    expect(getHighestVersion(e)).toEqual(R.Ok('~1.0.0'));
    expect(getHighestVersion(f)).toEqual(R.Ok('1.x.x'));
    expect(getHighestVersion(g)).toEqual(R.Ok('^1.0.0'));
    expect(getHighestVersion(h)).toEqual(R.Ok('>=1.0.0'));
    expect(getHighestVersion(i)).toEqual(R.Ok('>1.0.0'));
    expect(getHighestVersion(j)).toEqual(R.Ok('*'));
  });
});
