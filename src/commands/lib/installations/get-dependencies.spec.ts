import 'expect-more-jest';
import { DEFAULT_CONFIG } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { getDependencies, Installation } from './get-dependencies';
import { withJson } from '../../../../test/mock';

const mocked = {
  projects: (): SourceWrapper[] =>
    [
      { filePath: '', contents: { dependencies: { chalk: '2.3.0' } } },
      { filePath: '', contents: { devDependencies: { jest: '22.1.4' } } },
      { filePath: '', contents: { peerDependencies: { jest: '22.1.4' } } },
      { filePath: '', contents: { dependencies: { chalk: '1.0.0' } } },
      { filePath: '', contents: { dependencies: { biggy: '0.1.0' } } },
    ].map((obj) => withJson(obj)),
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
