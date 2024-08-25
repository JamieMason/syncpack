import { readFileSync } from 'node:fs';
import { globbySync } from 'globby';
import { satisfies } from 'semver';
import root from '../package.json' assert { type: 'json' };

const syncpackEngine = root.engines.node.replace('>=', '');

const unsatisfiedDependencies = globbySync('node_modules/**/package.json')
  .map(filePath => readFileSync(filePath, 'utf8'))
  .filter(json => json.includes('"engines"'))
  .map(json => JSON.parse(json))
  .filter(file => file.engines?.node)
  .map(file => {
    const name = file.name;
    const dependencyEngine = file.engines.node;
    const isSatisfied = satisfies(syncpackEngine, dependencyEngine);
    return {
      name,
      expected: syncpackEngine,
      actual: dependencyEngine,
      isSatisfied,
    };
  })
  .filter(result => !result.isSatisfied);

if (unsatisfiedDependencies.length) {
  console.error(
    'The following dependencies have incompatible engines:',
    unsatisfiedDependencies,
  );
  process.exit(1);
} else {
  console.info('All dependencies have compatible engines.');
}
