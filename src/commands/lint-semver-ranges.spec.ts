import 'expect-more-jest';
import * as mock from '../../test/mock';
import { DEFAULT_CONFIG } from '../constants';
import * as api from './lint-semver-ranges';

describe('lint-semver-ranges', () => {
  let lintSemverRanges: typeof api.lintSemverRanges;
  let log: jest.Mock;

  afterEach(() => {
    jest.restoreAllMocks();
  });

  beforeEach(() => {
    jest.mock('./lib/log', () => ({ log: jest.fn() }));
    lintSemverRanges = require('./lint-semver-ranges').lintSemverRanges;
    log = require('./lib/log').log;
  });

  it('outputs all dependencies with incorrect versions', () => {
    const wrappers = [
      mock.wrapper('a', ['foo@0.1.0'], [], [], {name: 'pkg1'}),
      mock.wrapper('b', ['foo@0.2.0', 'bar@^0.2.0', 'baz@~0.3.0'], [], [], {name: 'pkg2'}),
    ];
    lintSemverRanges(wrappers, { ...DEFAULT_CONFIG, dev: false, peer: false, prod: true });
    expect(log.mock.calls).toMatchSnapshot();
  });
});
