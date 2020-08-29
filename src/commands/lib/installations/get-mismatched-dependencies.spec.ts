import 'expect-more-jest';
import { DEFAULT_CONFIG } from '../../../constants';
import { SourceWrapper } from '../get-wrappers';
import { getMismatchedDependencies } from './get-mismatched-dependencies';

const types = permutations([
  'dependencies',
  'devDependencies',
  'peerDependencies',
  'dependencies',
  'devDependencies',
  'peerDependencies',
]);

describe('getMismatchedDependencies', () => {
  describe('when 2 versions of the same dependency in 2 packages match', () => {
    const versionA = '1.0.0';
    const versionB = '1.0.0';
    types.forEach(([typeA, typeB]) => {
      [true, false].forEach((dev) => {
        describe(`when dev ${dev}`, () => {
          [true, false].forEach((peer) => {
            describe(`when peer ${peer}`, () => {
              [true, false].forEach((prod) => {
                describe(`when prod ${prod}`, () => {
                  it('should find no mismatches', () => {
                    expect(
                      Array.from(
                        getMismatchedDependencies(
                          [{ [typeA]: { chalk: versionA } }, { [typeB]: { chalk: versionB } }].map(
                            (contents): SourceWrapper => ({ filePath: '', contents }),
                          ),
                          { ...DEFAULT_CONFIG, dev, peer, prod },
                        ),
                      ),
                    ).toBeEmptyArray();
                  });
                });
              });
            });
          });
        });
      });
    });
  });

  describe('when 2 versions of the same dependency in 2 packages differ', () => {
    const versionA = '1.0.0';
    const versionB = '2.0.0';
    types.forEach(([typeA, typeB]) => {
      describe(`between ${typeA} and ${typeB}`, () => {
        [true, false].forEach((dev) => {
          describe(`when dev ${dev}`, () => {
            [true, false].forEach((peer) => {
              describe(`when peer ${peer}`, () => {
                [true, false].forEach((prod) => {
                  describe(`when prod ${prod}`, () => {
                    it('should find mismatches if present', () => {
                      const result = Array.from(
                        getMismatchedDependencies(
                          [{ [typeA]: { chalk: versionA } }, { [typeB]: { chalk: versionB } }].map(
                            (contents): SourceWrapper => ({ filePath: '', contents }),
                          ),
                          { ...DEFAULT_CONFIG, dev, peer, prod },
                        ),
                      );
                      expect(result).toMatchSnapshot();
                    });
                  });
                });
              });
            });
          });
        });
      });
    });
  });
});

function permutations(array: string[]) {
  const result: string[][] = [];
  const len = array.length;
  const tmp: number[] = [];
  function nodup() {
    const got: { [key: string]: boolean } = {};
    for (let l = 0; l < tmp.length; l++) {
      if (got[tmp[l]]) return false;
      got[tmp[l]] = true;
    }
    return true;
  }
  function iter(index: number) {
    let l: number;
    let rr: string[] = [];
    if (index === len) {
      if (nodup()) {
        rr = [];
        for (l = 0; l < tmp.length; l++) rr.push(array[tmp[l]]);
        result.push(rr);
      }
    } else {
      for (l = 0; l < len; l++) {
        tmp[index] = l;
        iter(index + 1);
      }
    }
  }
  iter(0);
  return result;
}
