import 'expect-more-jest';
import { DEFAULT_CONFIG, DependencyType } from '../../../constants';
import { Source, SourceWrapper } from '../get-wrappers';
import { getInstallations } from './get-installations';
import { Installation } from './get-dependencies';


const filePath = '';
const sourceWrapper = (source: Source): SourceWrapper => ({contents: source, filePath})

const sources: Source[] = [
  { name: 'package1', dependencies: { chalk: '2.3.0' } },
  { name: 'package2', peerDependencies: { jest: '22.1.4' } },
  { name: 'package3', dependencies: { biggy: '0.1.0' } },
  { name: 'package4', devDependencies: { jest: '0.1.0' } }
];

const sourceWrappers = sources.map(source => sourceWrapper(source))

const installation = (source: SourceWrapper, dependencyName: string, dependencyVersion: string, dependencyType: DependencyType): Installation => ({
  name: dependencyName,
  source,
  type:dependencyType,
  version: dependencyVersion
})

describe('getInstallations', () => {
  it('lists all installations', () => {
    const iterator = getInstallations(sourceWrappers, DEFAULT_CONFIG);
    expect(Array.from(iterator)).toEqual([
      installation(sourceWrappers[0], 'chalk', '2.3.0', 'dependencies'),
      installation(sourceWrappers[2], 'biggy', '0.1.0', 'dependencies'),
      installation(sourceWrappers[3], 'jest', '0.1.0', 'devDependencies'),
      installation(sourceWrappers[1], 'jest', '22.1.4', 'peerDependencies')
    ]);
  });

  it('lists all installations of packages matching the filter', () => {
    const iterator = getInstallations(sourceWrappers, { ...DEFAULT_CONFIG, filter: 'jes|b' });
    expect(Array.from(iterator)).toEqual([
      installation(sourceWrappers[2], 'biggy', '0.1.0', 'dependencies'),
      installation(sourceWrappers[3], 'jest', '0.1.0', 'devDependencies'),
      installation(sourceWrappers[1], 'jest', '22.1.4', 'peerDependencies')
    ]);
  });
});
