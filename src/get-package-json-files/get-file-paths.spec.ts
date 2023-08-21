import { pipe } from '@effect/data/Function';
import * as Effect from '@effect/io/Effect';
import 'expect-more-jest';
import type { MockEnv } from '../../test/lib/mock-env';
import { createMockEnv } from '../../test/lib/mock-env';
import { CWD } from '../constants';
import { createEnv } from '../env/create-env';
import { EnvTag } from '../env/tags';
import type { Ctx } from '../get-context';
import { getFilePaths, NoSourcesFoundError } from './get-file-paths';

function runSync(config: Ctx['config'], mockedEffects: MockEnv, onValue: (value: any) => void) {
  Effect.runSync(
    pipe(
      getFilePaths(config),
      Effect.match({
        onFailure: onValue,
        onSuccess: onValue,
      }),
      Effect.provideService(EnvTag, createEnv(mockedEffects)),
    ),
  );
}

it('return error when patterns return no files', () => {
  const env = createMockEnv();
  env.globSync.mockReturnValue([]);
  runSync({ cli: {}, rcFile: {} }, env, (result) => {
    expect(result).toEqual(
      new NoSourcesFoundError({
        CWD,
        patterns: ['package.json', 'packages/*/package.json'],
      }),
    );
  });
});

it('returns strings when patterns return files', () => {
  const env = createMockEnv();
  const root = ['/fake/dir/package.json'];
  const packages = ['/fake/dir/packages/a/package.json', '/fake/dir/packages/b/package.json'];
  env.globSync.mockImplementation(() => {
    return [...root, ...packages];
  });
  runSync({ cli: {}, rcFile: {} }, env, (result) => {
    expect(result).toEqual([...root, ...packages]);
  });
});
