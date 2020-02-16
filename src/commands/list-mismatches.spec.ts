import * as mock from '../../test/mock';

describe('listMismatches', () => {
  let listMismatches: any;
  let log: any;

  afterEach(() => {
    jest.restoreAllMocks();
  });

  beforeEach(() => {
    jest.mock('./lib/log', () => ({ log: jest.fn() }));
    listMismatches = require('./list-mismatches').listMismatches;
    log = require('./lib/log').log;
  });

  it('outputs all dependencies installed with different versions', () => {
    const wrappers = [mock.wrapper('a', ['foo@0.1.0']), mock.wrapper('b', ['foo@0.2.0'])];
    listMismatches(['dependencies'], /./, wrappers);
    expect(log.mock.calls).toMatchSnapshot();
  });
});
