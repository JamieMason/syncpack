import path from 'path';
import bluebird from 'bluebird';
import chalk from 'chalk';
import semver from 'semver';
import getFiles from './lib/get-files';
import writeJson from './lib/write-json';

const keys = (object = {}) => Object.keys(object);
const concatAll = arrayOfArrays => [].concat.apply([], arrayOfArrays);
const entries = object => keys(object).map(key => [key, object[key]]);
const isEmptyObject = object => keys(object).length === 0;
const pluck = key => object => object[key];

const stripWildCards = version => version.replace(/[*^=><]/g, '');
const getPackage = location => ({ location, json: require(location) });
const takeNewest = (max, next) =>
  !semver.valid(stripWildCards(next)) || semver.gt(stripWildCards(next), stripWildCards(max))
    ? next
    : max;

const indexEntries = array =>
  array.reduce((index, [key, value]) => {
    index[key] = index[key] || [];
    index[key] = index[key].indexOf(value) === -1 ? index[key].concat(value) : index[key];
    return index;
  }, {});

const getNewestDeps = (key, packages) => {
  const dependencies = concatAll(packages.map(pluck('json')).map(pluck(key)).map(entries));
  const versionsByDependencyName = indexEntries(dependencies);
  return entries(versionsByDependencyName).map(([name, versions]) => {
    const newest = versions.reduce(takeNewest, '0.0.0');
    return [name, newest];
  });
};

const getChangedDeps = (key, packages) =>
  packages.map(pkg =>
    getNewestDeps(key, packages).reduce((changes, [name, version]) => {
      if (pkg.json[key] && name in pkg.json[key] && pkg.json[key][name] !== version) {
        changes[name] = version;
      }
      return changes;
    }, {})
  );

const reportChanges = (key, pkg, changes) => {
  const changedEntries = entries(changes);
  if (changedEntries.length > 0) {
    changedEntries.forEach(([name, version]) => {
      console.log(`${key} ${name} ${chalk.red(pkg.json[key][name])} → ${chalk.green(version)}`);
    });
  } else {
    console.log(`${key} ${chalk.green('✓ unchanged')}`);
  }
};

export default async ({ pattern = './packages/*/package.json' }) => {
  const packages = (await getFiles(pattern)).map(getPackage);
  const changedDeps = getChangedDeps('dependencies', packages);
  const changedDevDeps = getChangedDeps('devDependencies', packages);
  const changedPeerDeps = getChangedDeps('peerDependencies', packages);

  const nextPackages = packages.map(({ location, json }, i) => {
    const nextPkg = {
      location,
      json: {
        ...json,
        dependencies: { ...json.dependencies, ...changedDeps[i] },
        devDependencies: { ...json.devDependencies, ...changedDevDeps[i] },
        peerDependencies: { ...json.peerDependencies, ...changedPeerDeps[i] }
      }
    };
    if (isEmptyObject(nextPkg.json.dependencies)) {
      delete nextPkg.json.dependencies;
    }
    if (isEmptyObject(nextPkg.json.devDependencies)) {
      delete nextPkg.json.devDependencies;
    }
    if (isEmptyObject(nextPkg.json.peerDependencies)) {
      delete nextPkg.json.peerDependencies;
    }
    return nextPkg;
  });

  packages.forEach((pkg, i) => {
    console.log(chalk.grey.underline(path.relative(process.cwd(), pkg.location)));
    reportChanges('dependencies', pkg, changedDeps[i]);
    reportChanges('devDependencies', pkg, changedDevDeps[i]);
    reportChanges('peerDependencies', pkg, changedPeerDeps[i]);
  });

  await bluebird.all(nextPackages.map(pkg => writeJson(pkg.location, pkg.json)));
};
