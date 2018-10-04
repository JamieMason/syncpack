import chalk from 'chalk';
import { CommanderStatic } from 'commander';
import _ = require('lodash');
import { relative } from 'path';
import { OPTION_SOURCES, SORT_AZ, SORT_FIRST } from './constants';
import { collect } from './lib/collect';
import { getPackages } from './lib/get-packages';
import { writeJson } from './lib/write-json';
import { IManifest } from './typings';

export const run = async (program: CommanderStatic) => {
  const shortenBugs = (manifest: IManifest): IManifest => {
    if (
      manifest.bugs &&
      typeof manifest.bugs === 'object' &&
      manifest.bugs.url
    ) {
      return {
        ...manifest,
        bugs: manifest.bugs.url
      };
    }
    return manifest;
  };

  const shortenRepository = (manifest: IManifest): IManifest => {
    if (
      manifest.repository &&
      typeof manifest.repository === 'object' &&
      manifest.repository.url &&
      manifest.repository.url.indexOf('github.com') !== -1
    ) {
      return {
        ...manifest,
        repository: manifest.repository.url.split('github.com/')[1]
      };
    }
    return manifest;
  };

  const sortObject = (obj: IManifest) =>
    _(obj)
      .entries()
      .sortBy('0')
      .reduce((next, [key, value]) => ({ ...next, [key]: value }), {});

  const sortValue = (value: any) =>
    _.isArray(value)
      ? value.slice(0).sort()
      : _.isObject(value)
        ? sortObject(value)
        : value;

  const sortManifest = (manifest: IManifest): IManifest => {
    const [first, rest] = _(manifest)
      .entries()
      .sortBy('0')
      .partition(([key, value]) => SORT_FIRST.indexOf(key) !== -1)
      .value();

    const firstSorted = [...first].sort(
      ([keyA], [keyB]) => SORT_FIRST.indexOf(keyA) - SORT_FIRST.indexOf(keyB)
    );

    const restSorted = _(rest)
      .map(([key, value]) => [
        key,
        SORT_AZ.indexOf(key) !== -1 ? sortValue(value) : value
      ])
      .value();

    return _([...firstSorted, ...restSorted]).reduce(
      (obj, [key, value]) => ({ ...obj, [key]: value }),
      {} as IManifest
    );
  };

  program
    .option(OPTION_SOURCES.spec, OPTION_SOURCES.description, collect)
    .parse(process.argv);

  const pkgs = await getPackages(program);

  await Promise.all(
    pkgs.map(({ data, path }) =>
      writeJson(path, sortManifest(shortenBugs(shortenRepository(data))))
    )
  );

  _.each(pkgs, (pkg) => {
    console.log(chalk.blue(`./${relative('.', pkg.path)}`));
  });
};
