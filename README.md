# syncpack

[![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![Build Status](http://img.shields.io/travis/JamieMason/syncpack/master.svg?style=flat-square)](https://travis-ci.org/JamieMason/syncpack)
[![Dependency Status](http://img.shields.io/david/JamieMason/syncpack.svg?style=flat-square)](https://david-dm.org/JamieMason/syncpack)
[![Code Climate](https://img.shields.io/codeclimate/github/JamieMason/syncpack.svg?style=flat-square)](https://codeclimate.com/github/JamieMason/syncpack)
[![Join the chat at https://gitter.im/JamieMason/syncpack](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/JamieMason/syncpack)
[![Analytics](https://ga-beacon.appspot.com/UA-45466560-5/syncpack?flat&useReferer)](https://github.com/igrigorik/ga-beacon)
<br>
[![Donate via Gratipay](https://img.shields.io/gratipay/user/JamieMason.svg)](https://gratipay.com/~JamieMason/)
[![Follow JamieMason on GitHub](https://img.shields.io/github/followers/JamieMason.svg?style=social&label=Follow)](https://github.com/JamieMason)
[![Follow fold_left on Twitter](https://img.shields.io/twitter/follow/fold_left.svg?style=social&label=Follow)](https://twitter.com/fold_left)

Synchronises the contents of multiple `package.json` files, such as `packages/*/package.json` in
[Lerna](https://lernajs.io) Monorepos.

## Contents

* [Installation](#installation)
* [Usage](#usage)
  * [`sync-versions`](#sync-versions)
  * [`copy-values`](#copy-values)

## Installation

```
npm install --global syncpack
```

## Usage

```
Usage: syncpack [options] [command]

Options:

  -V, --version  output the version number
  -h, --help     output usage information

Commands:

  sync-versions          synchronise dependency versions between packages
  copy-values <keys...>  copy values from eg. ./package.json to ./packages/*/package.json
  help [cmd]             display help for [cmd]
```

### `sync-versions`

```
Usage: syncpack sync-versions [options]

Options:

  -p, --packages <glob>  location of packages. defaults to ./packages/*/package.json
  -h, --help             output usage information
```

Imagine the packages `guybrush`, `herman`, and `elaine` all have `react` as a dependency, but
versions `'15.4.0'`, `'15.5.4'`, and `'15.6.1'` respectively.

```
/Users/foldleft/Dev/monorepo/packages/
├── guybrush
│   └── package.json
├── herman
│   └── package.json
└── elaine
    └── package.json
```

To update each `package.json` to use version `'15.6.1'` of `react` in `dependencies`,
`devDependencies`, and `peerDependencies` (as needed) you can run

```
syncpack sync-versions
```

### `copy-values`

```
Usage: syncpack copy-values [options] <keys...>

Options:

  -p, --packages <glob>  location of packages. defaults to ./packages/*/package.json
  -s, --source <glob>    location of source. defaults to ./package.json
  -h, --help             output usage information
```

Imagine the packages `carla` and `murray` were previously hosted at their own repositories, but are
now part of your new Monorepo.

```
/Users/foldleft/Dev/monorepo/packages/
├── carla
│   └── package.json
└── murray
    └── package.json
```

With the following contents

```
"bugs": "https://github.com/Scumm/carla/issues",
"homepage": "https://github.com/Scumm/carla#readme",
"repository": "Scumm/carla",
```

```
"bugs": "https://github.com/Scumm/murray/issues",
"homepage": "https://github.com/Scumm/murray#readme",
"repository": "Scumm/murray",
```

To copy these fields from your Monorepo's `package.json` to each of its packages, you can run

```
syncpack copy-values bugs homepage repository
```

to copy the value of those properties, leaving them like so

```
"bugs": "https://github.com/Scumm/monorepo/issues",
"homepage": "https://github.com/Scumm/monorepo#readme",
"repository": "Scumm/monorepo",
```

to copy deeply nested values, pass the path to the key as follows

```
syncpack copy-values scripts.test
```
