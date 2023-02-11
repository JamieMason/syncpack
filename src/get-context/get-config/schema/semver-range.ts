import { z } from 'zod';
import { RANGE } from '../../../constants';

export const value = z.enum([
  RANGE.ANY,
  RANGE.EXACT,
  RANGE.GT,
  RANGE.GTE,
  RANGE.LOOSE,
  RANGE.LT,
  RANGE.LTE,
  RANGE.MINOR,
  RANGE.PATCH,
]);
