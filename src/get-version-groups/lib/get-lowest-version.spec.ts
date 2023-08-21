import 'expect-more-jest';
import { Ok } from 'tightrope/result';
import { shuffle } from '../../../test/lib/shuffle';
import { getLowestVersion } from './get-lowest-version';

describe('getLowestVersion', () => {
  const a = ['*'];
  const b = shuffle([...a, '>1.0.0']);
  const c = shuffle([...b, '>=1.0.0']);
  const d = shuffle([...c, '^1.0.0']);
  const e = shuffle([...d, '1.x.x']);
  const f = shuffle([...e, '~1.0.0']);
  const g = shuffle([...f, '1.0.0']);
  const h = shuffle([...g, '1']);
  const i = shuffle([...h, '<=1.0.0']);
  const j = shuffle([...i, '<1.0.0']);
  const k = shuffle([...j, 'workspace:*']);

  // "1" and "1.0.0" are equal and first match wins
  const eitherFormat = expect.stringMatching(/^(1|1\.0\.0)$/);

  it('returns "*" when it is the only version', () => {
    expect(getLowestVersion(a)).toEqual(new Ok('*'));
  });

  it('returns ">1.0.0" when added', () => {
    expect(getLowestVersion(b)).toEqual(new Ok('>1.0.0'));
  });

  it('returns ">=1.0.0" when added', () => {
    expect(getLowestVersion(c)).toEqual(new Ok('>=1.0.0'));
  });

  it('returns "^1.0.0" when added', () => {
    expect(getLowestVersion(d)).toEqual(new Ok('^1.0.0'));
  });

  it('returns "1.x.x" when added', () => {
    expect(getLowestVersion(e)).toEqual(new Ok('1.x.x'));
  });

  it('returns "~1.0.0" when added', () => {
    expect(getLowestVersion(f)).toEqual(new Ok('~1.0.0'));
  });

  it('returns "1.0.0" when added', () => {
    expect(getLowestVersion(g)).toEqual(new Ok('1.0.0'));
  });

  it('returns "1" when added', () => {
    expect(getLowestVersion(h)).toEqual(new Ok(eitherFormat));
  });

  it('returns "<=1.0.0" when added', () => {
    expect(getLowestVersion(i)).toEqual(new Ok('<=1.0.0'));
  });

  it('returns "<1.0.0" when added', () => {
    expect(getLowestVersion(j)).toEqual(new Ok('<1.0.0'));
  });

  it('returns "workspace:*" when added', () => {
    expect(getLowestVersion(k)).toEqual(new Ok('workspace:*'));
  });
});
