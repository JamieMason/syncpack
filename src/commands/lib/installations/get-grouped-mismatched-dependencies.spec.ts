import 'expect-more-jest';
import { DEFAULT_CONFIG, SyncpackConfig } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { getMismatchedDependencies } from './get-mismatched-dependencies';

const expectedMismatch = (nameAndVersion: string, packageName: string, dependencyType: string) => {
  const [dependencyName, dependencyVersion] = nameAndVersion.split('@');
  return expect.objectContaining({
    name: dependencyName,
    source: {
      filePath: '',
      contents: expect.objectContaining({
        name: packageName,
        [dependencyType]: expect.objectContaining({
          [dependencyName]: dependencyVersion,
        }),
      }),
    },
    type: dependencyType,
    version: dependencyVersion,
  });
};

const mockWrapper = (contents: SourceWrapper['contents']): SourceWrapper => ({ filePath: '', contents });

describe('getMismatchedDependencies', () => {
  describe('when versions match inside a group, but differ to those outside the group', () => {
    it('returns no mismatches because the grouped and non-grouped packages each match internally', () => {
      const config: SyncpackConfig = {
        ...DEFAULT_CONFIG,
        versionGroups: [{ dependencies: ['core'], packages: ['@next/a', '@next/b'] }],
      };
      const iterator = getMismatchedDependencies(
        [
          mockWrapper({ name: '@next/a', dependencies: { core: '0.0.1' } }),
          mockWrapper({ name: '@next/b', dependencies: { core: '0.0.1' } }),
          mockWrapper({ name: '@legacy/a', dependencies: { core: '0.0.0' } }),
          mockWrapper({ name: '@legacy/b', dependencies: { core: '0.0.0' } }),
          mockWrapper({ name: 'ungrouped', dependencies: { outside: '1.0.0' } }),
        ],
        config,
      );
      expect(Array.from(iterator)).toBeEmptyArray();
    });
  });

  describe('when versions differ inside a group, but match outside the group', () => {
    describe('when the mismatched dependency is named in the group', () => {
      it('returns mismatches for inside the group only', () => {
        const config: SyncpackConfig = {
          ...DEFAULT_CONFIG,
          versionGroups: [{ dependencies: ['core'], packages: ['@next/a', '@next/b'] }],
        };
        const iterator = getMismatchedDependencies(
          [
            mockWrapper({ name: '@next/a', dependencies: { core: '0.0.0' } }),
            mockWrapper({ name: '@next/b', dependencies: { core: '0.0.1' } }),
            mockWrapper({ name: '@legacy/a', dependencies: { core: '0.0.0' } }),
            mockWrapper({ name: '@legacy/b', dependencies: { core: '0.0.0' } }),
            mockWrapper({ name: 'ungrouped', dependencies: { outside: '1.0.0' } }),
          ],
          config,
        );
        expect(Array.from(iterator)).toEqual([
          {
            name: 'core',
            installations: [
              expectedMismatch('core@0.0.0', '@next/a', 'dependencies'),
              expectedMismatch('core@0.0.1', '@next/b', 'dependencies'),
            ],
          },
        ]);
      });
    });

    describe('when the mismatched dependency is not named in the group', () => {
      it('returns mismatches everywhere they appear', () => {
        const config: SyncpackConfig = {
          ...DEFAULT_CONFIG,
          versionGroups: [{ dependencies: ['core'], packages: ['@next/a', '@next/b'] }],
        };
        const iterator = getMismatchedDependencies(
          [
            mockWrapper({ name: '@next/a', dependencies: { core: '0.0.1', outside: '1.0.0' } }),
            mockWrapper({ name: '@next/b', dependencies: { core: '0.0.1', outside: '1.0.1' } }),
            mockWrapper({ name: '@legacy/a', dependencies: { core: '0.0.0' } }),
            mockWrapper({ name: '@legacy/b', dependencies: { core: '0.0.0' } }),
            mockWrapper({ name: 'ungrouped', dependencies: { outside: '1.0.0' } }),
          ],
          config,
        );
        expect(Array.from(iterator)).toEqual([
          {
            name: 'outside',
            installations: [
              expectedMismatch('outside@1.0.0', '@next/a', 'dependencies'),
              expectedMismatch('outside@1.0.1', '@next/b', 'dependencies'),
              expectedMismatch('outside@1.0.0', 'ungrouped', 'dependencies'),
            ],
          },
        ]);
      });
    });
  });

  describe('when versions differ outside a group, but match inside the group', () => {
    it('returns mismatches for outside the group only', () => {
      const config: SyncpackConfig = {
        ...DEFAULT_CONFIG,
        versionGroups: [{ dependencies: ['core'], packages: ['@next/a', '@next/b'] }],
      };
      const iterator = getMismatchedDependencies(
        [
          mockWrapper({ name: '@next/a', dependencies: { core: '0.0.0' } }),
          mockWrapper({ name: '@next/b', dependencies: { core: '0.0.0' } }),
          mockWrapper({ name: '@legacy/a', dependencies: { core: '0.0.1' } }),
          mockWrapper({ name: '@legacy/b', dependencies: { core: '0.0.0' } }),
          mockWrapper({ name: 'ungrouped', dependencies: { outside: '1.0.0' } }),
        ],
        config,
      );
      expect(Array.from(iterator)).toEqual([
        {
          name: 'core',
          installations: [
            expectedMismatch('core@0.0.1', '@legacy/a', 'dependencies'),
            expectedMismatch('core@0.0.0', '@legacy/b', 'dependencies'),
          ],
        },
      ]);
    });
  });

  describe('when versions differ inside a group and outside it', () => {
    it('returns mismatches for inside and outside the group', () => {
      const config: SyncpackConfig = {
        ...DEFAULT_CONFIG,
        versionGroups: [{ dependencies: ['core'], packages: ['@next/a', '@next/b'] }],
      };
      const iterator = getMismatchedDependencies(
        [
          mockWrapper({ name: '@next/a', dependencies: { core: '0.1.0' } }),
          mockWrapper({ name: '@next/b', dependencies: { core: '0.2.0' } }),
          mockWrapper({ name: '@legacy/a', dependencies: { core: '0.0.1' } }),
          mockWrapper({ name: '@legacy/b', dependencies: { core: '0.0.0' } }),
          mockWrapper({ name: 'ungrouped', dependencies: { outside: '1.0.0' } }),
        ],
        config,
      );
      expect(Array.from(iterator)).toEqual([
        {
          name: 'core',
          installations: [
            expectedMismatch('core@0.1.0', '@next/a', 'dependencies'),
            expectedMismatch('core@0.2.0', '@next/b', 'dependencies'),
          ],
        },
        {
          name: 'core',
          installations: [
            expectedMismatch('core@0.0.1', '@legacy/a', 'dependencies'),
            expectedMismatch('core@0.0.0', '@legacy/b', 'dependencies'),
          ],
        },
      ]);
    });
  });

  describe('when versions differ inside multiple groups, and outside the groups', () => {
    it('returns mismatches for all groups and outside the groups', () => {
      const config: SyncpackConfig = {
        ...DEFAULT_CONFIG,
        versionGroups: [{ dependencies: ['core'], packages: ['@next/a', '@next/b'] }],
      };
      const iterator = getMismatchedDependencies(
        [
          mockWrapper({ name: '@group1/a', dependencies: { foo: '0.0.1' } }),
          mockWrapper({ name: '@group1/b', dependencies: { foo: '0.0.2' } }),
          mockWrapper({ name: '@group2/a', dependencies: { bar: '0.1.0' } }),
          mockWrapper({ name: '@group2/b', dependencies: { bar: '0.2.0' } }),
          mockWrapper({ name: '@ungrouped/a', dependencies: { baz: '1.0.0' } }),
          mockWrapper({ name: '@ungrouped/b', dependencies: { baz: '2.0.0' } }),
        ],
        config,
      );
      expect(Array.from(iterator)).toEqual([
        {
          name: 'foo',
          installations: [
            expectedMismatch('foo@0.0.1', '@group1/a', 'dependencies'),
            expectedMismatch('foo@0.0.2', '@group1/b', 'dependencies'),
          ],
        },
        {
          name: 'bar',
          installations: [
            expectedMismatch('bar@0.1.0', '@group2/a', 'dependencies'),
            expectedMismatch('bar@0.2.0', '@group2/b', 'dependencies'),
          ],
        },
        {
          name: 'baz',
          installations: [
            expectedMismatch('baz@1.0.0', '@ungrouped/a', 'dependencies'),
            expectedMismatch('baz@2.0.0', '@ungrouped/b', 'dependencies'),
          ],
        },
      ]);
    });
  });
});
