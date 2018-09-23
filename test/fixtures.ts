import { IManifest } from '../src/typings';
import { createManifest } from './helpers';

export const getAnyVersionManifests = () => [
  createManifest(
    'foo',
    { chalk: '*', commander: '*' },
    { jest: '*', prettier: '*', rimraf: '*' },
    { gulp: '*' }
  ),
  createManifest('bar', { chalk: '*' }, { jest: '*' }),
  createManifest(
    'baz',
    null,
    { npm: 'https://github.com/npm/npm.git', prettier: '*' },
    { gulp: '*' }
  )
];

export const getExactVersionManifests = () => [
  createManifest(
    'foo',
    { chalk: '2.3.0', commander: '2.13.0' },
    { jest: '22.1.3', prettier: '1.10.2', rimraf: '2.6.2' },
    { gulp: '0.9.1' }
  ),
  createManifest('bar', { chalk: '1.0.0' }, { jest: '22.1.4' }),
  createManifest(
    'baz',
    null,
    { npm: 'https://github.com/npm/npm.git', prettier: '1.10.2' },
    { gulp: '*' }
  )
];

export const getGreaterThanOrEqualVersionManifests = () => [
  createManifest(
    'foo',
    { chalk: '>=2.3.0', commander: '>=2.13.0' },
    { jest: '>=22.1.3', prettier: '>=1.10.2', rimraf: '>=2.6.2' },
    { gulp: '>=0.9.1' }
  ),
  createManifest('bar', { chalk: '>=1.0.0' }, { jest: '>=22.1.4' }),
  createManifest(
    'baz',
    null,
    { npm: 'https://github.com/npm/npm.git', prettier: '>=1.10.2' },
    { gulp: '*' }
  )
];

export const getGreaterThanVersionManifests = () => [
  createManifest(
    'foo',
    { chalk: '>2.3.0', commander: '>2.13.0' },
    { jest: '>22.1.3', prettier: '>1.10.2', rimraf: '>2.6.2' },
    { gulp: '>0.9.1' }
  ),
  createManifest('bar', { chalk: '>1.0.0' }, { jest: '>22.1.4' }),
  createManifest(
    'baz',
    null,
    { npm: 'https://github.com/npm/npm.git', prettier: '>1.10.2' },
    { gulp: '*' }
  )
];

export const getLessThanOrEqualVersionManifests = () => [
  createManifest(
    'foo',
    { chalk: '<=2.3.0', commander: '<=2.13.0' },
    { jest: '<=22.1.3', prettier: '<=1.10.2', rimraf: '<=2.6.2' },
    { gulp: '<=0.9.1' }
  ),
  createManifest('bar', { chalk: '<=1.0.0' }, { jest: '<=22.1.4' }),
  createManifest(
    'baz',
    null,
    { npm: 'https://github.com/npm/npm.git', prettier: '<=1.10.2' },
    { gulp: '*' }
  )
];

export const getLessThanVersionManifests = () => [
  createManifest(
    'foo',
    { chalk: '<2.3.0', commander: '<2.13.0' },
    { jest: '<22.1.3', prettier: '<1.10.2', rimraf: '<2.6.2' },
    { gulp: '<0.9.1' }
  ),
  createManifest('bar', { chalk: '<1.0.0' }, { jest: '<22.1.4' }),
  createManifest(
    'baz',
    null,
    { npm: 'https://github.com/npm/npm.git', prettier: '<1.10.2' },
    { gulp: '*' }
  )
];

export const getLooseVersionManifests = () => [
  createManifest(
    'foo',
    { chalk: '2.x.x', commander: '2.x.x' },
    { jest: '22.x.x', prettier: '1.x.x', rimraf: '2.x.x' },
    { gulp: '0.9.x' }
  ),
  createManifest('bar', { chalk: '1.x.x' }, { jest: '22.x.x' }),
  createManifest(
    'baz',
    null,
    { npm: 'https://github.com/npm/npm.git', prettier: '1.x.x' },
    { gulp: '*' }
  )
];

export const getMinorVersionManifests = () => [
  createManifest(
    'foo',
    { chalk: '^2.3.0', commander: '^2.13.0' },
    { jest: '^22.1.3', prettier: '^1.10.2', rimraf: '^2.6.2' },
    { gulp: '^0.9.1' }
  ),
  createManifest('bar', { chalk: '^1.0.0' }, { jest: '^22.1.4' }),
  createManifest(
    'baz',
    null,
    { npm: 'https://github.com/npm/npm.git', prettier: '^1.10.2' },
    { gulp: '*' }
  )
];

export const getPatchVersionManifests = () => [
  createManifest(
    'foo',
    { chalk: '~2.3.0', commander: '~2.13.0' },
    { jest: '~22.1.3', prettier: '~1.10.2', rimraf: '~2.6.2' },
    { gulp: '~0.9.1' }
  ),
  createManifest('bar', { chalk: '~1.0.0' }, { jest: '~22.1.4' }),
  createManifest(
    'baz',
    null,
    { npm: 'https://github.com/npm/npm.git', prettier: '~1.10.2' },
    { gulp: '*' }
  )
];

export const getUntidyManifest = (): IManifest =>
  JSON.parse(`{
  "files": [
    "assets",
    "dist"
  ],
  "bugs": {
    "url": "https://github.com/JaneDoe/do-it/issues"
  },
  "author": "Jane Doe <jane@doe.io>",
  "devDependencies": {
    "waldorf": "22.1.4",
    "stroopwafel": "4.4.2"
  },
  "scripts": {
    "test": "jest",
    "build": "tsc",
    "lint": "tslint",
    "format": "prettier"
  },
  "version": "1.0.2",
  "main": "do-it",
  "license": "MIT",
  "description": "Does the thing",
  "homepage": "https://github.com/JaneDoe/do-it#readme",
  "dependencies": {
    "guybrush": "7.1.1",
    "arnold": "5.0.0",
    "dog": "2.13.0",
    "mango": "2.3.0"
  },
  "name": "do-it",
  "repository": {
    "url": "git://github.com/JaneDoe/do-it",
    "type": "git"
  },
  "keywords": [
    "those",
    "whatsits",
    "thing"
  ],
  "bin": {
    "zoo": "dist/zoo.js",
    "moose": "dist/moose.js",
    "apple": "dist/apple.js"
  },
  "peerDependencies": {
    "jambalaya": "6.1.4",
    "giftwrap": "0.1.2",
    "zoolander": "1.4.25"
  }
}
`);
