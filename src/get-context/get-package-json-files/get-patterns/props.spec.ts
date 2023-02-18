import { O } from '@mobily/ts-belt';
import { isNumber } from 'expect-more/dist/is-number';
import { isString } from 'expect-more/dist/is-string';
import { isUndefined } from 'expect-more/dist/is-undefined';
import { props } from './props';

it('Some when value is found and passes predicate', () => {
  expect(props('a.b', isNumber)({ a: { b: 1 } })).toEqual(O.Some(1));
  expect(props('a.0', isNumber)({ a: [1] })).toEqual(O.Some(1));
});

it('None when value is found but fails predicate', () => {
  expect(props('a.b', isString)({ a: { b: 1 } })).toEqual(O.None);
  expect(props('a.0', isString)({ a: [1] })).toEqual(O.None);
});

it('None when value is not found', () => {
  expect(props('a.b', isString)({})).toEqual(O.None);
  expect(props('a.b', isString)([])).toEqual(O.None);
  expect(props('a.b', isString)(undefined)).toEqual(O.None);
  expect(props('a.b', isString)(null)).toEqual(O.None);
});

it('None when value is not found but matches predicate', () => {
  expect(props('a.b', isUndefined)({})).toEqual(O.None);
  expect(props('a.b', isUndefined)(undefined)).toEqual(O.None);
});
