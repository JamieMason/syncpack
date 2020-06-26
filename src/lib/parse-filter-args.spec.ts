import { parseFilterArgs } from './parse-filter-args';

describe('parseFilterArgs', () => {
  it('converts the filter args from an array of strings to an array of RegExps', () => {
    const parsedFilterArgs = parseFilterArgs(['@react', 'webpack']);
    expect(parsedFilterArgs.length).toEqual(2);

    expect(parsedFilterArgs[0].toString()).toEqual('/@react/');
    expect(parsedFilterArgs[1].toString()).toEqual('/webpack/');
  });

  it('converts the filter args from a string to an array of RegExps', () => {
    const parsedFilterArgs = parseFilterArgs('@react');
    expect(parsedFilterArgs.length).toEqual(1);
    expect(parsedFilterArgs[0].toString()).toEqual('/@react/');
  });

  it('returns a match all RegExp when an empty string is specified', () => {
    const parsedFilterArgs = parseFilterArgs('');
    expect(parsedFilterArgs.length).toEqual(1);
    expect(parsedFilterArgs[0].toString()).toEqual('/(?:)/');
  });
});
