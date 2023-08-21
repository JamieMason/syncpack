import 'expect-more-jest';
import { join } from 'path';
import { createMockEnv } from '../../test/lib/mock-env';
import { runContextSync } from '../../test/lib/run-context-sync';
import { CWD } from '../constants';
import { NoSourcesFoundError } from '../get-package-json-files/get-file-paths';

describe('getContext', () => {
  describe('packageJsonFiles', () => {
    describe('when --source cli options are given', () => {
      describe('for a single package.json file', () => {
        it('reads that file only', () => {
          const filePath = join(CWD, 'package.json');
          const contents = { name: 'foo' };
          const json = '{"name":"foo"}';
          const env = createMockEnv();
          env.globSync.mockReturnValue([filePath]);
          env.readFileSync.mockReturnValue(json);
          runContextSync({ source: ['package.json'] }, env, (ctx) => {
            expect(ctx).toEqual(
              expect.objectContaining({
                packageJsonFiles: [
                  {
                    contents,
                    dirPath: CWD,
                    filePath,
                    json,
                    config: expect.toBeNonEmptyObject(),
                    shortPath: 'package.json',
                  },
                ],
              }),
            );
          });
        });
      });

      describe('for a pattern that matches nothing', () => {
        it('returns a relevant error', () => {
          const env = createMockEnv();
          env.globSync.mockReturnValue([]);
          runContextSync({ source: ['typo.json'] }, env, (ctx) => {
            expect(ctx).toEqual(
              new NoSourcesFoundError({
                CWD,
                patterns: ['package.json', 'typo.json/package.json'],
              }),
            );
            expect(env.readFileSync).not.toHaveBeenCalled();
          });
        });
      });
    });

    describe('when no --source cli options are given', () => {
      it('performs a default search', () => {
        const env = createMockEnv();
        runContextSync({}, env, () => {
          expect(env.globSync).toHaveBeenCalledWith(['package.json', 'packages/*/package.json']);
          expect(env.globSync).toHaveBeenCalledTimes(1);
        });
      });

      describe('when yarn workspaces are defined', () => {
        describe('as an array', () => {
          it('resolves yarn workspace packages', () => {
            const filePath = join(CWD, 'package.json');
            const contents = { workspaces: ['as-array/*'] };
            const json = JSON.stringify(contents);
            const env = createMockEnv();
            env.globSync.mockReturnValue([filePath]);
            env.readFileSync.mockReturnValue(json);
            runContextSync({}, env, () => {
              expect(env.globSync).toHaveBeenCalledWith([
                'package.json',
                'as-array/*/package.json',
              ]);
              expect(env.globSync).toHaveBeenCalledTimes(1);
            });
          });
        });

        describe('as an object', () => {
          it('resolves yarn workspace packages', () => {
            const filePath = join(CWD, 'package.json');
            const contents = { workspaces: { packages: ['as-object/*'] } };
            const json = JSON.stringify(contents);
            const env = createMockEnv();
            env.globSync.mockReturnValue([filePath]);
            env.readFileSync.mockReturnValue(json);
            runContextSync({}, env, () => {
              expect(env.globSync).toHaveBeenCalledWith([
                'package.json',
                'as-object/*/package.json',
              ]);
              expect(env.globSync).toHaveBeenCalledTimes(1);
            });
          });
        });
      });

      describe('when yarn workspaces are not defined', () => {
        describe('when lerna.json is defined', () => {
          it('resolves lerna packages', () => {
            const filePath = join(CWD, 'package.json');
            const contents = { name: 'foo' };
            const json = JSON.stringify(contents);
            const env = createMockEnv();
            env.globSync.mockReturnValue([filePath]);
            env.readFileSync.mockImplementation((filePath) => {
              if (filePath.endsWith('package.json')) return json;
              if (filePath.endsWith('lerna.json')) return JSON.stringify({ packages: ['lerna/*'] });
            });
            runContextSync({}, env, () => {
              expect(env.globSync).toHaveBeenCalledWith(['package.json', 'lerna/*/package.json']);
              expect(env.globSync).toHaveBeenCalledTimes(1);
            });
          });
        });

        describe('when lerna.json is not defined', () => {
          describe('when pnpm config is present', () => {
            describe('when pnpm workspaces are defined', () => {
              it('resolves pnpm packages', () => {
                const filePath = join(CWD, 'package.json');
                const env = createMockEnv();
                env.globSync.mockReturnValue([filePath]);
                env.readYamlFileSync.mockReturnValue({
                  packages: ['from-pnpm/*'],
                });
                runContextSync({}, env, () => {
                  expect(env.globSync).toHaveBeenCalledWith([
                    'package.json',
                    'from-pnpm/*/package.json',
                  ]);
                  expect(env.globSync).toHaveBeenCalledTimes(1);
                });
              });
            });

            describe('when pnpm config is invalid', () => {
              it('performs a default search', () => {
                const filePath = join(CWD, 'package.json');
                const env = createMockEnv();
                env.globSync.mockReturnValue([filePath]);
                env.readYamlFileSync.mockImplementation(() => {
                  throw new Error('Some YAML Error');
                });
                runContextSync({}, env, () => {
                  expect(env.globSync).toHaveBeenCalledWith([
                    'package.json',
                    'packages/*/package.json',
                  ]);
                  expect(env.globSync).toHaveBeenCalledTimes(1);
                });
              });
            });
          });
        });
      });
    });
  });
});
