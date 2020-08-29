import 'expect-more-jest';
import { DEFAULT_CONFIG } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { Installation } from './get-dependencies';
import { getMismatchedDependencies } from './get-mismatched-dependencies';

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

describe('getMismatchedDependencies', () => {
  it('lists dependencies installed with different versions', () => {
    const iterator = getMismatchedDependencies(mocked.projects(), DEFAULT_CONFIG);
    expect(Array.from(iterator)).toEqual([getShape('chalk', ['dependencies', '2.3.0'], ['dependencies', '1.0.0'])]);
  });
});
