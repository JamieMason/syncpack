import { readJsonSync } from 'fs-extra';
import _ = require('lodash');
import { CommanderApi } from '../src/typings';

const shuffle = (value: any): typeof value =>
  _.isArray(value)
    ? _.shuffle(value)
    : _.isObject(value)
    ? shuffleObject(value)
    : value;

export const shuffleObject = (obj: object): object =>
  _(obj)
    .entries()
    .map(([key, value]) => [key, shuffle(value)])
    .shuffle()
    .reduce((next, [key, value]) => ({ ...next, [key]: value }), {});

export const getFixture = (name: string, transform = (a: any | any[]) => a) => {
  const fixturePath = `${__dirname}/fixtures/${name}.json`;
  const data = transform(readJsonSync(fixturePath));
  const json = JSON.stringify(data, null, 2);
  return { data, json };
};

export const getMockCommander = (source: string[]) => {
  const program = ({
    command: () => program,
    option: () => program,
    parse: () => program,
    source,
    version: () => program
  } as unknown) as CommanderApi;
  return program;
};
