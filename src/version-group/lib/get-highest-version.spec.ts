import { Effect } from 'effect';
import { describe, expect, it } from 'vitest';
import { shuffle } from '../../../test/lib/shuffle.js';
import type { Specifier } from '../../specifier/index.js';
import { getHighestVersion } from './get-highest-version.js';

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

  function toSpecifier(version: string): Specifier.Any {
    return {
      getSemver: () => Effect.succeed(version),
      raw: version,
    } as Specifier.Any;
  }

  // "1" and "1.0.0" are equal and first match wins
  const eitherFormat = expect.stringMatching(/^(1|1\.0\.0)$/);

  it('returns "workspace:*" when it is the only version', async () => {
    const specifiers = a.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: 'workspace:*',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns "<1.0.0" when added', async () => {
    const specifiers = b.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '<1.0.0',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns "<=1.0.0" when added', async () => {
    const specifiers = c.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '<=1.0.0',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns "1" when added', async () => {
    const specifiers = d.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '1',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns "1.0.0" when added', async () => {
    const specifiers = e.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: eitherFormat,
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns "~1.0.0" when added', async () => {
    const specifiers = f.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '~1.0.0',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns "1.x.x" when added', async () => {
    const specifiers = g.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '1.x.x',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns "^1.0.0" when added', async () => {
    const specifiers = h.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '^1.0.0',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns ">=1.0.0" when added', async () => {
    const specifiers = i.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '>=1.0.0',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns ">1.0.0" when added', async () => {
    const specifiers = j.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '>1.0.0',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });

  it('returns "*" when added', async () => {
    const specifiers = k.map(toSpecifier);
    const expected = expect.objectContaining({
      raw: '*',
    });
    expect(await Effect.runPromise(getHighestVersion(specifiers))).toEqual(expected);
  });
});
