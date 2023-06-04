import 'expect-more-jest';
import { join } from 'path';
import { getContext } from '.';
import { mockEffects } from '../../test/mock-effects';
import { getSource } from '../config/get-source';
import { CWD, DEFAULT_CONFIG } from '../constants';

describe('getContext', () => {
  describe('source', () => {
    it('uses defaults when no CLI options or config are set', () => {
      const effects = mockEffects();
      const ctx = getContext({}, effects);
      expect(getSource(ctx.config)).toEqual(DEFAULT_CONFIG.source);
    });

    it('uses value from config when no CLI options are set', () => {
      const effects = mockEffects();
      effects.readConfigFileSync.mockReturnValue({ source: ['foo'] });
      const ctx = getContext({}, effects);
      expect(getSource(ctx.config)).toEqual(['foo']);
    });

    it('uses value from CLI when config and CLI options are set', () => {
      const effects = mockEffects();
      effects.readConfigFileSync.mockReturnValue({ source: ['foo'] });
      const ctx = getContext({ source: ['bar'] }, effects);
      expect(getSource(ctx.config)).toEqual(['bar']);
    });
  });

  describe('packageJsonFiles', () => {
    describe('when --source cli options are given', () => {
      describe('for a single package.json file', () => {
        it('reads that file only', () => {
          const filePath = join(CWD, 'package.json');
          const contents = { name: 'foo' };
          const json = '{"name":"foo"}';
          const effects = mockEffects();
          effects.globSync.mockReturnValue([filePath]);
          effects.readFileSync.mockReturnValue(json);
          const config = getContext({ source: ['package.json'] }, effects);
          expect(config).toEqual(
            expect.objectContaining({
              packageJsonFiles: [
                {
                  contents,
                  effects: expect.toBeNonEmptyObject(),
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

      describe('for a pattern that matches nothing', () => {
        it('returns an empty array', () => {
          const effects = mockEffects();
          effects.globSync.mockReturnValue([]);
          expect(getContext({ source: ['typo.json'] }, effects)).toHaveProperty(
            'packageJsonFiles',
            [],
          );
          expect(effects.readFileSync).not.toHaveBeenCalled();
        });
      });
    });

    describe('when no --source cli options are given', () => {
      it('performs a default search', () => {
        const effects = mockEffects();
        getContext({}, effects);
        expect(effects.globSync.mock.calls).toEqual([
          ['package.json'],
          ['packages/*/package.json'],
        ]);
      });

      describe('when yarn workspaces are defined', () => {
        describe('as an array', () => {
          it('resolves yarn workspace packages', () => {
            const filePath = join(CWD, 'package.json');
            const contents = { workspaces: ['as-array/*'] };
            const json = JSON.stringify(contents);
            const effects = mockEffects();
            effects.globSync.mockReturnValue([filePath]);
            effects.readFileSync.mockReturnValue(json);
            getContext({}, effects);
            expect(effects.globSync.mock.calls).toEqual([
              ['package.json'],
              ['as-array/*/package.json'],
            ]);
          });
        });

        describe('as an object', () => {
          it('resolves yarn workspace packages', () => {
            const filePath = join(CWD, 'package.json');
            const contents = { workspaces: { packages: ['as-object/*'] } };
            const json = JSON.stringify(contents);
            const effects = mockEffects();
            effects.globSync.mockReturnValue([filePath]);
            effects.readFileSync.mockReturnValue(json);
            getContext({}, effects);
            expect(effects.globSync.mock.calls).toEqual([
              ['package.json'],
              ['as-object/*/package.json'],
            ]);
          });
        });
      });

      describe('when yarn workspaces are not defined', () => {
        describe('when lerna.json is defined', () => {
          it('resolves lerna packages', () => {
            const filePath = join(CWD, 'package.json');
            const contents = { name: 'foo' };
            const json = JSON.stringify(contents);
            const effects = mockEffects();
            effects.globSync.mockReturnValue([filePath]);
            effects.readFileSync.mockImplementation((filePath) => {
              if (filePath.endsWith('package.json')) return json;
              if (filePath.endsWith('lerna.json')) return JSON.stringify({ packages: ['lerna/*'] });
            });
            getContext({}, effects);
            expect(effects.globSync.mock.calls).toEqual([
              ['package.json'],
              ['lerna/*/package.json'],
            ]);
          });
        });

        describe('when lerna.json is not defined', () => {
          describe('when pnpm config is present', () => {
            describe('when pnpm workspaces are defined', () => {
              it('resolves pnpm packages', () => {
                const filePath = join(CWD, 'package.json');
                const effects = mockEffects();
                effects.globSync.mockReturnValue([filePath]);
                effects.readYamlFileSync.mockReturnValue({
                  packages: ['from-pnpm/*'],
                });
                getContext({}, effects);
                expect(effects.globSync.mock.calls).toEqual([
                  ['package.json'],
                  ['from-pnpm/*/package.json'],
                ]);
              });
            });

            describe('when pnpm config is invalid', () => {
              it('performs a default search', () => {
                const filePath = join(CWD, 'package.json');
                const effects = mockEffects();
                effects.globSync.mockReturnValue([filePath]);
                effects.readYamlFileSync.mockImplementation(() => {
                  throw new Error('Some YAML Error');
                });
                getContext({}, effects);
                expect(effects.globSync.mock.calls).toEqual([
                  ['package.json'],
                  ['packages/*/package.json'],
                ]);
              });
            });
          });
        });
      });
    });
  });
});
