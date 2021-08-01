import * as mock from '../../../test/mock';
import * as api from './write-if-changed';
import chalk = require('chalk');

describe('writeIfChanged', () => {
  let log: jest.Mock;
  let writeFileSync: jest.Mock;
  let writeIfChanged: typeof api.writeIfChanged;

  afterEach(() => {
    jest.restoreAllMocks();
    jest.resetModules();
  });

  beforeEach(() => {
    jest.mock('fs-extra', () => ({ writeFileSync: jest.fn() }));
    writeFileSync = require('fs-extra').writeFileSync;
    jest.mock('./log', () => ({ log: jest.fn() }));
    log = require('./log').log;
    writeIfChanged = require('./write-if-changed').writeIfChanged;
  });

  describe('when a package is mutated', () => {
    beforeEach(() => {
      const wrapper = mock.wrapper('a', ['foo@0.1.0']);
      writeIfChanged('  ', wrapper, () => {
        if (wrapper.contents.dependencies) {
          wrapper.contents.dependencies.foo = '0.2.0';
        }
      });
    });
    it('writes the changed file to disk', () => {
      expect(writeFileSync).toHaveBeenCalledWith('/a/package.json', expect.stringContaining('"foo": "0.2.0"'));
    });
    it('logs that the file has changed', () => {
      expect(log).toHaveBeenCalledWith(chalk.green('✓'), expect.stringContaining('/a/package.json'));
    });
  });
  describe('when a package is unchanged', () => {
    beforeEach(() => {
      const wrapper = mock.wrapper('b', ['bar@3.0.0']);
      writeIfChanged('  ', wrapper, () => undefined);
    });
    it('does not write to disk', () => {
      expect(writeFileSync).not.toHaveBeenCalled();
    });
    it('logs that the file has not changed', () => {
      expect(log).toHaveBeenCalledWith(chalk.dim('-'), expect.stringContaining('/b/package.json'));
    });
  });
});
