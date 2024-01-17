import { Effect } from 'effect';
import { describe, expect, it } from 'vitest';
import { shuffle } from '../../../test/lib/shuffle.js';
import type { Specifier } from '../../specifier/index.js';
import { getLowestVersion } from './get-lowest-version.js';

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

  function toSpecifier(version: string): Specifier.Any {
    return {
      getSemver: () => Effect.succeed(version),
      raw: version,
    } as Specifier.Any;
  }

  // "1" and "1.0.0" are equal and first match wins
  const eitherFormat = expect.stringMatching(/^(1|1\.0\.0)$/);

  it('returns "*" when it is the only version', async () => {
    const specifiers = a.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '*',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns ">1.0.0" when added', async () => {
    const specifiers = b.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '>1.0.0',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns ">=1.0.0" when added', async () => {
    const specifiers = c.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '>=1.0.0',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns "^1.0.0" when added', async () => {
    const specifiers = d.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '^1.0.0',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns "1.x.x" when added', async () => {
    const specifiers = e.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '1.x.x',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns "~1.0.0" when added', async () => {
    const specifiers = f.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '~1.0.0',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns "1.0.0" when added', async () => {
    const specifiers = g.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '1.0.0',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns "1" when added', async () => {
    const specifiers = h.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: eitherFormat,
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns "<=1.0.0" when added', async () => {
    const specifiers = i.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '<=1.0.0',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns "<1.0.0" when added', async () => {
    const specifiers = j.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '<1.0.0',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });

  it('returns "workspace:*" when added', async () => {
    const specifiers = k.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: 'workspace:*',
    });
    expect(await Effect.runPromise(getLowestVersion(specifiers))).toEqual(expected);
  });
});
