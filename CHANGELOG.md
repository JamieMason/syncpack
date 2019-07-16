## [4.5.4](https://github.com/JamieMason/syncpack/compare/4.5.3...4.5.4) (2019-07-16)


### Bug Fixes

* **npm:** update dependencies ([e07cc44](https://github.com/JamieMason/syncpack/commit/e07cc44)), closes [#28](https://github.com/JamieMason/syncpack/issues/28)



## [4.5.3](https://github.com/JamieMason/syncpack/compare/4.5.2...4.5.3) (2019-06-17)


### Bug Fixes

* **npm:** update dependencies ([10834a9](https://github.com/JamieMason/syncpack/commit/10834a9))



## [4.5.2](https://github.com/JamieMason/syncpack/compare/4.4.2...4.5.2) (2019-05-14)


### Features

* **cli:** add support for yarn workspaces ([a5a45dd](https://github.com/JamieMason/syncpack/commit/a5a45dd)), closes [#20](https://github.com/JamieMason/syncpack/issues/20) [#22](https://github.com/JamieMason/syncpack/issues/22)



## [4.4.2](https://github.com/JamieMason/syncpack/compare/4.4.1...4.4.2) (2019-05-06)


### Bug Fixes

* **filter:** --filter is a string, not a boolean ([5587f2b](https://github.com/JamieMason/syncpack/commit/5587f2b))



## [4.4.1](https://github.com/JamieMason/syncpack/compare/4.3.1...4.4.1) (2019-04-29)


### Features

* **options:** add dependency filter regex ([bfb1f1d](https://github.com/JamieMason/syncpack/commit/bfb1f1d)), closes [#18](https://github.com/JamieMason/syncpack/issues/18)



## [4.3.1](https://github.com/JamieMason/syncpack/compare/4.0.1...4.3.1) (2019-02-03)


### Features

* **fix-mismatches:** output which files are (un)changed ([a79b078](https://github.com/JamieMason/syncpack/commit/a79b078))
* **format:** output which files are (un)changed ([3a08a7a](https://github.com/JamieMason/syncpack/commit/3a08a7a))
* **list:** sort output alphabetically ([f61bde4](https://github.com/JamieMason/syncpack/commit/f61bde4))



## [4.0.1](https://github.com/JamieMason/syncpack/compare/4.0.0...4.0.1) (2019-01-14)


### Bug Fixes

* **ci:** cannot read property concat of undefined ([46a45e2](https://github.com/JamieMason/syncpack/commit/46a45e2)), closes [#16](https://github.com/JamieMason/syncpack/issues/16)



# [4.0.0](https://github.com/JamieMason/syncpack/compare/3.5.2...4.0.0) (2019-01-11)


### Bug Fixes

* **node:** support Node.js 8.x or newer ([c71009e](https://github.com/JamieMason/syncpack/commit/c71009e))
* **npm:** update dependencies ([23b02e3](https://github.com/JamieMason/syncpack/commit/23b02e3)), closes [#15](https://github.com/JamieMason/syncpack/issues/15)


### BREAKING CHANGES

* **node:** Support Node.js 8.x or newer, Transitive Dependency ip-regex@3.0.0
supports node ">=8".



## [3.5.2](https://github.com/JamieMason/syncpack/compare/3.5.0...3.5.2) (2019-01-07)


### Bug Fixes

* **core:** improve handling of non-semver versions ([9e1176a](https://github.com/JamieMason/syncpack/commit/9e1176a)), closes [#14](https://github.com/JamieMason/syncpack/issues/14)
* **npm:** update dependencies ([09d9f04](https://github.com/JamieMason/syncpack/commit/09d9f04))



# [3.5.0](https://github.com/JamieMason/syncpack/compare/3.4.0...3.5.0) (2018-10-29)


### Features

* **cli:** improve --help output and examples ([dfe6274](https://github.com/JamieMason/syncpack/commit/dfe6274))



# [3.4.0](https://github.com/JamieMason/syncpack/compare/3.3.0...3.4.0) (2018-10-28)


### Features

* **cli:** read sources from lerna.json if present ([77b90eb](https://github.com/JamieMason/syncpack/commit/77b90eb)), closes [#11](https://github.com/JamieMason/syncpack/issues/11)



# [3.3.0](https://github.com/JamieMason/syncpack/compare/3.0.0...3.3.0) (2018-10-28)


### Features

* **cli:** specify dependency types as options ([ec5ef6b](https://github.com/JamieMason/syncpack/commit/ec5ef6b)), closes [#10](https://github.com/JamieMason/syncpack/issues/10)
* **cli:** specify indentation as option ([8b408bd](https://github.com/JamieMason/syncpack/commit/8b408bd)), closes [#12](https://github.com/JamieMason/syncpack/issues/12)
* **format:** sort contributors alphabetically ([935ffcf](https://github.com/JamieMason/syncpack/commit/935ffcf))


### Performance Improvements

* **npm:** move [@types](https://github.com/types) to devDependencies ([ad5951c](https://github.com/JamieMason/syncpack/commit/ad5951c)), closes [#13](https://github.com/JamieMason/syncpack/issues/13)



# [3.0.0](https://github.com/JamieMason/syncpack/compare/2.0.1...3.0.0) (2018-08-25)


### Features

* **bin:** override package locations using repeatable --source options ([5dbcfd4](https://github.com/JamieMason/syncpack/commit/5dbcfd4))
* **list-mismatches:** return exit code on finding mismatches ([06958c6](https://github.com/JamieMason/syncpack/commit/06958c6))


### BREAKING CHANGES

* **bin:** Previously the location of package.json files could be overridden like so:

```
syncpack list './package.json' './packages/*/package.json'
```

This is now done using a repeatable `--source` option:

```
syncpack list --source './package.json' --source './packages/*/package.json'
```

This change is to make way for new commands which will also require an
overridable `--target` option.



## [2.0.1](https://github.com/JamieMason/syncpack/compare/2.0.0...2.0.1) (2018-04-29)


### Bug Fixes

* **core:** ensure pattern overrides are read ([7513ba5](https://github.com/JamieMason/syncpack/commit/7513ba5))



# [2.0.0](https://github.com/JamieMason/syncpack/compare/1.3.2...2.0.0) (2018-04-29)


### Features

* **core:** support multiple glob patterns ([a2b5af0](https://github.com/JamieMason/syncpack/commit/a2b5af0)), closes [#5](https://github.com/JamieMason/syncpack/issues/5) [#6](https://github.com/JamieMason/syncpack/issues/6)


### BREAKING CHANGES

* **core:** --packages option replaced with variadic arguments



## [1.3.2](https://github.com/JamieMason/syncpack/compare/1.2.2...1.3.2) (2018-04-28)


### Features

* **core:** add set-semver-ranges command ([4d206b9](https://github.com/JamieMason/syncpack/commit/4d206b9))



## [1.2.2](https://github.com/JamieMason/syncpack/compare/1.0.2...1.2.2) (2018-02-10)


### Features

* **core:** add format command ([bae1133](https://github.com/JamieMason/syncpack/commit/bae1133))
* **core:** output current version ([e53cd99](https://github.com/JamieMason/syncpack/commit/e53cd99))



## [1.0.2](https://github.com/JamieMason/syncpack/compare/1.0.1...1.0.2) (2018-02-02)



## [1.0.1](https://github.com/JamieMason/syncpack/compare/1.0.0...1.0.1) (2018-02-02)


### Bug Fixes

* **core:** correct paths to binaries ([5682cd6](https://github.com/JamieMason/syncpack/commit/5682cd6))



# [1.0.0](https://github.com/JamieMason/syncpack/compare/0.3.1...1.0.0) (2018-02-02)


### Bug Fixes

* **core:** correctly check a file is package.json ([d1da609](https://github.com/JamieMason/syncpack/commit/d1da609))
* **core:** handle missing dependency maps ([372aa68](https://github.com/JamieMason/syncpack/commit/372aa68))
* **core:** handle semver ranges containing 1.x.x ([a0f8f56](https://github.com/JamieMason/syncpack/commit/a0f8f56))


### Features

* **core:** add fix-mismatches command ([4793f1f](https://github.com/JamieMason/syncpack/commit/4793f1f))
* **core:** add list command ([3b29176](https://github.com/JamieMason/syncpack/commit/3b29176))
* **core:** add list-mismatches command ([735ad2b](https://github.com/JamieMason/syncpack/commit/735ad2b))
* **core:** update command line API ([de8dcb2](https://github.com/JamieMason/syncpack/commit/de8dcb2))


### BREAKING CHANGES

* **core:** The previous commands have been replaced.



## [0.3.1](https://github.com/JamieMason/syncpack/compare/0.3.0...0.3.1) (2017-08-23)


### Bug Fixes

* **copy-values:** write results to disk ([a641de4](https://github.com/JamieMason/syncpack/commit/a641de4))



# [0.3.0](https://github.com/JamieMason/syncpack/compare/0.2.1...0.3.0) (2017-08-22)


### Features

* **cli:** add copy-values command ([b51a2c9](https://github.com/JamieMason/syncpack/commit/b51a2c9))



## [0.2.1](https://github.com/JamieMason/syncpack/compare/0.2.0...0.2.1) (2017-08-20)


### Bug Fixes

* **core:** update dependencies, fix lint warnings ([a65eef7](https://github.com/JamieMason/syncpack/commit/a65eef7))



# [0.2.0](https://github.com/JamieMason/syncpack/compare/0.1.0...0.2.0) (2017-08-20)


### Features

* **sync:** synchronise versions across multiple package.json ([7d5848a](https://github.com/JamieMason/syncpack/commit/7d5848a))



# [0.1.0](https://github.com/JamieMason/syncpack/compare/f6dada7...0.1.0) (2017-08-18)


### Features

* **cli:** create scaffold cli ([f6dada7](https://github.com/JamieMason/syncpack/commit/f6dada7))



