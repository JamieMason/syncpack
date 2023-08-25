// Hide console.log output (use console.info instead to debug in tests)
beforeAll(() => {
  jest.spyOn(console, 'log').mockImplementation(() => {});
});
