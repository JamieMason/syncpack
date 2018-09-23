afterEach(() => {
  jest.resetModules();
});

it('rejects with error thrown upstream', async () => {
  jest.doMock('fs', () => ({
    writeFile: jest.fn((path, data, options, cb) => cb(new Error('wat?')))
  }));
  const { writeJson } = require('./write-json');
  try {
    await writeJson('/Usr/you/some.json', { some: 'data' });
  } catch (err) {
    expect(err).toEqual(new Error('wat?'));
  }
});

it('resolves when file was written successfully', async () => {
  jest.doMock('fs', () => ({
    writeFile: jest.fn((path, data, options, cb) => cb(null))
  }));
  const { writeJson } = require('./write-json');
  const result = await writeJson('/Usr/you/some.json', { some: 'data' });
  expect(result).toBeUndefined();
});
