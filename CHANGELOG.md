<a name="3.3.0"></a>

# [3.3.0](https://github.com/JamieMason/syncpack/compare/3.0.0...3.3.0) (2018-10-28)

### Features

- **cli:** specify dependency types as options ([ec5ef6b](https://github.com/JamieMason/syncpack/commit/ec5ef6b)), closes [#10](https://github.com/JamieMason/syncpack/issues/10)
- **cli:** specify indentation as option ([8b408bd](https://github.com/JamieMason/syncpack/commit/8b408bd)), closes [#12](https://github.com/JamieMason/syncpack/issues/12)
- **format:** sort contributors alphabetically ([935ffcf](https://github.com/JamieMason/syncpack/commit/935ffcf))

### Performance Improvements

- **npm:** move [@types](https://github.com/types) to devDependencies ([ad5951c](https://github.com/JamieMason/syncpack/commit/ad5951c)), closes [#13](https://github.com/JamieMason/syncpack/issues/13)

<a name="3.0.0"></a>

# [3.0.0](https://github.com/JamieMason/syncpack/compare/2.0.1...3.0.0) (2018-08-25)

### Features

- **bin:** override package locations using repeatable --source options ([5dbcfd4](https://github.com/JamieMason/syncpack/commit/5dbcfd4))
- **list-mismatches:** return exit code on finding mismatches ([06958c6](https://github.com/JamieMason/syncpack/commit/06958c6))

### BREAKING CHANGES

- **bin:** Previously the location of package.json files could be overridden like so:

```
syncpack list './package.json' './packages/*/package.json'
```

This is now done using a repeatable `--source` option:

```
syncpack list --source './package.json' --source './packages/*/package.json'
```

This change is to make way for new commands which will also require an
overridable `--target` option.

<a name="2.0.1"></a>

## [2.0.1](https://github.com/JamieMason/syncpack/compare/2.0.0...2.0.1) (2018-04-29)

### Bug Fixes

- **core:** ensure pattern overrides are read ([7513ba5](https://github.com/JamieMason/syncpack/commit/7513ba5))

<a name="2.0.0"></a>

# [2.0.0](https://github.com/JamieMason/syncpack/compare/1.3.2...2.0.0) (2018-04-29)

### Features

- **core:** support multiple glob patterns ([a2b5af0](https://github.com/JamieMason/syncpack/commit/a2b5af0)), closes [#5](https://github.com/JamieMason/syncpack/issues/5) [#6](https://github.com/JamieMason/syncpack/issues/6)

### BREAKING CHANGES

- **core:** --packages option replaced with variadic arguments

<a name="1.3.2"></a>

## [1.3.2](https://github.com/JamieMason/syncpack/compare/1.2.2...1.3.2) (2018-04-28)

### Features

- **core:** add set-semver-ranges command ([4d206b9](https://github.com/JamieMason/syncpack/commit/4d206b9))

<a name="1.2.2"></a>

## [1.2.2](https://github.com/JamieMason/syncpack/compare/1.0.2...1.2.2) (2018-02-10)

### Features

- **core:** add format command ([bae1133](https://github.com/JamieMason/syncpack/commit/bae1133))
- **core:** output current version ([e53cd99](https://github.com/JamieMason/syncpack/commit/e53cd99))

<a name="1.0.2"></a>

## [1.0.2](https://github.com/JamieMason/syncpack/compare/1.0.1...1.0.2) (2018-02-02)

<a name="1.0.1"></a>

## [1.0.1](https://github.com/JamieMason/syncpack/compare/1.0.0...1.0.1) (2018-02-02)

### Bug Fixes

- **core:** correct paths to binaries ([5682cd6](https://github.com/JamieMason/syncpack/commit/5682cd6))

<a name="1.0.0"></a>

# [1.0.0](https://github.com/JamieMason/syncpack/compare/0.3.1...1.0.0) (2018-02-02)

### Bug Fixes

- **core:** correctly check a file is package.json ([d1da609](https://github.com/JamieMason/syncpack/commit/d1da609))
- **core:** handle missing dependency maps ([372aa68](https://github.com/JamieMason/syncpack/commit/372aa68))
- **core:** handle semver ranges containing 1.x.x ([a0f8f56](https://github.com/JamieMason/syncpack/commit/a0f8f56))

### Features

- **core:** add fix-mismatches command ([4793f1f](https://github.com/JamieMason/syncpack/commit/4793f1f))
- **core:** add list command ([3b29176](https://github.com/JamieMason/syncpack/commit/3b29176))
- **core:** add list-mismatches command ([735ad2b](https://github.com/JamieMason/syncpack/commit/735ad2b))
- **core:** update command line API ([de8dcb2](https://github.com/JamieMason/syncpack/commit/de8dcb2))

### BREAKING CHANGES

- **core:** The previous commands have been replaced.

<a name="0.3.1"></a>

## [0.3.1](https://github.com/JamieMason/syncpack/compare/0.3.0...0.3.1) (2017-08-23)

### Bug Fixes

- **copy-values:** write results to disk ([a641de4](https://github.com/JamieMason/syncpack/commit/a641de4))

<a name="0.3.0"></a>

# [0.3.0](https://github.com/JamieMason/syncpack/compare/0.2.1...0.3.0) (2017-08-22)

### Features

- **cli:** add copy-values command ([b51a2c9](https://github.com/JamieMason/syncpack/commit/b51a2c9))

<a name="0.2.1"></a>

## [0.2.1](https://github.com/JamieMason/syncpack/compare/0.2.0...0.2.1) (2017-08-20)

### Bug Fixes

- **core:** update dependencies, fix lint warnings ([a65eef7](https://github.com/JamieMason/syncpack/commit/a65eef7))

<a name="0.2.0"></a>

# [0.2.0](https://github.com/JamieMason/syncpack/compare/0.1.0...0.2.0) (2017-08-20)

### Features

- **sync:** synchronise versions across multiple package.json ([7d5848a](https://github.com/JamieMason/syncpack/commit/7d5848a))

<a name="0.1.0"></a>

# [0.1.0](https://github.com/JamieMason/syncpack/compare/f6dada7...0.1.0) (2017-08-18)

### Features

- **cli:** create scaffold cli ([f6dada7](https://github.com/JamieMason/syncpack/commit/f6dada7))
