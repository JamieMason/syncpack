# syncpack

[![NPM version](http://img.shields.io/npm/v/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![NPM downloads](http://img.shields.io/npm/dm/syncpack.svg?style=flat-square)](https://www.npmjs.com/package/syncpack)
[![Dependency Status](http://img.shields.io/david/JamieMason/syncpack.svg?style=flat-square)](https://david-dm.org/JamieMason/syncpack)
[![Gitter Chat for syncpack](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/JamieMason/syncpack)
[![Donate via PayPal](https://img.shields.io/badge/donate-paypal-blue.svg)](https://www.paypal.me/foldleft)
[![Analytics](https://ga-beacon.appspot.com/UA-45466560-5/syncpack?flat&useReferer)](https://github.com/igrigorik/ga-beacon)
[![Follow JamieMason on GitHub](https://img.shields.io/github/followers/JamieMason.svg?style=social&label=Follow)](https://github.com/JamieMason)
[![Follow fold_left on Twitter](https://img.shields.io/twitter/follow/fold_left.svg?style=social&label=Follow)](https://twitter.com/fold_left)

Manage multiple `package.json` files, such as `packages/*/package.json` in [Lerna](https://lernajs.io) Monorepos.

## Installation

```
npm install --global syncpack
```

## Usage

```
Usage: syncpack [options] [command]

  Options:

    -V, --version      output the version number
    -h, --help         output usage information

  Commands:

    fix-mismatches     set dependencies used with different versions to the same version
    format             sort and shorten properties according to a convention
    list               list every dependency used in your packages
    list-mismatches    list every dependency used with different versions in your packages
    set-semver-ranges  set semver ranges to the given format
    help [cmd]         display help for [cmd]
```
