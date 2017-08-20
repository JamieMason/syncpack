import path from 'path';
import chalk from 'chalk';
import { get, set } from 'lodash';
import getFiles from './lib/get-files';

const getPackage = location => ({ location, json: require(location) });
const formatValue = value => JSON.stringify(value);

export default async ({ keys, packagesPattern, sourcePattern }) => {
  const [source] = (await getFiles(sourcePattern)).map(getPackage);
  const packages = (await getFiles(packagesPattern)).map(getPackage);

  packages.forEach(pkg => {
    console.log(chalk.grey.underline(path.relative(process.cwd(), pkg.location)));
    keys.forEach(key => {
      const value = get(source.json, key);
      const previousValue = formatValue(get(pkg.json, key));
      set(pkg.json, key, value);
      const nextValue = formatValue(value);
      if (previousValue === nextValue) {
        console.log(`${key}: ${chalk.green('✓ unchanged')}`);
      } else {
        console.log(`${key}: ${chalk.red(previousValue)} → ${chalk.green(nextValue)}`);
      }
    });
  });
};
