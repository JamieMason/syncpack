import { log } from './log';

describe('log', () => {
  it('exists to make it easier to test logs from syncpack and not jest', () => {
    const actual = jest.spyOn(console, 'log').mockImplementation(() => undefined);
    log('hello', 'world');
    expect(actual).toHaveBeenCalledWith('hello', 'world');
    jest.restoreAllMocks();
  });
});
