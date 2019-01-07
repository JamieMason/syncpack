import { collect } from './collect';

describe('collect', () => {
  it('concatenates values', () => {
    expect(collect('baz', ['foo', 'bar'])).toEqual(['foo', 'bar', 'baz']);
  });
});
