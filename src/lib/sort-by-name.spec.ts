import { describe, expect, it } from 'vitest';
import { sortByName } from './sort-by-name.js';

describe('sortByName', () => {
  it('orders installed packages by name', async () => {
    const toShape = (name: string) => ({
      name,
    });
    const unordered = ['c', 'a', 'b', 'c'].map(toShape);
    const ordered = ['a', 'b', 'c', 'c'].map(toShape);
    expect([...unordered].sort(sortByName)).toEqual(ordered);
  });
});
