# syncpack

[![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![Build Status](http://img.shields.io/travis/JamieMason/syncpack/master.svg?style=flat-square)](https://travis-ci.org/JamieMason/syncpack)
[![Dependency Status](http://img.shields.io/david/JamieMason/syncpack.svg?style=flat-square)](https://david-dm.org/JamieMason/syncpack)
[![Join the chat at https://gitter.im/JamieMason/syncpack](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/JamieMason/syncpack)
[![Analytics](https://ga-beacon.appspot.com/UA-45466560-5/syncpack?flat&useReferer)](https://github.com/igrigorik/ga-beacon)

Synchronises the versions of dependencies used across multiple `package.json` files, such as
`packages/*/package.json` in [Lerna](https://lernajs.io) Monorepos.

## Overview

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

Running `syncpack` will update each `package.json` to use version `'15.6.1'` of `react` in
`dependencies`, `devDependencies`, and `peerDependencies` as needed.

## Installation

```
npm install --global syncpack
```

## Usage

### Command Line

```
Usage: syncpack [options] [pattern]

Options:

  -h, --help     output usage information
  -V, --version  output the version number
```

The default pattern of `'./packages/*/package.json'` can be overridden as follows
`syncpack './**/package.json'`
