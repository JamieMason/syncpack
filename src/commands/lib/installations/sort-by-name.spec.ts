import 'expect-more-jest';
import { Installation } from './get-dependencies';
import { sortByName } from './sort-by-name';

type ExpectedShape = {
  installations: Installation[];
  name: string;
};

const getShape = (name: string, ...installations: Array<[string, string]>): ExpectedShape => ({
  installations: installations.map(([type, version]) => expect.objectContaining({ name, type, version })),
  name,
});

describe('sortByName', () => {
  it('orders installed packages by name', () => {
    const toShape = (name: string): ExpectedShape => getShape(name);
    const unordered = ['c', 'a', 'b', 'c'].map(toShape);
    const ordered = ['a', 'b', 'c', 'c'].map(toShape);
    expect(unordered.sort(sortByName)).toEqual(ordered);
  });
});
