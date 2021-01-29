import * as mock from '../../test/mock';
import { DEFAULT_CONFIG } from '../constants';
import * as api from './fix-mismatches';

describe('fixMismatches', () => {
  let fixMismatches: typeof api.fixMismatches;

  afterEach(() => {
    jest.restoreAllMocks();
  });

  beforeEach(() => {
    jest.mock('./lib/log', () => ({ log: jest.fn() }));
    fixMismatches = require('./fix-mismatches').fixMismatches;
  });

  describe('when dependencies are installed with different versions', () => {
    describe('when the dependency is not a package maintained in this workspace', () => {
      it('uses the highest version', () => {
        const wrappers = [mock.wrapper('a', ['foo@0.1.0']), mock.wrapper('b', [], ['foo@0.2.0'])];
        fixMismatches(wrappers, DEFAULT_CONFIG);
        expect(wrappers).toMatchSnapshot();
      });
    });

    describe('when the dependency is a package maintained in this workspace', () => {
      it('uses the workspace version', () => {
        const wrappers = [
          mock.wrapper('a', ['foo@0.1.0']),
          mock.wrapper('b', [], ['foo@0.2.0']),
          mock.wrapper('foo', [], [], [], { name: 'foo', version: '0.0.1' }),
        ];
        fixMismatches(wrappers, DEFAULT_CONFIG);
        expect(wrappers).toMatchSnapshot();
      });
    });
  });

  it('ignores non-semver dependencies', () => {
    const wrappers = [
      mock.wrapper('a', ['foo@link:vendor/foo-0.1.0']),
      mock.wrapper('b', ['foo@link:vendor/foo-0.2.0']),
    ];
    fixMismatches(wrappers, DEFAULT_CONFIG);
    expect(wrappers).toMatchSnapshot();
  });
});
