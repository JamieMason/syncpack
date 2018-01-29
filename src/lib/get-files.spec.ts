afterEach(() => {
  jest.resetModules();
});

it('rejects with error thrown upstream', async () => {
  jest.doMock('glob', () => jest.fn((pattern, options, cb) => cb(new Error('wat?'))));
  const { getFiles } = require('./get-files');
  try {
    await getFiles('**/*.txt');
  } catch (err) {
    expect(err).toEqual(new Error('wat?'));
  }
});

it('resolves with absolute paths to files matching a given glob', async () => {
  jest.doMock('glob', () => jest.fn((pattern, options, cb) => cb(null, ['/Usr/you/foo.txt', '/Usr/you/bar.txt'])));
  const { getFiles } = require('./get-files');
  const returnedPaths = await getFiles('**/*.txt');
  expect(returnedPaths).toEqual(['/Usr/you/foo.txt', '/Usr/you/bar.txt']);
});
