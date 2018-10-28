import chalk from 'chalk';
import { writeJson } from 'fs-extra';
import _ = require('lodash');
import { relative } from 'path';
import {
  OPTION_INDENT,
  OPTION_SOURCES,
  SORT_AZ,
  SORT_FIRST
} from './constants';
import { collect } from './lib/collect';
import { getIndent } from './lib/get-indent';
import { getPackages } from './lib/get-packages';
import { CommanderApi, IManifest } from './typings';

export const run = async (program: CommanderApi) => {
  const shortenBugs = (manifest: IManifest): IManifest => {
    const bugsUrl = _.get(manifest, 'bugs.url') as string;
    return bugsUrl ? { ...manifest, bugs: bugsUrl } : manifest;
  };

  const shortenRepository = (manifest: IManifest): IManifest => {
    const repoUrl = _.get(manifest, 'repository.url', '') as string;
    return repoUrl.includes('github.com')
      ? { ...manifest, repository: repoUrl.split('github.com/')[1] }
      : manifest;
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
    .option(OPTION_INDENT.spec, OPTION_INDENT.description)
    .parse(process.argv);

  const pkgs = getPackages(program);
  const indent = getIndent(program);

  await Promise.all(
    pkgs.map(({ data, path }) => {
      console.log(chalk.blue(`./${relative('.', path)}`));
      const nextData = sortManifest(shortenBugs(shortenRepository(data)));
      return writeJson(path, nextData, { spaces: indent });
    })
  );
};
