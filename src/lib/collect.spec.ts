import { collect } from './collect';

describe('collect', () => {
  it('is a "custom options processor" for commander.js to flatten arrays of strings', () => {
    expect(collect('c', ['a', 'b'])).toEqual(['a', 'b', 'c']);
  });
});
