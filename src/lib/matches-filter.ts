import type { ProgramInput } from '../lib/get-input';
import type { Instance } from '../lib/get-input/get-instances';

export function matchesFilter(input: ProgramInput) {
  return function hasNameMatchingFilter({ name }: Instance): boolean {
    return name.search(new RegExp(input.filter)) !== -1;
  };
}
