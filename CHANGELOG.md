## [5.7.11](https://github.com/JamieMason/syncpack/compare/5.6.10...5.7.11) (2021-01-29)


### Bug Fixes

* **npm:** update dependencies ([5531da6](https://github.com/JamieMason/syncpack/commit/5531da60bc1cfb0fe3c5ca8e904d0a9e55d3d4b5))


### Features

* **fix-mismatches:** use local package version when available ([640cb7f](https://github.com/JamieMason/syncpack/commit/640cb7faf18b33fd491e68f66d3cf599845c9265)), closes [#47](https://github.com/JamieMason/syncpack/issues/47)



## [5.6.10](https://github.com/JamieMason/syncpack/compare/5.6.7...5.6.10) (2020-09-17)


### Bug Fixes

* **cli:** use defaults when source is empty array ([c2f6199](https://github.com/JamieMason/syncpack/commit/c2f61998add60ed5d52af1c3518d1f737cf75c80))
* **core:** support multiple version groups ([bfd12b4](https://github.com/JamieMason/syncpack/commit/bfd12b4f3a6693ac1b4580621b12995d2b04eee7)), closes [#43](https://github.com/JamieMason/syncpack/issues/43)
* **list:** display mismatches from version groups ([43ba18d](https://github.com/JamieMason/syncpack/commit/43ba18dff1aa7c749724b992b6eef17a227f5445))



## [5.6.7](https://github.com/JamieMason/syncpack/compare/5.5.6...5.6.7) (2020-08-30)


### Bug Fixes

* **npm:** update dependencies ([2e3ea3b](https://github.com/JamieMason/syncpack/commit/2e3ea3b0f6de8a97a390305a998053550183cc27))


### Features

* **core:** support granular versioning rules ([2197f90](https://github.com/JamieMason/syncpack/commit/2197f90608c119a04ddde6255e729fa1ec5c49ec)), closes [#41](https://github.com/JamieMason/syncpack/issues/41)



## [5.5.6](https://github.com/JamieMason/syncpack/compare/5.2.5...5.5.6) (2020-08-23)


### Bug Fixes

* **core:** ignore link: versions rather than throw ([7a48366](https://github.com/JamieMason/syncpack/commit/7a483666e64a046be9984bf4146ac8566b3f5920)), closes [#38](https://github.com/JamieMason/syncpack/issues/38)


### Features

* **core:** expose format configuration ([4f74d9a](https://github.com/JamieMason/syncpack/commit/4f74d9a0b9a92428278f66327630e5b0e9dc5add)), closes [#30](https://github.com/JamieMason/syncpack/issues/30)
* **core:** sort resolutions field a-z ([f76a127](https://github.com/JamieMason/syncpack/commit/f76a1278b45ec3b00b2658b5da327d0a480ff12d)), closes [#34](https://github.com/JamieMason/syncpack/issues/34)
* **core:** support yarn workspaces config as object ([34eceaf](https://github.com/JamieMason/syncpack/commit/34eceaffae143fdbc9729495ea693172c2944351)), closes [#33](https://github.com/JamieMason/syncpack/issues/33)



## [5.2.5](https://github.com/JamieMason/syncpack/compare/5.1.4...5.2.5) (2020-08-22)


### Bug Fixes

* **npm:** update dependencies ([19ad510](https://github.com/JamieMason/syncpack/commit/19ad510d09040e1aa098e16d6831836da3c9c12f))


### Features

* **core:** add support for config files ([cfd5df3](https://github.com/JamieMason/syncpack/commit/cfd5df35134de068eaf26bdb2cfaa1890c6c3545))



## [5.1.4](https://github.com/JamieMason/syncpack/compare/5.0.3...5.1.4) (2020-08-02)


### Bug Fixes

* **npm:** update dependencies ([f2cac6a](https://github.com/JamieMason/syncpack/commit/f2cac6a05eaf9f5a7736267a797cf75476292757))


### Features

* **core:** add support for pnpm workspaces ([a6112ec](https://github.com/JamieMason/syncpack/commit/a6112ec786fd26699a3734707218cda38baf9f0e)), closes [#42](https://github.com/JamieMason/syncpack/issues/42)



## [5.0.3](https://github.com/JamieMason/syncpack/compare/5.0.1...5.0.3) (2020-06-19)


### Bug Fixes

* **format:** leave sort order of "files" array unchanged ([1bd584f](https://github.com/JamieMason/syncpack/commit/1bd584f67054d4a37b91b1e5f285dbe9b53b4489)), closes [#35](https://github.com/JamieMason/syncpack/issues/35)
* **npm:** update dependencies ([9e0bd7e](https://github.com/JamieMason/syncpack/commit/9e0bd7ea257b3dcc425f306d4fcae195f6d0d126))



## [5.0.1](https://github.com/JamieMason/syncpack/compare/4.5.5...5.0.1) (2020-02-16)


### Bug Fixes

* **core:** include root package.json when reading yarn & lerna config ([a7875cb](https://github.com/JamieMason/syncpack/commit/a7875cb08e0f8382163e8c9e8a4d3e6772b4c160))
* **npm:** update dependencies ([5fdcc7b](https://github.com/JamieMason/syncpack/commit/5fdcc7bd112533f891b31dfcf0be79b54989b8d7))


### BREAKING CHANGES

* **npm:** engines.node has been increased to >=10 because
semver@7.1.1 is a hard dependency of syncpack and
requires node >=10



## [4.5.5](https://github.com/JamieMason/syncpack/compare/4.5.4...4.5.5) (2020-01-19)


### Bug Fixes

* **npm:** update dependencies ([1776b5f](https://github.com/JamieMason/syncpack/commit/1776b5fdbbc79315dd5ab8700f25daeb3ec46b05))



## [4.5.4](https://github.com/JamieMason/syncpack/compare/4.5.3...4.5.4) (2019-07-16)


### Bug Fixes

* **npm:** update dependencies ([e07cc44](https://github.com/JamieMason/syncpack/commit/e07cc44add6e9ae2d7775496f5585ac6f46e58e9)), closes [#28](https://github.com/JamieMason/syncpack/issues/28)



## [4.5.3](https://github.com/JamieMason/syncpack/compare/4.5.2...4.5.3) (2019-06-17)


### Bug Fixes

* **npm:** update dependencies ([10834a9](https://github.com/JamieMason/syncpack/commit/10834a905812aadd4e13b2420bf3dc1549939dab))



## [4.5.2](https://github.com/JamieMason/syncpack/compare/4.4.2...4.5.2) (2019-05-14)


### Features

* **cli:** add support for yarn workspaces ([a5a45dd](https://github.com/JamieMason/syncpack/commit/a5a45ddc937020b54c33a13d554fb871fee50e05)), closes [#20](https://github.com/JamieMason/syncpack/issues/20) [#22](https://github.com/JamieMason/syncpack/issues/22)



## [4.4.2](https://github.com/JamieMason/syncpack/compare/4.4.1...4.4.2) (2019-05-06)


### Bug Fixes

* **filter:** --filter is a string, not a boolean ([5587f2b](https://github.com/JamieMason/syncpack/commit/5587f2bdaef0dcc50022c9ae9e98b1c34ce9e164))



## [4.4.1](https://github.com/JamieMason/syncpack/compare/4.3.1...4.4.1) (2019-04-29)


### Features

* **options:** add dependency filter regex ([bfb1f1d](https://github.com/JamieMason/syncpack/commit/bfb1f1dde0ee3cbd11a0ef5cef80a0f53b28083a)), closes [#18](https://github.com/JamieMason/syncpack/issues/18)



## [4.3.1](https://github.com/JamieMason/syncpack/compare/4.0.1...4.3.1) (2019-02-03)


### Features

* **fix-mismatches:** output which files are (un)changed ([a79b078](https://github.com/JamieMason/syncpack/commit/a79b078d7527a27a6e1343dab06e901f0c0a0530))
* **format:** output which files are (un)changed ([3a08a7a](https://github.com/JamieMason/syncpack/commit/3a08a7a5a0bebfdf2d10503ce2cd1920ef94367e))
* **list:** sort output alphabetically ([f61bde4](https://github.com/JamieMason/syncpack/commit/f61bde46a08550daf96cca596cbd1e00c13c7564))



## [4.0.1](https://github.com/JamieMason/syncpack/compare/4.0.0...4.0.1) (2019-01-14)


### Bug Fixes

* **ci:** cannot read property concat of undefined ([46a45e2](https://github.com/JamieMason/syncpack/commit/46a45e26b51b9f81076148ff7483b19cd34aef73)), closes [#16](https://github.com/JamieMason/syncpack/issues/16)



# [4.0.0](https://github.com/JamieMason/syncpack/compare/3.5.2...4.0.0) (2019-01-11)


### Bug Fixes

* **node:** support Node.js 8.x or newer ([c71009e](https://github.com/JamieMason/syncpack/commit/c71009e1507cd66c735112a0ae685cd3e51ab2fe))
* **npm:** update dependencies ([23b02e3](https://github.com/JamieMason/syncpack/commit/23b02e3d72e51e8b069a336357e6cddcdc4979c1)), closes [#15](https://github.com/JamieMason/syncpack/issues/15)


### BREAKING CHANGES

* **node:** Support Node.js 8.x or newer, Transitive Dependency ip-regex@3.0.0
supports node ">=8".



## [3.5.2](https://github.com/JamieMason/syncpack/compare/3.5.0...3.5.2) (2019-01-07)


### Bug Fixes

* **core:** improve handling of non-semver versions ([9e1176a](https://github.com/JamieMason/syncpack/commit/9e1176a3495ea97648c61ab5869a12c3ff539c5f)), closes [#14](https://github.com/JamieMason/syncpack/issues/14)
* **npm:** update dependencies ([09d9f04](https://github.com/JamieMason/syncpack/commit/09d9f04480252edd0fd3b6af3cd8dce36c66d96b))



# [3.5.0](https://github.com/JamieMason/syncpack/compare/3.4.0...3.5.0) (2018-10-29)


### Features

* **cli:** improve --help output and examples ([dfe6274](https://github.com/JamieMason/syncpack/commit/dfe6274c50d6ba3ea3ec419cabd1ccf0bb73f8fb))



# [3.4.0](https://github.com/JamieMason/syncpack/compare/3.3.0...3.4.0) (2018-10-28)


### Features

* **cli:** read sources from lerna.json if present ([77b90eb](https://github.com/JamieMason/syncpack/commit/77b90eb3d656c50ff7b9d1317dc2cdad469b15a5)), closes [#11](https://github.com/JamieMason/syncpack/issues/11)



# [3.3.0](https://github.com/JamieMason/syncpack/compare/3.0.0...3.3.0) (2018-10-28)


### Features

* **cli:** specify dependency types as options ([ec5ef6b](https://github.com/JamieMason/syncpack/commit/ec5ef6b76f3c2fa0fba0f3a364b734f554d32c8a)), closes [#10](https://github.com/JamieMason/syncpack/issues/10)
* **cli:** specify indentation as option ([8b408bd](https://github.com/JamieMason/syncpack/commit/8b408bd14768fe7b3a2fd5cbb06233ba3b9707b3)), closes [#12](https://github.com/JamieMason/syncpack/issues/12)
* **format:** sort contributors alphabetically ([935ffcf](https://github.com/JamieMason/syncpack/commit/935ffcf307d0adabe06c04ff1e2b258277f060be))


### Performance Improvements

* **npm:** move [@types](https://github.com/types) to devDependencies ([ad5951c](https://github.com/JamieMason/syncpack/commit/ad5951ceba183761b0b73355a508111e7eb02508)), closes [#13](https://github.com/JamieMason/syncpack/issues/13)



# [3.0.0](https://github.com/JamieMason/syncpack/compare/2.0.1...3.0.0) (2018-08-25)


### Features

* **bin:** override package locations using repeatable --source options ([5dbcfd4](https://github.com/JamieMason/syncpack/commit/5dbcfd4915cf286cba0e665e554c319d717f6651))
* **list-mismatches:** return exit code on finding mismatches ([06958c6](https://github.com/JamieMason/syncpack/commit/06958c6446646c108fc1dc4e07c714cd08bf58fc))


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

* **core:** ensure pattern overrides are read ([7513ba5](https://github.com/JamieMason/syncpack/commit/7513ba5fa644bf445efbdca22d4797b4a973b56f))



# [2.0.0](https://github.com/JamieMason/syncpack/compare/1.3.2...2.0.0) (2018-04-29)


### Features

* **core:** support multiple glob patterns ([a2b5af0](https://github.com/JamieMason/syncpack/commit/a2b5af017a2152fb40d0522501db64ef739fe5f9)), closes [#5](https://github.com/JamieMason/syncpack/issues/5) [#6](https://github.com/JamieMason/syncpack/issues/6)


### BREAKING CHANGES

* **core:** --packages option replaced with variadic arguments



## [1.3.2](https://github.com/JamieMason/syncpack/compare/1.2.2...1.3.2) (2018-04-28)


### Features

* **core:** add set-semver-ranges command ([4d206b9](https://github.com/JamieMason/syncpack/commit/4d206b9d00c7cf5a9f8ee52b84d7e8b9487fd01a))



## [1.2.2](https://github.com/JamieMason/syncpack/compare/1.0.2...1.2.2) (2018-02-10)


### Features

* **core:** add format command ([bae1133](https://github.com/JamieMason/syncpack/commit/bae11337f1c68c87299ee3e33120d3dc0fa8643f))
* **core:** output current version ([e53cd99](https://github.com/JamieMason/syncpack/commit/e53cd99e989b1ea383530bcde78d105b39103a8c))



## [1.0.2](https://github.com/JamieMason/syncpack/compare/1.0.1...1.0.2) (2018-02-02)



## [1.0.1](https://github.com/JamieMason/syncpack/compare/1.0.0...1.0.1) (2018-02-02)


### Bug Fixes

* **core:** correct paths to binaries ([5682cd6](https://github.com/JamieMason/syncpack/commit/5682cd65b8559e9f47c9fe63b6294aebb73ba896))



# [1.0.0](https://github.com/JamieMason/syncpack/compare/0.3.1...1.0.0) (2018-02-02)


### Bug Fixes

* **core:** correctly check a file is package.json ([d1da609](https://github.com/JamieMason/syncpack/commit/d1da6096c3b7c6b01a05c112ffc1251ec4ba700d))
* **core:** handle missing dependency maps ([372aa68](https://github.com/JamieMason/syncpack/commit/372aa6877f47df1118c45931391c8b87ca851413))
* **core:** handle semver ranges containing 1.x.x ([a0f8f56](https://github.com/JamieMason/syncpack/commit/a0f8f5650f3855361914fc6f8303035dc3abfb8d))


### Features

* **core:** add fix-mismatches command ([4793f1f](https://github.com/JamieMason/syncpack/commit/4793f1fc6b67cfa1f87f73188944f8dd8d196bc0))
* **core:** add list command ([3b29176](https://github.com/JamieMason/syncpack/commit/3b291760f4cba611acc3f75034679303c55bf1a7))
* **core:** add list-mismatches command ([735ad2b](https://github.com/JamieMason/syncpack/commit/735ad2b2a1347b99a3f758b0c797b2fb7a3fc4c3))
* **core:** update command line API ([de8dcb2](https://github.com/JamieMason/syncpack/commit/de8dcb2b0dbe7bb63c91aeb05e8422696b0bd178))


### BREAKING CHANGES

* **core:** The previous commands have been replaced.



## [0.3.1](https://github.com/JamieMason/syncpack/compare/0.3.0...0.3.1) (2017-08-23)


### Bug Fixes

* **copy-values:** write results to disk ([a641de4](https://github.com/JamieMason/syncpack/commit/a641de41faaf6851cf9177ff87acd0d3e16494fb))



# [0.3.0](https://github.com/JamieMason/syncpack/compare/0.2.1...0.3.0) (2017-08-22)


### Features

* **cli:** add copy-values command ([b51a2c9](https://github.com/JamieMason/syncpack/commit/b51a2c96e133a1b5020577cf3c6bef31e79de850))



## [0.2.1](https://github.com/JamieMason/syncpack/compare/0.2.0...0.2.1) (2017-08-20)


### Bug Fixes

* **core:** update dependencies, fix lint warnings ([a65eef7](https://github.com/JamieMason/syncpack/commit/a65eef765d868a27913e173543dcbda43a2202a5))



# [0.2.0](https://github.com/JamieMason/syncpack/compare/0.1.0...0.2.0) (2017-08-20)


### Features

* **sync:** synchronise versions across multiple package.json ([7d5848a](https://github.com/JamieMason/syncpack/commit/7d5848a0edbe0c0a312be323cc8d9a4a8ed0ea30))



# [0.1.0](https://github.com/JamieMason/syncpack/compare/f6dada7aae149b7d0299206308347c8497e249d0...0.1.0) (2017-08-18)


### Features

* **cli:** create scaffold cli ([f6dada7](https://github.com/JamieMason/syncpack/commit/f6dada7aae149b7d0299206308347c8497e249d0))



