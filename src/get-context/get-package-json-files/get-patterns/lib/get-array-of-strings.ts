import { isArrayOfStrings } from 'expect-more';
import { props } from '../props';

// Yarn's config for this can be in more than one place
export const getArrayOfStrings = (path: string) =>
  props<string[]>(path, isArrayOfStrings);
