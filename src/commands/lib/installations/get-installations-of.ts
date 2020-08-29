import { DependencyType } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { Installation } from './get-dependencies';

export function* getInstallationsOf(
  name: string,
  types: DependencyType[],
  wrappers: SourceWrapper[],
): Generator<Installation> {
  for (const type of types) {
    for (const wrapper of wrappers) {
      const dependencies = wrapper.contents[type];
      if (dependencies && dependencies[name]) {
        yield {
          name,
          source: wrapper,
          type,
          version: dependencies[name],
        };
      }
    }
  }
}
