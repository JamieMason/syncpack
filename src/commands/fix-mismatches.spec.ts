import * as mock from '../../test/mock';

describe('fixMismatches', () => {
  let fixMismatches: any;
  let log: any;

  afterEach(() => {
    jest.restoreAllMocks();
  });

  beforeEach(() => {
    jest.mock('./lib/log', () => ({ log: jest.fn() }));
    fixMismatches = require('./fix-mismatches').fixMismatches;
    log = require('./lib/log').log;
  });

  it('sets all dependencies installed with different versions to the highest version', () => {
    const wrappers = [mock.wrapper('a', ['foo@0.1.0']), mock.wrapper('b', ['foo@0.2.0'])];
    fixMismatches(['dependencies'], /./, wrappers);
    expect(wrappers).toMatchSnapshot();
  });
});
