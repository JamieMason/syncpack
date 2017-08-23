import path from 'path';
import chalk from 'chalk';
import { get, set } from 'lodash';
import getFiles from './lib/get-files';
import writeJson from './lib/write-json';

const getPackage = location => ({ location, json: require(location) });
const toJson = value => JSON.stringify(value);

const reportChange = (key, previous, next) => {
  if (toJson(previous) === toJson(next)) {
    console.log(`${key}: ${chalk.green('✓ unchanged')}`);
  } else {
    console.log(`${key}: ${chalk.red(toJson(previous))} → ${chalk.green(toJson(next))}`);
  }
};

export default async ({ keys, packagesPattern, sourcePattern }) => {
  const [source] = (await getFiles(sourcePattern)).map(getPackage);
  const packages = (await getFiles(packagesPattern)).map(getPackage);
  packages.forEach(pkg => {
    console.log(chalk.grey.underline(path.relative(process.cwd(), pkg.location)));
    keys.forEach(key => {
      const value = get(source.json, key);
      const previousValue = get(pkg.json, key);
      set(pkg.json, key, value);
      writeJson(pkg.location, pkg.json);
      reportChange(key, previousValue, value);
    });
  });
};
