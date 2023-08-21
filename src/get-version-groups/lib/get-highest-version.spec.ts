import 'expect-more-jest';
import { Ok } from 'tightrope/result';
import { shuffle } from '../../../test/lib/shuffle';
import { getHighestVersion } from './get-highest-version';

describe('getHighestVersion', () => {
  const a = ['workspace:*'];
  const b = shuffle([...a, '<1.0.0']);
  const c = shuffle([...b, '<=1.0.0']);
  const d = shuffle([...c, '1']);
  const e = shuffle([...d, '1.0.0']);
  const f = shuffle([...e, '~1.0.0']);
  const g = shuffle([...f, '1.x.x']);
  const h = shuffle([...g, '^1.0.0']);
  const i = shuffle([...h, '>=1.0.0']);
  const j = shuffle([...i, '>1.0.0']);
  const k = shuffle([...j, '*']);

  // "1" and "1.0.0" are equal and first match wins
  const eitherFormat = expect.stringMatching(/^(1|1\.0\.0)$/);

  it('returns "workspace:*" when it is the only version', () => {
    expect(getHighestVersion(a)).toEqual(new Ok('workspace:*'));
  });

  it('returns "<1.0.0" when added', () => {
    expect(getHighestVersion(b)).toEqual(new Ok('<1.0.0'));
  });

  it('returns "<=1.0.0" when added', () => {
    expect(getHighestVersion(c)).toEqual(new Ok('<=1.0.0'));
  });

  it('returns "1" when added', () => {
    expect(getHighestVersion(d)).toEqual(new Ok('1'));
  });

  it('returns "1.0.0" when added', () => {
    expect(getHighestVersion(e)).toEqual(new Ok(eitherFormat));
  });

  it('returns "~1.0.0" when added', () => {
    expect(getHighestVersion(f)).toEqual(new Ok('~1.0.0'));
  });

  it('returns "1.x.x" when added', () => {
    expect(getHighestVersion(g)).toEqual(new Ok('1.x.x'));
  });

  it('returns "^1.0.0" when added', () => {
    expect(getHighestVersion(h)).toEqual(new Ok('^1.0.0'));
  });

  it('returns ">=1.0.0" when added', () => {
    expect(getHighestVersion(i)).toEqual(new Ok('>=1.0.0'));
  });

  it('returns ">1.0.0" when added', () => {
    expect(getHighestVersion(j)).toEqual(new Ok('>1.0.0'));
  });

  it('returns "*" when added', () => {
    expect(getHighestVersion(k)).toEqual(new Ok('*'));
  });
});
