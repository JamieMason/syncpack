import * as mock from '../../test/mock';
import * as api from './list';

describe('list', () => {
  let list: typeof api.list;
  let log: jest.Mock;

  afterEach(() => {
    jest.restoreAllMocks();
  });

  beforeEach(() => {
    jest.mock('./lib/log', () => ({ log: jest.fn() }));
    list = require('./list').list;
    log = require('./lib/log').log;
  });

  it('outputs all dependencies', () => {
    const wrappers = [mock.wrapper('a', ['foo@0.1.0']), mock.wrapper('b', ['foo@0.2.0'])];
    list(['dependencies'], [/./], wrappers);
    expect(log.mock.calls).toMatchSnapshot();
  });
});
