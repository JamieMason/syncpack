import 'expect-more-jest';
import { DEFAULT_CONFIG } from '../../constants';
import { getDependencies, getMismatchedDependencies, Installation, sortByName } from './get-installations';
import { SourceWrapper } from './get-wrappers';

const mocked = {
  projects: (): SourceWrapper[] => [
    { filePath: '', contents: { dependencies: { chalk: '2.3.0' } } },
    { filePath: '', contents: { devDependencies: { jest: '22.1.4' } } },
    { filePath: '', contents: { peerDependencies: { jest: '22.1.4' } } },
    { filePath: '', contents: { dependencies: { chalk: '1.0.0' } } },
    { filePath: '', contents: { dependencies: { biggy: '0.1.0' } } },
  ],
};

type ExpectedShape = {
  installations: Installation[];
  name: string;
};

const getShape = (name: string, ...installations: Array<[string, string]>): ExpectedShape => ({
  installations: installations.map(([type, version]) => expect.objectContaining({ name, type, version })),
  name,
});

describe('getDependencies', () => {
  it('lists all dependencies and their versions', () => {
    const iterator = getDependencies(mocked.projects(), DEFAULT_CONFIG);
    expect(Array.from(iterator)).toEqual([
      getShape('chalk', ['dependencies', '2.3.0'], ['dependencies', '1.0.0']),
      getShape('biggy', ['dependencies', '0.1.0']),
      getShape('jest', ['devDependencies', '22.1.4'], ['peerDependencies', '22.1.4']),
    ]);
  });
});

describe('getMismatchedDependencies', () => {
  it('lists dependencies installed with different versions', () => {
    const iterator = getMismatchedDependencies(mocked.projects(), DEFAULT_CONFIG);
    expect(Array.from(iterator)).toEqual([getShape('chalk', ['dependencies', '2.3.0'], ['dependencies', '1.0.0'])]);
  });
});

describe('sortByName', () => {
  it('orders installed packages by name', () => {
    const toShape = (name: string): ExpectedShape => getShape(name);
    const unordered = ['c', 'a', 'b', 'c'].map(toShape);
    const ordered = ['a', 'b', 'c', 'c'].map(toShape);
    expect(unordered.sort(sortByName)).toEqual(ordered);
  });
});
