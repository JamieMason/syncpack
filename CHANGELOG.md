## [9.8.6](https://github.com/JamieMason/syncpack/compare/9.8.4...9.8.6) (2023-04-23)


### Bug Fixes

* **config:** prevent default source overriding rcfile ([1d6a4ba](https://github.com/JamieMason/syncpack/commit/1d6a4bab953ceee90461d8f85ece44bd384d9cd7)), closes [#123](https://github.com/JamieMason/syncpack/issues/123)
* **npm:** update minor dependencies ([91f4967](https://github.com/JamieMason/syncpack/commit/91f496767d834b7372565a094599ece935bb346d))



## [9.8.4](https://github.com/JamieMason/syncpack/compare/9.7.4...9.8.4) (2023-02-21)


### Features

* **semver:** support resolving with lowest version ([a17e423](https://github.com/JamieMason/syncpack/commit/a17e4234730b433777f320bb5ca35e8c46545832)), closes [#110](https://github.com/JamieMason/syncpack/issues/110)



## [9.7.4](https://github.com/JamieMason/syncpack/compare/9.3.2...9.7.4) (2023-02-19)


### Bug Fixes

* **indent:** use value from config file ([aa31244](https://github.com/JamieMason/syncpack/commit/aa312448156c6814213502e5f98c2671d7a166a8))
* **npm:** update dependencies ([558d177](https://github.com/JamieMason/syncpack/commit/558d177026ca89606b534f3ce37958b80faa7b1a))


### Features

* **engines:** increase node from 10 to 14 ([603f058](https://github.com/JamieMason/syncpack/commit/603f0587a62d6d452c01bc24ec23827bc2fb582b))
* **groups:** handle long and multi-line labels ([ecc58ff](https://github.com/JamieMason/syncpack/commit/ecc58fff645f9639f003934c109bc5d17af7b3d6))
* **semver:** recognise ^6, >=5 etc as valid ([be637f0](https://github.com/JamieMason/syncpack/commit/be637f0349018b2b3d5f204613a4af187c8f7aa0)), closes [#122](https://github.com/JamieMason/syncpack/issues/122)
* **versionGroups:** add optional snapTo property ([fd0edb6](https://github.com/JamieMason/syncpack/commit/fd0edb6a25ec14aefdaf72f796bbe5d3c22d3692)), closes [#87](https://github.com/JamieMason/syncpack/issues/87)


### Performance Improvements

* **imports:** skip barrel files where possible ([1ee2776](https://github.com/JamieMason/syncpack/commit/1ee2776b86c3ded344e9a34dd009efd5a879c48d))



## [9.3.2](https://github.com/JamieMason/syncpack/compare/9.1.2...9.3.2) (2023-02-17)


### Features

* **groups:** add optional label to semver/version groups ([ff466af](https://github.com/JamieMason/syncpack/commit/ff466af88850bd8ddd1d80d0a272bae85b39cbe1)), closes [#118](https://github.com/JamieMason/syncpack/issues/118)
* **groups:** output groups in order they're defined ([88950f1](https://github.com/JamieMason/syncpack/commit/88950f1208405d3c25a743e03fa59053a1c7662a)), closes [#120](https://github.com/JamieMason/syncpack/issues/120)



## [9.1.2](https://github.com/JamieMason/syncpack/compare/9.0.2...9.1.2) (2023-02-16)


### Features

* **semver:** mention unsupported versions in output ([69edcaf](https://github.com/JamieMason/syncpack/commit/69edcaf9ad2a64c58b5e6dfddfef3bd1e937327e)), closes [#121](https://github.com/JamieMason/syncpack/issues/121) [#119](https://github.com/JamieMason/syncpack/issues/119)


### Reverts

* **fix-mismatches:** don't remove nested empty objects ([393d004](https://github.com/JamieMason/syncpack/commit/393d004a70e02d44d73c921b80ad5db1454c4ddd)), closes [#117](https://github.com/JamieMason/syncpack/issues/117)



## [9.0.2](https://github.com/JamieMason/syncpack/compare/9.0.0...9.0.2) (2023-02-15)


### Bug Fixes

* **options:** fix --source regression in 9.0.0 ([379409f](https://github.com/JamieMason/syncpack/commit/379409f892e98532ed52ec974da3c4ec73618d63)), closes [#116](https://github.com/JamieMason/syncpack/issues/116)
* **semver:** fix false positive for workspace mismatches ([4f696c5](https://github.com/JamieMason/syncpack/commit/4f696c5de4b7a4106b05b4c2de1e76f607ba5ca2))



# [9.0.0](https://github.com/JamieMason/syncpack/compare/8.5.14...9.0.0) (2023-02-14)


### Features

* **custom:** support custom version locations ([2cd34fd](https://github.com/JamieMason/syncpack/commit/2cd34fd1f41e949cedd28b901a123906d8bc5d08)), closes [#112](https://github.com/JamieMason/syncpack/issues/112) [#113](https://github.com/JamieMason/syncpack/issues/113)
* **fix-mismatches:** remove any empty objects ([a279e56](https://github.com/JamieMason/syncpack/commit/a279e56dfaf8ba11bd507a89315b2a5b038b027b))


### BREAKING CHANGES

* **custom:** 1. The following options were replaced in syncpack@9.0.0:

  -p, --prod              include dependencies
  -d, --dev               include devDependencies
  -P, --peer              include peerDependencies
  -R, --resolutions       include resolutions (yarn)
  -o, --overrides         include overrides (npm)
  -O, --pnpmOverrides     include overrides (pnpm)
  -w, --workspace         include locally developed package versions

  Instead use the new --types option like so:

    --types dev,prod,peer

2. In .syncpackrc, the following options were replaced:

  "dev": true,
  "overrides": true,
  "peer": true,
  "pnpmOverrides": true,
  "prod": true,
  "resolutions": true,
  "workspace": true,

  Instead use the new dependencyTypes array like so:

    "dependencyTypes": ["dev", "prod", "peer"]



## [8.5.14](https://github.com/JamieMason/syncpack/compare/8.4.11...8.5.14) (2023-02-07)


### Bug Fixes

* **npm:** update dependencies ([f2c0322](https://github.com/JamieMason/syncpack/commit/f2c0322ccb1724b566bf818c0a3b03ac3ed9a27e))
* **npm:** update dependencies ([3ec1361](https://github.com/JamieMason/syncpack/commit/3ec13613d1f7446d7798adabf33eb82101c51ee8))
* **output:** remove console.log in readConfigFileSync ([1c51366](https://github.com/JamieMason/syncpack/commit/1c51366a557f2a69e441c33f29d4927c9cdf88f5)), closes [#106](https://github.com/JamieMason/syncpack/issues/106)


### Features

* **cli:** tidy log output and make it more consistent ([cb58070](https://github.com/JamieMason/syncpack/commit/cb58070e646d80acec893e72154a96a25ae8d8ca))



## [8.4.11](https://github.com/JamieMason/syncpack/compare/8.4.10...8.4.11) (2022-12-01)


### Bug Fixes

* **semver-ranges:** fix regression in 8.4.10 {lint,set}-semver-ranges ([5bce1d8](https://github.com/JamieMason/syncpack/commit/5bce1d84e3018c08ec72459955de2631b3a3aabe))



## [8.4.10](https://github.com/JamieMason/syncpack/compare/8.3.9...8.4.10) (2022-11-23)


### Bug Fixes

* **lint-ranges:** remove empty groups from output ([fa6db49](https://github.com/JamieMason/syncpack/commit/fa6db490f63cc0e59889628c3f08cc1f92aa9fd5))


### Features

* **groups:** add support to ignore dependencies ([f96df8f](https://github.com/JamieMason/syncpack/commit/f96df8fe6faf80c28e02178f2dd23863290a1da6))



## [8.3.9](https://github.com/JamieMason/syncpack/compare/8.3.8...8.3.9) (2022-10-28)


### Bug Fixes

* **semver:** add support for setting '*' ([779772b](https://github.com/JamieMason/syncpack/commit/779772b039ad18aed84df560b13fa92b2a4ad741))



## [8.3.8](https://github.com/JamieMason/syncpack/compare/8.2.5...8.3.8) (2022-10-28)


### Bug Fixes

* **npm:** update dependencies ([6311999](https://github.com/JamieMason/syncpack/commit/63119995d5ab7e98b867edad28bf1655eb96bbdf))
* **pnpm:** fix regression affecting --pnpmOverrides ([6a782f9](https://github.com/JamieMason/syncpack/commit/6a782f95b558b2ae2800f429b4135f306e7abbfb)), closes [#94](https://github.com/JamieMason/syncpack/issues/94)
* **set-semver-ranges:** handle 'workspace' type ([2134658](https://github.com/JamieMason/syncpack/commit/213465882ee6182bbc0f3ef00108cc562b63deae)), closes [#84](https://github.com/JamieMason/syncpack/issues/84)


### Features

* **config:** include config.syncpack of package.json ([40dcdce](https://github.com/JamieMason/syncpack/commit/40dcdcef00d66db0866882e2ef0138972199eea3)), closes [#86](https://github.com/JamieMason/syncpack/issues/86)



## [8.2.5](https://github.com/JamieMason/syncpack/compare/8.2.4...8.2.5) (2022-10-28)


### Bug Fixes

* **format:** skip .repository when its .directory is present ([688bc0c](https://github.com/JamieMason/syncpack/commit/688bc0c29c09b4ea3e289f4584a6cc62adec936e)), closes [#91](https://github.com/JamieMason/syncpack/issues/91) [#93](https://github.com/JamieMason/syncpack/issues/93) [#100](https://github.com/JamieMason/syncpack/issues/100)



## [8.2.4](https://github.com/JamieMason/syncpack/compare/8.0.0...8.2.4) (2022-06-25)


### Bug Fixes

* **npm:** update dependencies ([1bdf0c7](https://github.com/JamieMason/syncpack/commit/1bdf0c756a369fc65987d04d5867a525e2e2248d))
* **npm:** update dependencies ([34c5d68](https://github.com/JamieMason/syncpack/commit/34c5d68194e7134305422f84a05d4774f812ef08))
* **output:** edit log output and colours ([3623c41](https://github.com/JamieMason/syncpack/commit/3623c41cb8bd16c4cd9186199140905847722b50))
* **workspace:** find missed mismatches against workspace versions ([25c1836](https://github.com/JamieMason/syncpack/commit/25c18363ebacbd5870e14d234e68cfb570ce913e)), closes [#66](https://github.com/JamieMason/syncpack/issues/66)


### Features

* **eol:** detect line endings when writing ([7e61f5c](https://github.com/JamieMason/syncpack/commit/7e61f5cd6edbaab48cfe89b356bee1ac6bb4d442)), closes [#76](https://github.com/JamieMason/syncpack/issues/76)
* **output:** better explain mismatch reasons ([45e0cf1](https://github.com/JamieMason/syncpack/commit/45e0cf141337f692909b72de3edaf52b86f1ec42)), closes [#65](https://github.com/JamieMason/syncpack/issues/65) [#77](https://github.com/JamieMason/syncpack/issues/77) [#79](https://github.com/JamieMason/syncpack/issues/79)



# [8.0.0](https://github.com/JamieMason/syncpack/compare/7.2.2...8.0.0) (2022-05-31)


### Features

* **pnpm:** add support for pnpm overrides ([2d1bf05](https://github.com/JamieMason/syncpack/commit/2d1bf059a239dff3af203104491a315b894b8796)), closes [#78](https://github.com/JamieMason/syncpack/issues/78)


### BREAKING CHANGES

* **pnpm:** The `--overrides` option delivered in 6.0.0 was originally intended to
support pnpm, but erroneously read from the `.overrides` property of
package.json files and not `.pnpm.overrides`.

However, npm now also has an `.overrides` property to support the same
functionality for users of npm.

From this release, the `--overrides` option of syncpack now refers to
npm overrides. Pnpm users should change to using the new
`--pnpmOverrides` option instead.



## [7.2.2](https://github.com/JamieMason/syncpack/compare/7.2.1...7.2.2) (2022-05-29)


### Bug Fixes

* **windows:** normalise file paths ([c5e87c2](https://github.com/JamieMason/syncpack/commit/c5e87c2efe47bb538701ec3d83b813c47eddab8b)), closes [#66](https://github.com/JamieMason/syncpack/issues/66)



## [7.2.1](https://github.com/JamieMason/syncpack/compare/7.1.0...7.2.1) (2022-05-02)


### Bug Fixes

* **npm:** update dependencies ([b9a9f5d](https://github.com/JamieMason/syncpack/commit/b9a9f5da2dee72e8aea7ccebca7607c80cd8391e))


### Features

* **versionGroups:** mark specific dependencies for removal ([e571775](https://github.com/JamieMason/syncpack/commit/e571775ebc120786b2742d0047dc8f79f7f8a539)), closes [#65](https://github.com/JamieMason/syncpack/issues/65)



# [7.1.0](https://github.com/JamieMason/syncpack/compare/7.0.0...7.1.0) (2022-05-01)


### Features

* **cli:** add --config to specify path to config file ([4b19a13](https://github.com/JamieMason/syncpack/commit/4b19a1375a7856bffb50c9fda84a3a8b6def877b)), closes [#71](https://github.com/JamieMason/syncpack/issues/71) [#72](https://github.com/JamieMason/syncpack/issues/72)



# [7.0.0](https://github.com/JamieMason/syncpack/compare/6.2.1...7.0.0) (2022-04-27)


### Bug Fixes

* **npm:** update dependencies ([4e5a1cf](https://github.com/JamieMason/syncpack/commit/4e5a1cf484bcfcdab2eef6ded1558ddb51a49286))
* **npm:** update dependencies ([eebbcde](https://github.com/JamieMason/syncpack/commit/eebbcde479adefdbf0dee0f7560c8bc0952a1c03))


### Features

* **cli:** sync versions of locally developed packages ([0367c9f](https://github.com/JamieMason/syncpack/commit/0367c9fe669172fad27d9a8fdf2125e3a5054c51)), closes [#66](https://github.com/JamieMason/syncpack/issues/66)


### BREAKING CHANGES

* **cli:** If a package developed in your Monorepo depends on another package
developed in your Monorepo, syncpack will now fix the installed version
of the dependent to match the actual version from the package.json file
of the local package.

You can disable this functionality by setting `"workspace": false` in
your `.syncpackrc` config file, or by omitting the new `--workspace`
option when using `--dev`, `--prod` etc to define which dependency types
you wish to include.



## [6.2.1](https://github.com/JamieMason/syncpack/compare/6.2.0...6.2.1) (2022-04-12)


### Bug Fixes

* **globs:** ignore node_modules ([8e11545](https://github.com/JamieMason/syncpack/commit/8e115451f2e9f08745a1ca53c03d502f8a21c2fb)), closes [#68](https://github.com/JamieMason/syncpack/issues/68) [#70](https://github.com/JamieMason/syncpack/issues/70)



# [6.2.0](https://github.com/JamieMason/syncpack/compare/6.1.0...6.2.0) (2022-01-03)


### Features

* **groups:** target specific dependency types ([565c1e7](https://github.com/JamieMason/syncpack/commit/565c1e76c71592dc0353266e1289dd9d0bf3fd9b))



# [6.1.0](https://github.com/JamieMason/syncpack/compare/6.0.0...6.1.0) (2022-01-03)


### Features

* **groups:** pin a version group to a specific version ([3de6f90](https://github.com/JamieMason/syncpack/commit/3de6f90752b04f8324c4e30823a39e009ef0587b)), closes [#44](https://github.com/JamieMason/syncpack/issues/44) [#53](https://github.com/JamieMason/syncpack/issues/53) [#63](https://github.com/JamieMason/syncpack/issues/63) [#64](https://github.com/JamieMason/syncpack/issues/64)



# [6.0.0](https://github.com/JamieMason/syncpack/compare/5.8.15...6.0.0) (2022-01-01)


### Bug Fixes

* **npm:** update dependencies ([fdef0a2](https://github.com/JamieMason/syncpack/commit/fdef0a202340e4287bf8f7e6ae27953002c5b4ee))


### Features

* **core:** add glob support and semver range rule groups ([787757c](https://github.com/JamieMason/syncpack/commit/787757c4b09163ec12a60b190954811c0cf4f15f))


### BREAKING CHANGES

* **core:** Dependencies defined within the `resolutions` and `overrides` fields are
now processed by syncpack and are enabled by default. To exclude these
new fields you will need to define only the fields you do want to
process, either in your configuration file:

```json
{
  "dev": true,
  "peer": true,
  "prod": true
}
```

or via the command line:

```
syncpack list --dev --peer --prod
```



## [5.8.15](https://github.com/JamieMason/syncpack/compare/5.8.14...5.8.15) (2021-08-08)


### Bug Fixes

* **npm:** update dependencies ([aea1f37](https://github.com/JamieMason/syncpack/commit/aea1f374e4e039a35aec7b2e629b9607fd922c75))



## [5.8.14](https://github.com/JamieMason/syncpack/compare/5.8.12...5.8.14) (2021-08-01)


### Bug Fixes

* **cli:** apply breaking change from commander ([a61d384](https://github.com/JamieMason/syncpack/commit/a61d3845a55f0796defce7cafaaf29541a4ff07f))
* **format:** write files if only whitespace changes ([f38ea40](https://github.com/JamieMason/syncpack/commit/f38ea4092af8181acbc0281c775c0c84acb5be3d)), closes [#54](https://github.com/JamieMason/syncpack/issues/54)



## [5.8.12](https://github.com/JamieMason/syncpack/compare/5.7.11...5.8.12) (2021-08-01)


### Bug Fixes

* **npm:** update dependencies ([91254f6](https://github.com/JamieMason/syncpack/commit/91254f6aa283afcc0b32163864468359dd4f888f))


### Features

* **core:** add lint-semver-ranges command ([b4209f0](https://github.com/JamieMason/syncpack/commit/b4209f076344a9d59830d3bbd75569de9e19b4b3)), closes [#56](https://github.com/JamieMason/syncpack/issues/56)



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



