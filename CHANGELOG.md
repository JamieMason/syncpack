# Changelog

## [14.3.0](https://github.com/JamieMason/syncpack/compare/14.2.1...14.3.0) (2026-03-29)

### Features

* **rcfile:** allow // comment properties ([df0cc01](https://github.com/JamieMason/syncpack/commit/df0cc01b4e716f260de8cb5a6e2d9e0526f0d6a3))

### Bug Fixes

* **cargo:** update dependencies ([6af7e45](https://github.com/JamieMason/syncpack/commit/6af7e45abfac48313958196ea3ea20aebe9243fa))

## [14.2.1](https://github.com/JamieMason/syncpack/compare/14.2.0...14.2.1) (2026-03-21)

### Bug Fixes

* **cargo:** update dependencies ([c6b8a0c](https://github.com/JamieMason/syncpack/commit/c6b8a0cd33d885a448bd5be4e6f98389b95db2cb))

### Performance Improvements

* **core:** avoid clone when reading props ([e1dea16](https://github.com/JamieMason/syncpack/commit/e1dea165461ac7b2bbf85cc3534ce9a35c5fd280))
* **core:** avoid reserialising json to compare ([422a26a](https://github.com/JamieMason/syncpack/commit/422a26ae46104c30e8cfcacb0a79e008fab9acb1))
* **core:** detect indent/newline once ([e476300](https://github.com/JamieMason/syncpack/commit/e4763006a6d9a98c719db3b932ae721f17e13182))
* **specifier:** lazily create node_[range|version] ([cc1eac7](https://github.com/JamieMason/syncpack/commit/cc1eac70726966cd81f38152e4f7ceb9d58d5d67))
* **specifier:** optimise parsing ([9a91292](https://github.com/JamieMason/syncpack/commit/9a91292961e210a8081ac6130cd7bff10c86173f))

## [14.2.0](https://github.com/JamieMason/syncpack/compare/14.1.0...14.2.0) (2026-03-08)

### Features

* **groups:** extend and improve sameMinor policy ([42c31a3](https://github.com/JamieMason/syncpack/commit/42c31a34073d816317e5144a29fa63572b28a0e1))

### Bug Fixes

* **cli:** fix link to DiffersToHighestOrLowestSemver docs ([51bf8ff](https://github.com/JamieMason/syncpack/commit/51bf8fffda78b50881df5b0c5cb5a7ace5868e88)), closes [#324](https://github.com/JamieMason/syncpack/issues/324)

## [14.1.0](https://github.com/JamieMason/syncpack/compare/14.0.2...14.1.0) (2026-03-07)

### Features

* **cli:** add --reporter json to fix and format commands ([5522ffe](https://github.com/JamieMason/syncpack/commit/5522ffe646fec8f9803b4aa63eb62a8b366d46d4)), closes [#322](https://github.com/JamieMason/syncpack/issues/322)
* **cli:** add statusType property to json output ([5a82a85](https://github.com/JamieMason/syncpack/commit/5a82a85ca5dd93037488bf16e771dd01e041cc8b)), closes [#322](https://github.com/JamieMason/syncpack/issues/322)
* **write:** detect indentation when config is not set ([66c0493](https://github.com/JamieMason/syncpack/commit/66c0493cb019dc9df1d415d2ec1487f9ce40bd3e)), closes [#318](https://github.com/JamieMason/syncpack/issues/318)

## [14.0.2](https://github.com/JamieMason/syncpack/compare/14.0.1...14.0.2) (2026-03-07)

### Bug Fixes

* **globs:** ignore node_modules ([467cb57](https://github.com/JamieMason/syncpack/commit/467cb5797e3914f988c4a74370790abe4d38ff03)), closes [#321](https://github.com/JamieMason/syncpack/issues/321)

## [14.0.1](https://github.com/JamieMason/syncpack/compare/14.0.0...14.0.1) (2026-03-05)

### Bug Fixes

* **cargo:** update dependencies ([c85144b](https://github.com/JamieMason/syncpack/commit/c85144b17d324cde937899bdb0aeddf3823ad03e))
* **core:** handle negated source globs ([aaf5032](https://github.com/JamieMason/syncpack/commit/aaf5032e115e81e09c0d71395949ca96c6639716)), closes [#319](https://github.com/JamieMason/syncpack/issues/319)

## [14.0.0](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.41...14.0.0) (2026-02-16)

### Features

* **syncpack:** remove alpha status ([b3ec0ba](https://github.com/JamieMason/syncpack/commit/b3ec0ba0bb871b5cbbbd76f56934b4c5cea988cf))

### Bug Fixes

* **cargo:** update dependencies ([8edfa81](https://github.com/JamieMason/syncpack/commit/8edfa81bcd5e5d74f11290ad739624625927fb8a))

## [14.0.0-alpha.41](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.40...14.0.0-alpha.41) (2026-02-08)

### Bug Fixes

* **cargo:** update dependencies ([1700ed9](https://github.com/JamieMason/syncpack/commit/1700ed9a4b1b3d396cf7a408ef84645d8430335b))
* **core:** apply semver groups to highest semver calculation ([48a80ee](https://github.com/JamieMason/syncpack/commit/48a80ee6f0048cdc87a2f01a5bce684a219d25d0)), closes [#314](https://github.com/JamieMason/syncpack/issues/314)

## [14.0.0-alpha.40](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.39...14.0.0-alpha.40) (2026-01-30)

### Features

* **cli:** print migration guides for deprecated commands ([1e1b904](https://github.com/JamieMason/syncpack/commit/1e1b904d5fe5d03e2670572cf2af86fe2f17d2f2))

### Bug Fixes

* **cargo:** update dependencies ([1bf8f04](https://github.com/JamieMason/syncpack/commit/1bf8f0400878149450fee2360a36b85b69f3250c))

## [14.0.0-alpha.39](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.38...14.0.0-alpha.39) (2026-01-28)

### Features

* **config:** replace tsx with node@>=22.6 type stripping ([8b1eaa0](https://github.com/JamieMason/syncpack/commit/8b1eaa0bb21787ee3a9d8a83a6915a76b2aff1a0))

## [14.0.0-alpha.38](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.37...14.0.0-alpha.38) (2026-01-27)

### Bug Fixes

* **core:** resolve semverGroup / highestSemver conflict ([593e1c8](https://github.com/JamieMason/syncpack/commit/593e1c8567ad33689c261269a97c0de9d3f736a5)), closes [#314](https://github.com/JamieMason/syncpack/issues/314)

## [14.0.0-alpha.37](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.36...14.0.0-alpha.37) (2026-01-11)

### Features

* **rcfile:** exit on deprecated or hallucinated config ([ed5cc51](https://github.com/JamieMason/syncpack/commit/ed5cc51ec42c9424dc7a1fbe59ff8d693357f07f))

### Bug Fixes

* **cargo:** update dependencies ([229618e](https://github.com/JamieMason/syncpack/commit/229618e8edefa78f0d97985b15135b431ed90c52))
* **rcfile:** try to continue if tsx errors ([c7cdb97](https://github.com/JamieMason/syncpack/commit/c7cdb9716cccab6f6baabec2a31a6ffabcfa451d)), closes [#313](https://github.com/JamieMason/syncpack/issues/313)

## [14.0.0-alpha.36](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.35...14.0.0-alpha.36) (2026-01-11)

### Features

* **rcfile:** skip npx etc entirely ([16031b6](https://github.com/JamieMason/syncpack/commit/16031b671801a8ffdcbc0de58871797d82af5f98))

## [14.0.0-alpha.35](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.34...14.0.0-alpha.35) (2026-01-10)

### Bug Fixes

* **config:** use pnpm dlx in pnpm projects ([350fbb1](https://github.com/JamieMason/syncpack/commit/350fbb13ec4cce43b31a57322765b1099cfeeb85)), closes [#310](https://github.com/JamieMason/syncpack/issues/310)

## [14.0.0-alpha.34](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.33...14.0.0-alpha.34) (2025-12-28)

### Bug Fixes

* **core:** ensure source globs apply correctly ([636dfac](https://github.com/JamieMason/syncpack/commit/636dfac47141991e2a444c6285c581c17baf9dde)), closes [#312](https://github.com/JamieMason/syncpack/issues/312) [#311](https://github.com/JamieMason/syncpack/issues/311)

## [14.0.0-alpha.33](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.32...14.0.0-alpha.33) (2025-12-28)

### Bug Fixes

* **cargo:** update dependencies ([8ae1cc7](https://github.com/JamieMason/syncpack/commit/8ae1cc79b3900cf4658d25fb673942c0a2eedb24))
* **core:** ensure ignored paths are not visited ([34d9810](https://github.com/JamieMason/syncpack/commit/34d9810dffcd5e4f510ea0b57d71ff2438acedaf)), closes [#311](https://github.com/JamieMason/syncpack/issues/311)

## [14.0.0-alpha.32](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.31...14.0.0-alpha.32) (2025-12-07)

### Features

* **npm:** add 2 musl build targets ([78378dd](https://github.com/JamieMason/syncpack/commit/78378dd26a71217627d6fa504b7661fe5bd4543e)), closes [#289](https://github.com/JamieMason/syncpack/issues/289)

### Bug Fixes

* **cargo:** update dependencies ([28f1b8a](https://github.com/JamieMason/syncpack/commit/28f1b8a6cef52cf37879179746c55a039d8db228))
* **npm:** update dependencies ([2d23393](https://github.com/JamieMason/syncpack/commit/2d23393228256addf4691b2d2421649b70545b9f))
* **pnpm:** support PnP ([1b43030](https://github.com/JamieMason/syncpack/commit/1b430301204d069f483f4ec46d35013754c4245c)), closes [#297](https://github.com/JamieMason/syncpack/issues/297)

### Performance Improvements

* **core:** optimise pattern matching ([8582812](https://github.com/JamieMason/syncpack/commit/8582812c2c265e037014b3eb6795e70f829df457))

## [14.0.0-alpha.31](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.30...14.0.0-alpha.31) (2025-12-06)

### Features

* **npm:** use trusted publishing (attempt 2) ([a9a9899](https://github.com/JamieMason/syncpack/commit/a9a9899624c5dc1e824529cad6825f7771436dc7))

## [14.0.0-alpha.30](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.29...14.0.0-alpha.30) (2025-12-06)

### Features

* **npm:** use trusted publishing ([bbb761d](https://github.com/JamieMason/syncpack/commit/bbb761db1e802133c63cf65669d9905e0dcd5611))

## [14.0.0-alpha.29](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.28...14.0.0-alpha.29) (2025-11-25)

### Features

* **core:** add support for link: specifiers ([74cdef0](https://github.com/JamieMason/syncpack/commit/74cdef0a99f53ab13bbdd3183e363ee5afdaf5fc)), closes [#293](https://github.com/JamieMason/syncpack/issues/293)

### Bug Fixes

* **cargo:** update dependencies ([0e425fa](https://github.com/JamieMason/syncpack/commit/0e425fa0b75b595a5885eb6b0de989364b2f7da0))

### Performance Improvements

* **core:** remove unused code ([ace7f79](https://github.com/JamieMason/syncpack/commit/ace7f79bc6a5d0a47423b7727d988f37450a2107))

## [14.0.0-alpha.28](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.27...14.0.0-alpha.28) (2025-11-23)

### Bug Fixes

* **cargo:** update dependencies ([c2c928e](https://github.com/JamieMason/syncpack/commit/c2c928ed32eff14842a420df7c8fc6be381d9b9d))
* **cli:** completely ignore instances not matching cli filters ([ac62d61](https://github.com/JamieMason/syncpack/commit/ac62d61517cd29bae7354d6ff3e7c2206a799543)), closes [#308](https://github.com/JamieMason/syncpack/issues/308)
* **npm:** update dependencies ([024d845](https://github.com/JamieMason/syncpack/commit/024d8455a3789240d321706d7d6a6bcefacf26d1))

## [14.0.0-alpha.27](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.26...14.0.0-alpha.27) (2025-11-12)

### Features

* **core:** support catalog: specifiers ([6920e9e](https://github.com/JamieMason/syncpack/commit/6920e9e07c2e6afc6834f108b3a7461c501d1a7e)), closes [#258](https://github.com/JamieMason/syncpack/issues/258)

### Bug Fixes

* **cargo:** update dependencies ([1c6c937](https://github.com/JamieMason/syncpack/commit/1c6c937741eec6ba8caeb038542d0e2aeb30b87e))

## [14.0.0-alpha.26](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.25...14.0.0-alpha.26) (2025-11-09)

### Features

* **cli:** add --config option ([f1a19ae](https://github.com/JamieMason/syncpack/commit/f1a19ae1dcfafc9054b32678999ba0d6a284b143))
* **config:** warn on deprecated values ([f5e91d7](https://github.com/JamieMason/syncpack/commit/f5e91d78f701b480996d02eb2ff596cd93c103d9))

## [14.0.0-alpha.25](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.24...14.0.0-alpha.25) (2025-11-05)

### Features

* **config:** default format[Bugs|Repository] to false ([0fd7c9d](https://github.com/JamieMason/syncpack/commit/0fd7c9d6cabe38b4c56269826c89e4e438c3ae4f)), closes [#306](https://github.com/JamieMason/syncpack/issues/306)

### Bug Fixes

* **config:** auto enable custom types ([1a9bd68](https://github.com/JamieMason/syncpack/commit/1a9bd6811ebc66629462a48761852eba3aef04f0)), closes [#307](https://github.com/JamieMason/syncpack/issues/307)
* **core:** compare unresolved workspace: specifiers ([20319d7](https://github.com/JamieMason/syncpack/commit/20319d7f0df32dd37f31d7e8cddb06d97dd76703))
* **core:** handle specifiers like =9.0.0 ([4a06c2c](https://github.com/JamieMason/syncpack/commit/4a06c2c40d9f6de9477a945e524c5a2b86db9c9d)), closes [#239](https://github.com/JamieMason/syncpack/issues/239)
* **core:** normalise windows backslashes ([863f82a](https://github.com/JamieMason/syncpack/commit/863f82a7870c453aeebe4d8cdf88ca6d22b1850b)), closes [#126](https://github.com/JamieMason/syncpack/issues/126)
* **update:** skip deprecated versions ([7e87a0f](https://github.com/JamieMason/syncpack/commit/7e87a0fc438c808ddce89fff8bfa47e7457ce3ee)), closes [#292](https://github.com/JamieMason/syncpack/issues/292)

## [14.0.0-alpha.24](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.23...14.0.0-alpha.24) (2025-11-04)

### Features

* **core:** support bunx when available ([98c0bf3](https://github.com/JamieMason/syncpack/commit/98c0bf3a036fd531a71cacacc2ec5481948d5e22)), closes [#305](https://github.com/JamieMason/syncpack/issues/305)

### Bug Fixes

* **cargo:** update dependencies ([cfe3f36](https://github.com/JamieMason/syncpack/commit/cfe3f364c0ef27706fcc6e15b8fcc533df8301dc))
* **npm:** update dependencies ([30c65fe](https://github.com/JamieMason/syncpack/commit/30c65feb05b54d4663b4902c77f3e6e25ffdbfdf))

## [14.0.0-alpha.23](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.22...14.0.0-alpha.23) (2025-09-29)

### Reverts

* **github:** revert actions update ([00afd9d](https://github.com/JamieMason/syncpack/commit/00afd9dbf324b11839c183a2a76619c735aa4601))

## [14.0.0-alpha.22](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.21...14.0.0-alpha.22) (2025-09-29)

### Features

* **groups:** add sameMinor policy ([1b1d5cb](https://github.com/JamieMason/syncpack/commit/1b1d5cb31029a0f94666d85131e8d5cc1b113830))

### Bug Fixes

* **cargo:** update dependencies ([d4c9f93](https://github.com/JamieMason/syncpack/commit/d4c9f9370c03df8ccb49d668452f18c39ea4609d))
* **cargo:** update dependencies ([34387d8](https://github.com/JamieMason/syncpack/commit/34387d822bec344cdcc2efa3af44ca5ec3071957))
* **cargo:** update dependencies ([c40a0e3](https://github.com/JamieMason/syncpack/commit/c40a0e3a899140493a5e5d65f4d51c20be19b894))
* **npm:** update dependencies ([99b2e93](https://github.com/JamieMason/syncpack/commit/99b2e93a7c3d8572cf3ad2a3a0046fd1e7d42111))

## [14.0.0-alpha.21](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.20...14.0.0-alpha.21) (2025-09-14)

### Bug Fixes

* **cargo:** update dependencies ([83761e6](https://github.com/JamieMason/syncpack/commit/83761e6be073274da1d77814b6337b789e21d58b))
* **update:** add timeouts to npm registry client ([1f3fb3e](https://github.com/JamieMason/syncpack/commit/1f3fb3e8ced2ec79d667f9a761e8fff401d48963)), closes [#291](https://github.com/JamieMason/syncpack/issues/291)
* **update:** exclude unpublished versions ([8411a77](https://github.com/JamieMason/syncpack/commit/8411a77606b9032ed6ae8c5069d0e6349b7f290f)), closes [#299](https://github.com/JamieMason/syncpack/issues/299)

### Performance Improvements

* **core:** disable serde rc ([3fdade1](https://github.com/JamieMason/syncpack/commit/3fdade1421e363420a687fcc63f1f6f95c933842))
* **core:** move ctor to dev dependencies ([b465088](https://github.com/JamieMason/syncpack/commit/b465088077a02f5c0eb6bd60bc035acd9d2307a0))

## [14.0.0-alpha.20](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.19...14.0.0-alpha.20) (2025-09-11)

### Bug Fixes

* **cargo:** update dependencies ([4d6c836](https://github.com/JamieMason/syncpack/commit/4d6c8362a7fef264a1b19e061a7a136250374a65))
* **format:** return exit code 0 when formatting files ([57f05d1](https://github.com/JamieMason/syncpack/commit/57f05d1fbd4e93296314253601ab05be83849c05))
* **npm:** update dependencies ([6a10f00](https://github.com/JamieMason/syncpack/commit/6a10f0022c2ce0355050bf6db69e01242baf97c6))

### Performance Improvements

* **cargo:** remove dhat dependency ([548e9bd](https://github.com/JamieMason/syncpack/commit/548e9bd6848fc0c136f48552ca37208dd7863683)), closes [#294](https://github.com/JamieMason/syncpack/issues/294)
* **core:** optimise regex dependency ([c9d454c](https://github.com/JamieMason/syncpack/commit/c9d454c795349164a4b606b538b9ce59df4e5a75))
* **core:** optimise tokio dependency ([c9b5075](https://github.com/JamieMason/syncpack/commit/c9b5075e698a5b59f2a9ac15c11c07e8dc35174d))
* **core:** remove icu dependency ([c6d48b5](https://github.com/JamieMason/syncpack/commit/c6d48b5908284ee9fc0a55a6d377e53805fe2eb0))

## [14.0.0-alpha.19](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.18...14.0.0-alpha.19) (2025-08-02)

### Bug Fixes

* **cargo:** update dependencies ([e8f7bf3](https://github.com/JamieMason/syncpack/commit/e8f7bf39a45eddcaf2dc5bc787dee6fa9e1210fa))
* **npm:** update dependencies ([a7096b3](https://github.com/JamieMason/syncpack/commit/a7096b3eade13a3043a32ba11de6d3abc5015b37))

### Performance Improvements

* **cargo:** optimise binary for file size and not speed ([07ce2c5](https://github.com/JamieMason/syncpack/commit/07ce2c5ddf05bf65702aaa709c7b1e5cde27cb6b)), closes [#294](https://github.com/JamieMason/syncpack/issues/294)
* **cargo:** remove openssl dependency ([99cdf35](https://github.com/JamieMason/syncpack/commit/99cdf35046339b1e5702bfa3cf63c6e0e0cc9b36)), closes [#294](https://github.com/JamieMason/syncpack/issues/294)
* **core:** optimise version specifier parsing ([86c0fa7](https://github.com/JamieMason/syncpack/commit/86c0fa7d114bbfd6028d939e18e42ac621a69343))

## [14.0.0-alpha.18](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.17...14.0.0-alpha.18) (2025-07-05)

### Features

* **json:** add command to output instances as json ([dba3cd6](https://github.com/JamieMason/syncpack/commit/dba3cd60e0eb67f9bf1e9eda9f25c5d7a02b7fcd)), closes [#197](https://github.com/JamieMason/syncpack/issues/197)

### Bug Fixes

* **cargo:** update dependencies ([30e848d](https://github.com/JamieMason/syncpack/commit/30e848ddc3165b8a4dd32467271f8125235f9257))
* **npm:** update dependencies ([a51fe77](https://github.com/JamieMason/syncpack/commit/a51fe77713948526654faf6fd80f334869e3afb0))

## [14.0.0-alpha.17](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.16...14.0.0-alpha.17) (2025-06-29)

### Bug Fixes

* **cargo:** update dependencies ([3611f16](https://github.com/JamieMason/syncpack/commit/3611f16249a248acf6f89acb82e276ec0345def8))
* **cli:** fix regression in filtering cli options ([320d0f2](https://github.com/JamieMason/syncpack/commit/320d0f20933dab316a358ab845571c31830e2b8c))

## [14.0.0-alpha.16](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.15...14.0.0-alpha.16) (2025-06-22)

### Bug Fixes

* **config:** locate config in JS/TS rcfiles correctly ([f2c8eb3](https://github.com/JamieMason/syncpack/commit/f2c8eb35da9bcdcc8e885a563778ad72f1e07c23))

### Performance Improvements

* **core:** only create client when running update ([73d9e26](https://github.com/JamieMason/syncpack/commit/73d9e26fca55665e0c05d36d0447b294e069aad8))
* **core:** optimise assigning instances to groups ([8074cc7](https://github.com/JamieMason/syncpack/commit/8074cc77090eee4a93574bef6868a123e78da788))

## [14.0.0-alpha.15](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.14...14.0.0-alpha.15) (2025-06-21)

### Bug Fixes

* **cargo:** update dependencies ([d9d8467](https://github.com/JamieMason/syncpack/commit/d9d84670ea711b91a6c60effab5a984b366aeb74))
* **config:** improve logging and error handling when discovering rcfile ([e469284](https://github.com/JamieMason/syncpack/commit/e46928498f8953b91374b208528fdfa2d0c3010d))
* **core:** move tsx from peers to dependencies ([40ab6e5](https://github.com/JamieMason/syncpack/commit/40ab6e5b5d4ea48e68f8edbcf1cedbfec3553eb5))
* **schema:** ensure typescript types and json schema are up to date ([db8b6f6](https://github.com/JamieMason/syncpack/commit/db8b6f676a4a8e88534eed106d3feb25cd08c1a9)), closes [#281](https://github.com/JamieMason/syncpack/issues/281)

## [14.0.0-alpha.14](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.13...14.0.0-alpha.14) (2025-06-15)

### Features

* **core:** complete removal of banned custom types ([3641fa4](https://github.com/JamieMason/syncpack/commit/3641fa43543f504072cdee2fe00a4d3bb2c8df52))

### Bug Fixes

* **cargo:** update dependencies ([201806f](https://github.com/JamieMason/syncpack/commit/201806fea4013b1906d73b61b99e674696b719b2))
* **cargo:** update icu to v2 ([8f9e82b](https://github.com/JamieMason/syncpack/commit/8f9e82bf96190f19b1efbc1722a6d6856d04b046))
* **npm:** update dependencies ([9192ed6](https://github.com/JamieMason/syncpack/commit/9192ed6ed86cb9b9a27f00c7852c4863df41ccf4))

### Performance Improvements

* **config:** migrate cosmiconfig to rust and tsx ([c529488](https://github.com/JamieMason/syncpack/commit/c529488e183d62fc3351b4a313b6406cba718535)), closes [#282](https://github.com/JamieMason/syncpack/issues/282)

## [14.0.0-alpha.13](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.12...14.0.0-alpha.13) (2025-06-08)

### Features

* **cli:** use clearer hints for option values ([4b0afb4](https://github.com/JamieMason/syncpack/commit/4b0afb40c9b64503aef80783cb5996cc894e216f))
* **update:** support updating @jsr/** dependencies ([236a7d1](https://github.com/JamieMason/syncpack/commit/236a7d16befa6c3ad81a3000df68432e7d5779cf)), closes [#249](https://github.com/JamieMason/syncpack/issues/249)

### Bug Fixes

* **cargo:** update dependencies ([40a61c7](https://github.com/JamieMason/syncpack/commit/40a61c72bdea4f0a397e89edf2650055023b215a))
* **npm:** update dependencies ([e695d13](https://github.com/JamieMason/syncpack/commit/e695d1300ab00ed506e78f10912360fc1c523528))
* **npm:** update dependencies ([6655910](https://github.com/JamieMason/syncpack/commit/6655910765a280f2655b9bd9dcb58d2068594e53))

## [14.0.0-alpha.12](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.11...14.0.0-alpha.12) (2025-05-05)

### Features

* **cli:** rewrite list command ([32c5568](https://github.com/JamieMason/syncpack/commit/32c5568b2bc9dbcef20c30b90f85f162962c08c9))
* **cli:** summarise status codes in each dependency ([25b7823](https://github.com/JamieMason/syncpack/commit/25b78239e68a5acb43cc88a4cfaf0c756ca75cd9))

## [14.0.0-alpha.11](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.10...14.0.0-alpha.11) (2025-04-27)

### Features

* **cli:** add a 'none' option to --show ([6776a78](https://github.com/JamieMason/syncpack/commit/6776a780514b3702b698c660803269a1279930e8))
* **cli:** tidy output and improve consistency ([89eccc2](https://github.com/JamieMason/syncpack/commit/89eccc21ebf44c108aedd9b4a4626fe8a17b3421)), closes [#263](https://github.com/JamieMason/syncpack/issues/263)
* **update:** rewrite syncpack update for v14 ([f11d741](https://github.com/JamieMason/syncpack/commit/f11d741b95dbf53286300aa961330f07ce8e3d60)), closes [#276](https://github.com/JamieMason/syncpack/issues/276) [#210](https://github.com/JamieMason/syncpack/issues/210) [#196](https://github.com/JamieMason/syncpack/issues/196) [#190](https://github.com/JamieMason/syncpack/issues/190) [#175](https://github.com/JamieMason/syncpack/issues/175)

## [14.0.0-alpha.10](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.9...14.0.0-alpha.10) (2025-02-11)

### Bug Fixes

* **core:** read rcfile correctly on windows ([ed01989](https://github.com/JamieMason/syncpack/commit/ed019898825122bda46b3f7b361aec8f0b3815e0)), closes [#262](https://github.com/JamieMason/syncpack/issues/262)

## [14.0.0-alpha.9](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.8...14.0.0-alpha.9) (2025-02-10)

### Bug Fixes

* **groups:** fix regression to [#204](https://github.com/JamieMason/syncpack/issues/204) in 52930896 ([ca44802](https://github.com/JamieMason/syncpack/commit/ca448023cd88b6b9ae5b41aa00ecd100224b3867))

## [14.0.0-alpha.8](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.7...14.0.0-alpha.8) (2025-02-10)

### Features

* **config:** add svelte to default sortExports ([c6219be](https://github.com/JamieMason/syncpack/commit/c6219beb3e2b57df930993efa5e0a29ea7d71c06)), closes [#251](https://github.com/JamieMason/syncpack/issues/251)

### Bug Fixes

* **cli:** read --dry-run option correctly ([865edaf](https://github.com/JamieMason/syncpack/commit/865edafebf90b31373af3f499b8a03b964ebfe26))

## [14.0.0-alpha.7](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.6...14.0.0-alpha.7) (2025-02-09)

### Features

* **cli:** add --dry-run option ([86505a0](https://github.com/JamieMason/syncpack/commit/86505a0dcac14394a957285463f56b42d07486ee))
* **cli:** improve readability of lint output ([df4e08c](https://github.com/JamieMason/syncpack/commit/df4e08ca40c331f028a2ef23b638db14d7a31166))
* **config:** exit 1 when a dependency type is not found ([66d043e](https://github.com/JamieMason/syncpack/commit/66d043eb8f7b3fe371e0839599025d6583e62b17)), closes [#234](https://github.com/JamieMason/syncpack/issues/234)

## [14.0.0-alpha.6](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.5...14.0.0-alpha.6) (2025-02-09)

### Features

* **core:** update semver in git urls, aliases etc ([b81c7cd](https://github.com/JamieMason/syncpack/commit/b81c7cd123ace64bdfaf740b7718ab59e19a036c)), closes [#213](https://github.com/JamieMason/syncpack/issues/213)
* **core:** write a new specifier parser ([5293089](https://github.com/JamieMason/syncpack/commit/52930896eb360861cb2d52b1cfb4cc4fcd101bcc)), closes [#261](https://github.com/JamieMason/syncpack/issues/261)

### Bug Fixes

* **cargo:** update dependencies ([67237b6](https://github.com/JamieMason/syncpack/commit/67237b6c190b7f248625e09b5c1abdc4855f575e))
* **core:** fix cli filters regression in 2340ea9a ([7ac8847](https://github.com/JamieMason/syncpack/commit/7ac8847571f29031b50dbcab6a62d1feb8783ad2))
* **core:** workaround package.json files with no name ([c3f5298](https://github.com/JamieMason/syncpack/commit/c3f529827e5a4e30d8b90ac5ef53fe819712aeae)), closes [#261](https://github.com/JamieMason/syncpack/issues/261)

## [14.0.0-alpha.5](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.4...14.0.0-alpha.5) (2025-02-05)

### Features

* **core:** ignore missing snapTo dependencies by default ([c1472bf](https://github.com/JamieMason/syncpack/commit/c1472bfc8ed4a57b03d63fb87288866eeea4862f)), closes [#173](https://github.com/JamieMason/syncpack/issues/173)

## [14.0.0-alpha.4](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.3...14.0.0-alpha.4) (2025-02-05)

### Features

* **cli:** reveal invalid instances by default ([315cf3c](https://github.com/JamieMason/syncpack/commit/315cf3c5c1025261d6d89eead21bb1fbd4998635))
* **pnpm:** consider workspace protocol valid by default ([5df8b5f](https://github.com/JamieMason/syncpack/commit/5df8b5fe76b19007ac9b92ffea8eebcd12218cff)), closes [#252](https://github.com/JamieMason/syncpack/issues/252)

### Bug Fixes

* **cargo:** update syncpack version in cargo.toml ([e06c640](https://github.com/JamieMason/syncpack/commit/e06c6400caaa71ac59dab354a53899cf120faca0))

## [14.0.0-alpha.3](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.2...14.0.0-alpha.3) (2025-02-05)

### Features

* **groups:** allow many deps to be treated as one ([fccf805](https://github.com/JamieMason/syncpack/commit/fccf805107cd4994d8d66fab20fe9d480fe183d5)), closes [#204](https://github.com/JamieMason/syncpack/issues/204)

### Bug Fixes

* **cli:** change approach to locating rcfile ([4cf7da5](https://github.com/JamieMason/syncpack/commit/4cf7da53178586564eb9f2f16fc4b863c77e3d91)), closes [#253](https://github.com/JamieMason/syncpack/issues/253)
* **pinned:** workspace:* was wrongly marked as pin mismatch ([87975b7](https://github.com/JamieMason/syncpack/commit/87975b7a9688588a775bb5169adff4954ee554ad))
* **types:** allow additional customTypes properties ([a969095](https://github.com/JamieMason/syncpack/commit/a969095a6551e5134da71a76c310622369b348b5)), closes [#255](https://github.com/JamieMason/syncpack/issues/255)

## [14.0.0-alpha.2](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.1...14.0.0-alpha.2) (2025-02-04)

### Bug Fixes

* **cargo:** update dependencies ([e328418](https://github.com/JamieMason/syncpack/commit/e328418cab18bd5b972e3a59a83f3a4bbeec66e3))
* **npm:** update dependencies ([98203c5](https://github.com/JamieMason/syncpack/commit/98203c5d2c1d276342f27f29f4dd012d594b3c13))

## [14.0.0-alpha.1](https://github.com/JamieMason/syncpack/compare/14.0.0-alpha.0...14.0.0-alpha.1) (2024-11-01)

## [14.0.0-alpha.0](https://github.com/JamieMason/syncpack/compare/13.0.0...14.0.0-alpha.0) (2024-11-01)

### Features

* **core:** migrate to rust ([a37e52e](https://github.com/JamieMason/syncpack/commit/a37e52e5a29215cc57c28b7a2fb7df27069fd2dc))

## [13.0.0](https://github.com/JamieMason/syncpack/compare/12.4.0...13.0.0) (2024-08-25)

### ⚠ BREAKING CHANGES

* **engines:** Changed `engines.node` from `>=16` to `>=18.18.0`.

`minimatch@10` was updated in `syncpack@12.4.0` which required `node@>=20` and
caused issues in some projects.

This change adds a local linter to verify that the node engines of syncpack's
dependencies all satisfy syncpack's own node engine. Using this linter found
that the minimum node engine of all of syncpack's dependencies is v18.18.0.
* **core:** The commands `fix-mismatches`, `set-semver-ranges`, `prompt`, and `update` will
no longer result in package.json files being formatted. This was a side effect
of `JSON.parse` and `JSON.stringify` being used to read and write to disk.

From this version, only specific changes are applied. To preserve the previous
behaviour, run `syncpack format` afterwards to apply fixes to formatting should
they be needed.

### Features

* **core:** preserve formatting when applying fixes ([e483862](https://github.com/JamieMason/syncpack/commit/e483862a62cfad68dc69d27c15ca17778be5d940)), closes [#241](https://github.com/JamieMason/syncpack/issues/241) [#195](https://github.com/JamieMason/syncpack/issues/195)

### Bug Fixes

* **engines:** ensure correct required node version ([fed04e4](https://github.com/JamieMason/syncpack/commit/fed04e4bc183e10d42df209ccb3bddf06766c60a)), closes [#237](https://github.com/JamieMason/syncpack/issues/237)
* **npm:** remove unused peer dependencies ([aac4e24](https://github.com/JamieMason/syncpack/commit/aac4e24f7fdd472b0b02224b18006df1085d6fda)), closes [#180](https://github.com/JamieMason/syncpack/issues/180) [#181](https://github.com/JamieMason/syncpack/issues/181)
* **npm:** run pnpm update ([f1e6ce2](https://github.com/JamieMason/syncpack/commit/f1e6ce2151c2cae964296cb38534a9d4c8bc66de))

## [12.4.0](https://github.com/JamieMason/syncpack/compare/12.3.3...12.4.0) (2024-07-24)

### Features

* **groups:** handle negation for packages option ([0d6b608](https://github.com/JamieMason/syncpack/commit/0d6b6085043c2483d7b6ddcb0316e10e523903dd)), closes [#232](https://github.com/JamieMason/syncpack/issues/232)

### Bug Fixes

* **config:** add $schema property to types ([ee54fd7](https://github.com/JamieMason/syncpack/commit/ee54fd70e673088f295e623bbf32582c98c0a5ad)), closes [#200](https://github.com/JamieMason/syncpack/issues/200) [#207](https://github.com/JamieMason/syncpack/issues/207) [#236](https://github.com/JamieMason/syncpack/issues/236)
* **format:** use localeCompare for sorting ([e8c9bd3](https://github.com/JamieMason/syncpack/commit/e8c9bd35bea20deaf7867d5fb4dfa89beb3cb835)), closes [#206](https://github.com/JamieMason/syncpack/issues/206) [#214](https://github.com/JamieMason/syncpack/issues/214)
* **npm:** apply breaking changes after updates ([c2a7744](https://github.com/JamieMason/syncpack/commit/c2a7744a7eb69c4b6f258f346b282fdf4b607d52))
* **npm:** update dependencies ([bbf2cd9](https://github.com/JamieMason/syncpack/commit/bbf2cd9282d9d3f660e5fccd43bcc2e9fa020888))

## [12.3.3](https://github.com/JamieMason/syncpack/compare/12.3.2...12.3.3) (2024-06-24)

### Bug Fixes

* **npm:** use types export in package.json ([e0ab6d2](https://github.com/JamieMason/syncpack/commit/e0ab6d23cd79c798caf9b15afb7e4f0670c0c141)), closes [#218](https://github.com/JamieMason/syncpack/issues/218) [#219](https://github.com/JamieMason/syncpack/issues/219)

## [12.3.2](https://github.com/JamieMason/syncpack/compare/12.3.1...12.3.2) (2024-04-24)

### Reverts

* **update:** revert commit a1c72704 ([1d2339b](https://github.com/JamieMason/syncpack/commit/1d2339b284293079f267dc48f751a985031ad85f)), closes [#210](https://github.com/JamieMason/syncpack/issues/210)

## [12.3.1](https://github.com/JamieMason/syncpack/compare/12.3.0...12.3.1) (2024-04-21)

### Bug Fixes

* **cli:** fix clashing shorthand option names ([3823825](https://github.com/JamieMason/syncpack/commit/3823825ba13309728d97be4ff31bc9bf40732f2d))
* **core:** switch to ESM & update dependencies ([b02c421](https://github.com/JamieMason/syncpack/commit/b02c4215e06fe28d83656860095dc72244913ad2))
* **npm:** update dependencies ([90e7d70](https://github.com/JamieMason/syncpack/commit/90e7d7094abc13ac7d63f70e5dde7f4c9a7af49e))
* **update:** apply update to every outdated instance ([a1c7270](https://github.com/JamieMason/syncpack/commit/a1c727049b4ce31e7d6a37b683764b5fe32165d6))

## [12.3.0](https://github.com/JamieMason/syncpack/compare/12.2.0...12.3.0) (2023-12-30)

### Features

* **format:** sort .exports, expose more config ([6cd7960](https://github.com/JamieMason/syncpack/commit/6cd7960c2ab01fbe1631ef2818060724a44a568b)), closes [#142](https://github.com/JamieMason/syncpack/issues/142)
* **lint:** check files are formatted ([eda4dbb](https://github.com/JamieMason/syncpack/commit/eda4dbb74846ed99cc23598a001698b7f9a0af6f)), closes [#102](https://github.com/JamieMason/syncpack/issues/102) [#3](https://github.com/JamieMason/syncpack/issues/3)

## [12.2.0](https://github.com/JamieMason/syncpack/compare/12.1.0...12.2.0) (2023-12-30)

### Features

* **config:** improve dependencyTypes intellisense ([9c73dc5](https://github.com/JamieMason/syncpack/commit/9c73dc59c46eaa7f3213cb5b01d4dfb17ab70780))
* **core:** add specifier type for * & latest ([e0cb0ef](https://github.com/JamieMason/syncpack/commit/e0cb0ef5d5e3fe8326bd7fff797f9b07572b47d2)), closes [#174](https://github.com/JamieMason/syncpack/issues/174)
* **groups:** target instances by specifier type ([9403c3c](https://github.com/JamieMason/syncpack/commit/9403c3c5a7acc432c21476e49e28f753509e1d78)), closes [#163](https://github.com/JamieMason/syncpack/issues/163)

### Bug Fixes

* **npm:** update dependencies ([2315227](https://github.com/JamieMason/syncpack/commit/2315227c640b17142a30e557d2646f3f72029662))

## [12.1.0](https://github.com/JamieMason/syncpack/compare/12.0.1...12.1.0) (2023-12-29)

### Features

* **config:** support async syncpack.config.mjs ([7216ded](https://github.com/JamieMason/syncpack/commit/7216ded3b1d47940af6397c78a2a9ac69974f0ee)), closes [#164](https://github.com/JamieMason/syncpack/issues/164)

### Bug Fixes

* **local:** allow missing .version if not depended on ([419a254](https://github.com/JamieMason/syncpack/commit/419a2543f9ab4400c6ce3b468e979d19b8e874d5)), closes [#183](https://github.com/JamieMason/syncpack/issues/183)
* **workspace:** add support for workspace:^ ([45f1731](https://github.com/JamieMason/syncpack/commit/45f1731fd98b3b3a718e6eb1f4cfd6ed7db3991c)), closes [#182](https://github.com/JamieMason/syncpack/issues/182)

## [12.0.1](https://github.com/JamieMason/syncpack/compare/12.0.0...12.0.1) (2023-12-26)

### Bug Fixes

* **effect:** add peer for @effect/schema ([04bdf63](https://github.com/JamieMason/syncpack/commit/04bdf63a1df85169023962d7122065173863233a)), closes [#180](https://github.com/JamieMason/syncpack/issues/180) [#181](https://github.com/JamieMason/syncpack/issues/181)

## [12.0.0](https://github.com/JamieMason/syncpack/compare/12.0.0-alpha.1...12.0.0) (2023-12-24)

### Features

* **update:** prompt formatted repository url ([637f2ea](https://github.com/JamieMason/syncpack/commit/637f2eae8467831ced8c354d7ecfbb36848c4ec7)), closes [#178](https://github.com/JamieMason/syncpack/issues/178) [#177](https://github.com/JamieMason/syncpack/issues/177)

### Bug Fixes

* **cli:** make status codes easier to read ([e17b696](https://github.com/JamieMason/syncpack/commit/e17b696b79e4f7b42bf4cbcbc2026e4545758056)), closes [#172](https://github.com/JamieMason/syncpack/issues/172)

## [12.0.0-alpha.1](https://github.com/JamieMason/syncpack/compare/12.0.0-alpha.0...12.0.0-alpha.1) (2023-12-23)

### Bug Fixes

* **effect:** apply breaking changes after update ([2ea9a79](https://github.com/JamieMason/syncpack/commit/2ea9a790493c25a87409643f89db0fbbc9ae7a52))
* **npm:** update dependencies ([ce80ea5](https://github.com/JamieMason/syncpack/commit/ce80ea5eda2aa42b24f6428327ac0804e908b0ea))
* **npm:** update dependencies ([f7c5382](https://github.com/JamieMason/syncpack/commit/f7c5382e2a5a9822296751c2070e54a831c491e0))

## [12.0.0-alpha.0](https://github.com/JamieMason/syncpack/compare/11.2.1...12.0.0-alpha.0) (2023-11-05)

### Features

* **cli:** throw if command is not found ([0608605](https://github.com/JamieMason/syncpack/commit/0608605cce74352a4d66177ef70876e0e376ee48))
* **core:** broaden version specifier support ([46a0143](https://github.com/JamieMason/syncpack/commit/46a0143fc260f0f4cb55c13f8eec3478290641ac)), closes [#161](https://github.com/JamieMason/syncpack/issues/161) [#162](https://github.com/JamieMason/syncpack/issues/162) [#157](https://github.com/JamieMason/syncpack/issues/157)

### Bug Fixes

* **local:** do not replace version with workspace:* ([d8a4eaa](https://github.com/JamieMason/syncpack/commit/d8a4eaa23819ccba72c454f90f79914ca1903ebe))
* **npm:** update dependencies ([f016552](https://github.com/JamieMason/syncpack/commit/f016552c618df84e4a05527589f8380ce921534d))

## [11.2.1](https://github.com/JamieMason/syncpack/compare/10.9.3...11.2.1) (2023-08-14)

### Features

* **config:** add a json schema ([d35ace5](https://github.com/JamieMason/syncpack/commit/d35ace59fa123949f18e8eb03ab76fb5340f6c8c)), closes [#146](https://github.com/JamieMason/syncpack/issues/146) [#147](https://github.com/JamieMason/syncpack/issues/147)
* **depTypes:** handle '!peer' and '**' ([06f2e88](https://github.com/JamieMason/syncpack/commit/06f2e880735b21078191c9deaddb24445f0666c0))
* **depTypes:** rename 'workspace' to 'local' ([463c936](https://github.com/JamieMason/syncpack/commit/463c936076c84ffb587c101c905c0151f48b2211)), closes [#154](https://github.com/JamieMason/syncpack/issues/154)

### Bug Fixes

* **npm:** update dependencies ([c8a5cab](https://github.com/JamieMason/syncpack/commit/c8a5cabf51ca1bdf793c1e7d688a82fa68bee66f))

## [10.9.3](https://github.com/JamieMason/syncpack/compare/10.7.3...10.9.3) (2023-07-31)

### Features

* **node:** support >=16 ([ea6cacf](https://github.com/JamieMason/syncpack/commit/ea6cacf2066cb909ec4866d52f2bb93d297c052a)), closes [#148](https://github.com/JamieMason/syncpack/issues/148)
* **versions:** support npm: alias protocol ([2a7ef86](https://github.com/JamieMason/syncpack/commit/2a7ef8645d458f0a65c51bf362f99902a0691800)), closes [#151](https://github.com/JamieMason/syncpack/issues/151)

## [10.7.3](https://github.com/JamieMason/syncpack/compare/10.7.2...10.7.3) (2023-07-03)

### Bug Fixes

* **workspace:** revert issue 95 ([5bea716](https://github.com/JamieMason/syncpack/commit/5bea7167d38be97dfca5f03dfdcd5070a5554c3d)), closes [#143](https://github.com/JamieMason/syncpack/issues/143) [#95](https://github.com/JamieMason/syncpack/issues/95)

## [10.7.2](https://github.com/JamieMason/syncpack/compare/10.6.1...10.7.2) (2023-07-02)

### Features

* **cli:** add command to update dependencies ([1c1be99](https://github.com/JamieMason/syncpack/commit/1c1be99461c9fccea66a3f0a0568b99577d1ffed))

### Bug Fixes

* **npm:** update dependencies ([bf432a7](https://github.com/JamieMason/syncpack/commit/bf432a7b75acf95e49d9faccb432fbdff176bb42))

## [10.6.1](https://github.com/JamieMason/syncpack/compare/10.5.1...10.6.1) (2023-06-18)

### Features

* **core:** refactor and general improvements ([f35c486](https://github.com/JamieMason/syncpack/commit/f35c486ab51f697f6259c509ebd7e0c575ad2dac)), closes [#140](https://github.com/JamieMason/syncpack/issues/140) [#139](https://github.com/JamieMason/syncpack/issues/139) [#132](https://github.com/JamieMason/syncpack/issues/132) [#111](https://github.com/JamieMason/syncpack/issues/111)

## [10.5.1](https://github.com/JamieMason/syncpack/compare/10.2.0...10.5.1) (2023-06-04)

### Features

* **cli:** add prompt to fix unsupported mismatches ([296fad5](https://github.com/JamieMason/syncpack/commit/296fad5b7ba29e5a1476285dad9320de199f4131))
* **format:** sort bin property alphabetically ([f7c87a8](https://github.com/JamieMason/syncpack/commit/f7c87a87557379612e638d621822430e0d8e27d6))
* **groups:** manage intersecting range versions ([96d6c6d](https://github.com/JamieMason/syncpack/commit/96d6c6d7837526c1e1dac603959b199d9d0b0f7d))

### Bug Fixes

* **cli:** add missing syncpack-lint binary ([ae265cb](https://github.com/JamieMason/syncpack/commit/ae265cbded2764f1f94ffad4e5fd528ed642add3))

## [10.2.0](https://github.com/JamieMason/syncpack/compare/10.1.0...10.2.0) (2023-06-03)

### Features

* **cli:** change output of lint command ([765376c](https://github.com/JamieMason/syncpack/commit/765376c17cf10422c349d76bb8335996046ea053)), closes [#134](https://github.com/JamieMason/syncpack/issues/134)

## [10.1.0](https://github.com/JamieMason/syncpack/compare/10.0.0...10.1.0) (2023-05-29)

### Features

* **cli:** add lint command ([2e3df1c](https://github.com/JamieMason/syncpack/commit/2e3df1c2178755faaf1116c910fd610b3bb43807)), closes [#3](https://github.com/JamieMason/syncpack/issues/3)

## [10.0.0](https://github.com/JamieMason/syncpack/compare/9.8.6...10.0.0) (2023-05-28)

### ⚠ BREAKING CHANGES

* **core:** - `fix-mismatches` will now exit with a status code of 1 if there are mismatches among unsupported versions which syncpack cannot auto-fix.
- Although they are still not auto-fixable, unsupported versions which were previously ignored are now acknowledged, which may introduce mismatches which previously would have been considered valid.
- This release was also a huge rewrite of Syncpack's internals and, while there is a large amount of tests, some scenarios may have been missed.
- If you run into any problems, please create an issue.

### Bug Fixes

* **core:** rewrite core architecture ([dc9355f](https://github.com/JamieMason/syncpack/commit/dc9355f987bd39588fb611d3152c5af9ee875cc4)), closes [#124](https://github.com/JamieMason/syncpack/issues/124) [#124](https://github.com/JamieMason/syncpack/issues/124) [#130](https://github.com/JamieMason/syncpack/issues/130) [#131](https://github.com/JamieMason/syncpack/issues/131) [#130](https://github.com/JamieMason/syncpack/issues/130) [#131](https://github.com/JamieMason/syncpack/issues/131) [#109](https://github.com/JamieMason/syncpack/issues/109) [#114](https://github.com/JamieMason/syncpack/issues/114) [#125](https://github.com/JamieMason/syncpack/issues/125) [#114](https://github.com/JamieMason/syncpack/issues/114) [#111](https://github.com/JamieMason/syncpack/issues/111) [#132](https://github.com/JamieMason/syncpack/issues/132) [#48](https://github.com/JamieMason/syncpack/issues/48) [#3](https://github.com/JamieMason/syncpack/issues/3)
* **npm:** update typescript ([2c5cd7f](https://github.com/JamieMason/syncpack/commit/2c5cd7f83cc46f0a95e4bfc09b80f6bd37181cae))

## [9.8.6](https://github.com/JamieMason/syncpack/compare/9.8.4...9.8.6) (2023-04-23)

### Bug Fixes

* **config:** prevent default source overriding rcfile ([1d6a4ba](https://github.com/JamieMason/syncpack/commit/1d6a4bab953ceee90461d8f85ece44bd384d9cd7)), closes [#123](https://github.com/JamieMason/syncpack/issues/123)
* **npm:** update minor dependencies ([91f4967](https://github.com/JamieMason/syncpack/commit/91f496767d834b7372565a094599ece935bb346d))

## [9.8.4](https://github.com/JamieMason/syncpack/compare/9.7.4...9.8.4) (2023-02-21)

### Features

* **semver:** support resolving with lowest version ([a17e423](https://github.com/JamieMason/syncpack/commit/a17e4234730b433777f320bb5ca35e8c46545832)), closes [#110](https://github.com/JamieMason/syncpack/issues/110)

## [9.7.4](https://github.com/JamieMason/syncpack/compare/9.3.2...9.7.4) (2023-02-19)

### Features

* **engines:** increase node from 10 to 14 ([603f058](https://github.com/JamieMason/syncpack/commit/603f0587a62d6d452c01bc24ec23827bc2fb582b))
* **groups:** handle long and multi-line labels ([ecc58ff](https://github.com/JamieMason/syncpack/commit/ecc58fff645f9639f003934c109bc5d17af7b3d6))
* **semver:** recognise ^6, >=5 etc as valid ([be637f0](https://github.com/JamieMason/syncpack/commit/be637f0349018b2b3d5f204613a4af187c8f7aa0)), closes [#122](https://github.com/JamieMason/syncpack/issues/122)
* **versionGroups:** add optional snapTo property ([fd0edb6](https://github.com/JamieMason/syncpack/commit/fd0edb6a25ec14aefdaf72f796bbe5d3c22d3692)), closes [#87](https://github.com/JamieMason/syncpack/issues/87)

### Bug Fixes

* **indent:** use value from config file ([aa31244](https://github.com/JamieMason/syncpack/commit/aa312448156c6814213502e5f98c2671d7a166a8))
* **npm:** update dependencies ([558d177](https://github.com/JamieMason/syncpack/commit/558d177026ca89606b534f3ce37958b80faa7b1a))

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

## [9.0.0](https://github.com/JamieMason/syncpack/compare/8.5.14...9.0.0) (2023-02-14)

### ⚠ BREAKING CHANGES

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

### Features

* **custom:** support custom version locations ([2cd34fd](https://github.com/JamieMason/syncpack/commit/2cd34fd1f41e949cedd28b901a123906d8bc5d08)), closes [#112](https://github.com/JamieMason/syncpack/issues/112) [#113](https://github.com/JamieMason/syncpack/issues/113)
* **fix-mismatches:** remove any empty objects ([a279e56](https://github.com/JamieMason/syncpack/commit/a279e56dfaf8ba11bd507a89315b2a5b038b027b))

## [8.5.14](https://github.com/JamieMason/syncpack/compare/8.4.11...8.5.14) (2023-02-07)

### Features

* **cli:** tidy log output and make it more consistent ([cb58070](https://github.com/JamieMason/syncpack/commit/cb58070e646d80acec893e72154a96a25ae8d8ca))

### Bug Fixes

* **npm:** update dependencies ([f2c0322](https://github.com/JamieMason/syncpack/commit/f2c0322ccb1724b566bf818c0a3b03ac3ed9a27e))
* **npm:** update dependencies ([3ec1361](https://github.com/JamieMason/syncpack/commit/3ec13613d1f7446d7798adabf33eb82101c51ee8))
* **output:** remove console.log in readConfigFileSync ([1c51366](https://github.com/JamieMason/syncpack/commit/1c51366a557f2a69e441c33f29d4927c9cdf88f5)), closes [#106](https://github.com/JamieMason/syncpack/issues/106)

## [8.4.11](https://github.com/JamieMason/syncpack/compare/8.4.10...8.4.11) (2022-12-01)

### Bug Fixes

* **semver-ranges:** fix regression in 8.4.10 {lint,set}-semver-ranges ([5bce1d8](https://github.com/JamieMason/syncpack/commit/5bce1d84e3018c08ec72459955de2631b3a3aabe))

## [8.4.10](https://github.com/JamieMason/syncpack/compare/8.3.9...8.4.10) (2022-11-23)

### Features

* **groups:** add support to ignore dependencies ([f96df8f](https://github.com/JamieMason/syncpack/commit/f96df8fe6faf80c28e02178f2dd23863290a1da6))

### Bug Fixes

* **lint-ranges:** remove empty groups from output ([fa6db49](https://github.com/JamieMason/syncpack/commit/fa6db490f63cc0e59889628c3f08cc1f92aa9fd5))

## [8.3.9](https://github.com/JamieMason/syncpack/compare/8.3.8...8.3.9) (2022-10-28)

### Bug Fixes

* **semver:** add support for setting '*' ([779772b](https://github.com/JamieMason/syncpack/commit/779772b039ad18aed84df560b13fa92b2a4ad741))

## [8.3.8](https://github.com/JamieMason/syncpack/compare/8.2.5...8.3.8) (2022-10-28)

### Features

* **config:** include config.syncpack of package.json ([40dcdce](https://github.com/JamieMason/syncpack/commit/40dcdcef00d66db0866882e2ef0138972199eea3)), closes [#86](https://github.com/JamieMason/syncpack/issues/86)

### Bug Fixes

* **npm:** update dependencies ([6311999](https://github.com/JamieMason/syncpack/commit/63119995d5ab7e98b867edad28bf1655eb96bbdf))
* **pnpm:** fix regression affecting --pnpmOverrides ([6a782f9](https://github.com/JamieMason/syncpack/commit/6a782f95b558b2ae2800f429b4135f306e7abbfb)), closes [#94](https://github.com/JamieMason/syncpack/issues/94)
* **set-semver-ranges:** handle 'workspace' type ([2134658](https://github.com/JamieMason/syncpack/commit/213465882ee6182bbc0f3ef00108cc562b63deae)), closes [#84](https://github.com/JamieMason/syncpack/issues/84)

## [8.2.5](https://github.com/JamieMason/syncpack/compare/8.2.4...8.2.5) (2022-10-28)

### Bug Fixes

* **format:** skip .repository when its .directory is present ([688bc0c](https://github.com/JamieMason/syncpack/commit/688bc0c29c09b4ea3e289f4584a6cc62adec936e)), closes [#91](https://github.com/JamieMason/syncpack/issues/91) [#93](https://github.com/JamieMason/syncpack/issues/93) [#100](https://github.com/JamieMason/syncpack/issues/100)

## [8.2.4](https://github.com/JamieMason/syncpack/compare/8.0.0...8.2.4) (2022-06-25)

### Features

* **eol:** detect line endings when writing ([7e61f5c](https://github.com/JamieMason/syncpack/commit/7e61f5cd6edbaab48cfe89b356bee1ac6bb4d442)), closes [#76](https://github.com/JamieMason/syncpack/issues/76)
* **output:** better explain mismatch reasons ([45e0cf1](https://github.com/JamieMason/syncpack/commit/45e0cf141337f692909b72de3edaf52b86f1ec42)), closes [#65](https://github.com/JamieMason/syncpack/issues/65) [#77](https://github.com/JamieMason/syncpack/issues/77) [#79](https://github.com/JamieMason/syncpack/issues/79)

### Bug Fixes

* **npm:** update dependencies ([1bdf0c7](https://github.com/JamieMason/syncpack/commit/1bdf0c756a369fc65987d04d5867a525e2e2248d))
* **npm:** update dependencies ([34c5d68](https://github.com/JamieMason/syncpack/commit/34c5d68194e7134305422f84a05d4774f812ef08))
* **output:** edit log output and colours ([3623c41](https://github.com/JamieMason/syncpack/commit/3623c41cb8bd16c4cd9186199140905847722b50))
* **workspace:** find missed mismatches against workspace versions ([25c1836](https://github.com/JamieMason/syncpack/commit/25c18363ebacbd5870e14d234e68cfb570ce913e)), closes [#66](https://github.com/JamieMason/syncpack/issues/66)

## [8.0.0](https://github.com/JamieMason/syncpack/compare/7.2.2...8.0.0) (2022-05-31)

### ⚠ BREAKING CHANGES

* **pnpm:** The `--overrides` option delivered in 6.0.0 was originally intended to
support pnpm, but erroneously read from the `.overrides` property of
package.json files and not `.pnpm.overrides`.

However, npm now also has an `.overrides` property to support the same
functionality for users of npm.

From this release, the `--overrides` option of syncpack now refers to
npm overrides. Pnpm users should change to using the new
`--pnpmOverrides` option instead.

### Features

* **pnpm:** add support for pnpm overrides ([2d1bf05](https://github.com/JamieMason/syncpack/commit/2d1bf059a239dff3af203104491a315b894b8796)), closes [#78](https://github.com/JamieMason/syncpack/issues/78)

## [7.2.2](https://github.com/JamieMason/syncpack/compare/7.2.1...7.2.2) (2022-05-29)

### Bug Fixes

* **windows:** normalise file paths ([c5e87c2](https://github.com/JamieMason/syncpack/commit/c5e87c2efe47bb538701ec3d83b813c47eddab8b)), closes [#66](https://github.com/JamieMason/syncpack/issues/66)

## [7.2.1](https://github.com/JamieMason/syncpack/compare/7.1.0...7.2.1) (2022-05-02)

### Features

* **versionGroups:** mark specific dependencies for removal ([e571775](https://github.com/JamieMason/syncpack/commit/e571775ebc120786b2742d0047dc8f79f7f8a539)), closes [#65](https://github.com/JamieMason/syncpack/issues/65)

### Bug Fixes

* **npm:** update dependencies ([b9a9f5d](https://github.com/JamieMason/syncpack/commit/b9a9f5da2dee72e8aea7ccebca7607c80cd8391e))

## [7.1.0](https://github.com/JamieMason/syncpack/compare/7.0.0...7.1.0) (2022-05-01)

### Features

* **cli:** add --config to specify path to config file ([4b19a13](https://github.com/JamieMason/syncpack/commit/4b19a1375a7856bffb50c9fda84a3a8b6def877b)), closes [#71](https://github.com/JamieMason/syncpack/issues/71) [#72](https://github.com/JamieMason/syncpack/issues/72)

## [7.0.0](https://github.com/JamieMason/syncpack/compare/6.2.1...7.0.0) (2022-04-27)

### ⚠ BREAKING CHANGES

* **cli:** If a package developed in your Monorepo depends on another package
developed in your Monorepo, syncpack will now fix the installed version
of the dependent to match the actual version from the package.json file
of the local package.

You can disable this functionality by setting `"workspace": false` in
your `.syncpackrc` config file, or by omitting the new `--workspace`
option when using `--dev`, `--prod` etc to define which dependency types
you wish to include.

### Features

* **cli:** sync versions of locally developed packages ([0367c9f](https://github.com/JamieMason/syncpack/commit/0367c9fe669172fad27d9a8fdf2125e3a5054c51)), closes [#66](https://github.com/JamieMason/syncpack/issues/66)

### Bug Fixes

* **npm:** update dependencies ([4e5a1cf](https://github.com/JamieMason/syncpack/commit/4e5a1cf484bcfcdab2eef6ded1558ddb51a49286))
* **npm:** update dependencies ([eebbcde](https://github.com/JamieMason/syncpack/commit/eebbcde479adefdbf0dee0f7560c8bc0952a1c03))

## [6.2.1](https://github.com/JamieMason/syncpack/compare/6.2.0...6.2.1) (2022-04-12)

### Bug Fixes

* **globs:** ignore node_modules ([8e11545](https://github.com/JamieMason/syncpack/commit/8e115451f2e9f08745a1ca53c03d502f8a21c2fb)), closes [#68](https://github.com/JamieMason/syncpack/issues/68) [#70](https://github.com/JamieMason/syncpack/issues/70)

## [6.2.0](https://github.com/JamieMason/syncpack/compare/6.1.0...6.2.0) (2022-01-03)

### Features

* **groups:** target specific dependency types ([565c1e7](https://github.com/JamieMason/syncpack/commit/565c1e76c71592dc0353266e1289dd9d0bf3fd9b))

## [6.1.0](https://github.com/JamieMason/syncpack/compare/6.0.0...6.1.0) (2022-01-03)

### Features

* **groups:** pin a version group to a specific version ([3de6f90](https://github.com/JamieMason/syncpack/commit/3de6f90752b04f8324c4e30823a39e009ef0587b)), closes [#44](https://github.com/JamieMason/syncpack/issues/44) [#53](https://github.com/JamieMason/syncpack/issues/53) [#63](https://github.com/JamieMason/syncpack/issues/63) [#64](https://github.com/JamieMason/syncpack/issues/64)

## [6.0.0](https://github.com/JamieMason/syncpack/compare/5.8.15...6.0.0) (2022-01-01)

### ⚠ BREAKING CHANGES

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

### Features

* **core:** add glob support and semver range rule groups ([787757c](https://github.com/JamieMason/syncpack/commit/787757c4b09163ec12a60b190954811c0cf4f15f))

### Bug Fixes

* **npm:** update dependencies ([fdef0a2](https://github.com/JamieMason/syncpack/commit/fdef0a202340e4287bf8f7e6ae27953002c5b4ee))

## [5.8.15](https://github.com/JamieMason/syncpack/compare/5.8.14...5.8.15) (2021-08-08)

### Bug Fixes

* **npm:** update dependencies ([aea1f37](https://github.com/JamieMason/syncpack/commit/aea1f374e4e039a35aec7b2e629b9607fd922c75))

## [5.8.14](https://github.com/JamieMason/syncpack/compare/5.8.12...5.8.14) (2021-08-01)

### Bug Fixes

* **cli:** apply breaking change from commander ([a61d384](https://github.com/JamieMason/syncpack/commit/a61d3845a55f0796defce7cafaaf29541a4ff07f))
* **format:** write files if only whitespace changes ([f38ea40](https://github.com/JamieMason/syncpack/commit/f38ea4092af8181acbc0281c775c0c84acb5be3d)), closes [#54](https://github.com/JamieMason/syncpack/issues/54)

## [5.8.12](https://github.com/JamieMason/syncpack/compare/5.7.11...5.8.12) (2021-08-01)

### Features

* **core:** add lint-semver-ranges command ([b4209f0](https://github.com/JamieMason/syncpack/commit/b4209f076344a9d59830d3bbd75569de9e19b4b3)), closes [#56](https://github.com/JamieMason/syncpack/issues/56)

### Bug Fixes

* **npm:** update dependencies ([91254f6](https://github.com/JamieMason/syncpack/commit/91254f6aa283afcc0b32163864468359dd4f888f))

## [5.7.11](https://github.com/JamieMason/syncpack/compare/5.6.10...5.7.11) (2021-01-29)

### Features

* **fix-mismatches:** use local package version when available ([640cb7f](https://github.com/JamieMason/syncpack/commit/640cb7faf18b33fd491e68f66d3cf599845c9265)), closes [#47](https://github.com/JamieMason/syncpack/issues/47)

### Bug Fixes

* **npm:** update dependencies ([5531da6](https://github.com/JamieMason/syncpack/commit/5531da60bc1cfb0fe3c5ca8e904d0a9e55d3d4b5))

## [5.6.10](https://github.com/JamieMason/syncpack/compare/5.6.7...5.6.10) (2020-09-17)

### Bug Fixes

* **cli:** use defaults when source is empty array ([c2f6199](https://github.com/JamieMason/syncpack/commit/c2f61998add60ed5d52af1c3518d1f737cf75c80))
* **core:** support multiple version groups ([bfd12b4](https://github.com/JamieMason/syncpack/commit/bfd12b4f3a6693ac1b4580621b12995d2b04eee7)), closes [#43](https://github.com/JamieMason/syncpack/issues/43)
* **list:** display mismatches from version groups ([43ba18d](https://github.com/JamieMason/syncpack/commit/43ba18dff1aa7c749724b992b6eef17a227f5445))

## [5.6.7](https://github.com/JamieMason/syncpack/compare/5.5.6...5.6.7) (2020-08-30)

### Features

* **core:** support granular versioning rules ([2197f90](https://github.com/JamieMason/syncpack/commit/2197f90608c119a04ddde6255e729fa1ec5c49ec)), closes [#41](https://github.com/JamieMason/syncpack/issues/41)

### Bug Fixes

* **npm:** update dependencies ([2e3ea3b](https://github.com/JamieMason/syncpack/commit/2e3ea3b0f6de8a97a390305a998053550183cc27))

## [5.5.6](https://github.com/JamieMason/syncpack/compare/5.2.5...5.5.6) (2020-08-23)

### Features

* **core:** expose format configuration ([4f74d9a](https://github.com/JamieMason/syncpack/commit/4f74d9a0b9a92428278f66327630e5b0e9dc5add)), closes [#30](https://github.com/JamieMason/syncpack/issues/30)
* **core:** sort resolutions field a-z ([f76a127](https://github.com/JamieMason/syncpack/commit/f76a1278b45ec3b00b2658b5da327d0a480ff12d)), closes [#34](https://github.com/JamieMason/syncpack/issues/34)
* **core:** support yarn workspaces config as object ([34eceaf](https://github.com/JamieMason/syncpack/commit/34eceaffae143fdbc9729495ea693172c2944351)), closes [#33](https://github.com/JamieMason/syncpack/issues/33)

### Bug Fixes

* **core:** ignore link: versions rather than throw ([7a48366](https://github.com/JamieMason/syncpack/commit/7a483666e64a046be9984bf4146ac8566b3f5920)), closes [#38](https://github.com/JamieMason/syncpack/issues/38)

## [5.2.5](https://github.com/JamieMason/syncpack/compare/5.1.4...5.2.5) (2020-08-22)

### Features

* **core:** add support for config files ([cfd5df3](https://github.com/JamieMason/syncpack/commit/cfd5df35134de068eaf26bdb2cfaa1890c6c3545))

### Bug Fixes

* **npm:** update dependencies ([19ad510](https://github.com/JamieMason/syncpack/commit/19ad510d09040e1aa098e16d6831836da3c9c12f))

## [5.1.4](https://github.com/JamieMason/syncpack/compare/5.0.3...5.1.4) (2020-08-02)

### Features

* **core:** add support for pnpm workspaces ([a6112ec](https://github.com/JamieMason/syncpack/commit/a6112ec786fd26699a3734707218cda38baf9f0e)), closes [#42](https://github.com/JamieMason/syncpack/issues/42)

### Bug Fixes

* **npm:** update dependencies ([f2cac6a](https://github.com/JamieMason/syncpack/commit/f2cac6a05eaf9f5a7736267a797cf75476292757))

## [5.0.3](https://github.com/JamieMason/syncpack/compare/5.0.1...5.0.3) (2020-06-19)

### Bug Fixes

* **format:** leave sort order of "files" array unchanged ([1bd584f](https://github.com/JamieMason/syncpack/commit/1bd584f67054d4a37b91b1e5f285dbe9b53b4489)), closes [#35](https://github.com/JamieMason/syncpack/issues/35)
* **npm:** update dependencies ([9e0bd7e](https://github.com/JamieMason/syncpack/commit/9e0bd7ea257b3dcc425f306d4fcae195f6d0d126))

## [5.0.1](https://github.com/JamieMason/syncpack/compare/4.5.5...5.0.1) (2020-02-16)

### ⚠ BREAKING CHANGES

* **npm:** engines.node has been increased to >=10 because
semver@7.1.1 is a hard dependency of syncpack and
requires node >=10

### Bug Fixes

* **core:** include root package.json when reading yarn & lerna config ([a7875cb](https://github.com/JamieMason/syncpack/commit/a7875cb08e0f8382163e8c9e8a4d3e6772b4c160))
* **npm:** update dependencies ([5fdcc7b](https://github.com/JamieMason/syncpack/commit/5fdcc7bd112533f891b31dfcf0be79b54989b8d7))

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

## [4.0.0](https://github.com/JamieMason/syncpack/compare/3.5.2...4.0.0) (2019-01-11)

### ⚠ BREAKING CHANGES

* **node:** Support Node.js 8.x or newer, Transitive Dependency ip-regex@3.0.0
supports node ">=8".

### Bug Fixes

* **node:** support Node.js 8.x or newer ([c71009e](https://github.com/JamieMason/syncpack/commit/c71009e1507cd66c735112a0ae685cd3e51ab2fe))
* **npm:** update dependencies ([23b02e3](https://github.com/JamieMason/syncpack/commit/23b02e3d72e51e8b069a336357e6cddcdc4979c1)), closes [#15](https://github.com/JamieMason/syncpack/issues/15)

## [3.5.2](https://github.com/JamieMason/syncpack/compare/3.5.0...3.5.2) (2019-01-07)

### Bug Fixes

* **core:** improve handling of non-semver versions ([9e1176a](https://github.com/JamieMason/syncpack/commit/9e1176a3495ea97648c61ab5869a12c3ff539c5f)), closes [#14](https://github.com/JamieMason/syncpack/issues/14)
* **npm:** update dependencies ([09d9f04](https://github.com/JamieMason/syncpack/commit/09d9f04480252edd0fd3b6af3cd8dce36c66d96b))

## [3.5.0](https://github.com/JamieMason/syncpack/compare/3.4.0...3.5.0) (2018-10-29)

### Features

* **cli:** improve --help output and examples ([dfe6274](https://github.com/JamieMason/syncpack/commit/dfe6274c50d6ba3ea3ec419cabd1ccf0bb73f8fb))

## [3.4.0](https://github.com/JamieMason/syncpack/compare/3.3.0...3.4.0) (2018-10-28)

### Features

* **cli:** read sources from lerna.json if present ([77b90eb](https://github.com/JamieMason/syncpack/commit/77b90eb3d656c50ff7b9d1317dc2cdad469b15a5)), closes [#11](https://github.com/JamieMason/syncpack/issues/11)

## [3.3.0](https://github.com/JamieMason/syncpack/compare/3.0.0...3.3.0) (2018-10-28)

### Features

* **cli:** specify dependency types as options ([ec5ef6b](https://github.com/JamieMason/syncpack/commit/ec5ef6b76f3c2fa0fba0f3a364b734f554d32c8a)), closes [#10](https://github.com/JamieMason/syncpack/issues/10)
* **cli:** specify indentation as option ([8b408bd](https://github.com/JamieMason/syncpack/commit/8b408bd14768fe7b3a2fd5cbb06233ba3b9707b3)), closes [#12](https://github.com/JamieMason/syncpack/issues/12)
* **format:** sort contributors alphabetically ([935ffcf](https://github.com/JamieMason/syncpack/commit/935ffcf307d0adabe06c04ff1e2b258277f060be))

### Performance Improvements

* **npm:** move [@types](https://github.com/types) to devDependencies ([ad5951c](https://github.com/JamieMason/syncpack/commit/ad5951ceba183761b0b73355a508111e7eb02508)), closes [#13](https://github.com/JamieMason/syncpack/issues/13)

## [3.0.0](https://github.com/JamieMason/syncpack/compare/2.0.1...3.0.0) (2018-08-25)

### ⚠ BREAKING CHANGES

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

### Features

* **bin:** override package locations using repeatable --source options ([5dbcfd4](https://github.com/JamieMason/syncpack/commit/5dbcfd4915cf286cba0e665e554c319d717f6651))
* **list-mismatches:** return exit code on finding mismatches ([06958c6](https://github.com/JamieMason/syncpack/commit/06958c6446646c108fc1dc4e07c714cd08bf58fc))

## [2.0.1](https://github.com/JamieMason/syncpack/compare/2.0.0...2.0.1) (2018-04-29)

### Bug Fixes

* **core:** ensure pattern overrides are read ([7513ba5](https://github.com/JamieMason/syncpack/commit/7513ba5fa644bf445efbdca22d4797b4a973b56f))

## [2.0.0](https://github.com/JamieMason/syncpack/compare/1.3.2...2.0.0) (2018-04-29)

### ⚠ BREAKING CHANGES

* **core:** --packages option replaced with variadic arguments

### Features

* **core:** support multiple glob patterns ([a2b5af0](https://github.com/JamieMason/syncpack/commit/a2b5af017a2152fb40d0522501db64ef739fe5f9)), closes [#5](https://github.com/JamieMason/syncpack/issues/5) [#6](https://github.com/JamieMason/syncpack/issues/6)

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

## [1.0.0](https://github.com/JamieMason/syncpack/compare/0.3.1...1.0.0) (2018-02-02)

### ⚠ BREAKING CHANGES

* **core:** The previous commands have been replaced.

### Features

* **core:** add fix-mismatches command ([4793f1f](https://github.com/JamieMason/syncpack/commit/4793f1fc6b67cfa1f87f73188944f8dd8d196bc0))
* **core:** add list command ([3b29176](https://github.com/JamieMason/syncpack/commit/3b291760f4cba611acc3f75034679303c55bf1a7))
* **core:** add list-mismatches command ([735ad2b](https://github.com/JamieMason/syncpack/commit/735ad2b2a1347b99a3f758b0c797b2fb7a3fc4c3))
* **core:** update command line API ([de8dcb2](https://github.com/JamieMason/syncpack/commit/de8dcb2b0dbe7bb63c91aeb05e8422696b0bd178))

### Bug Fixes

* **core:** correctly check a file is package.json ([d1da609](https://github.com/JamieMason/syncpack/commit/d1da6096c3b7c6b01a05c112ffc1251ec4ba700d))
* **core:** handle missing dependency maps ([372aa68](https://github.com/JamieMason/syncpack/commit/372aa6877f47df1118c45931391c8b87ca851413))
* **core:** handle semver ranges containing 1.x.x ([a0f8f56](https://github.com/JamieMason/syncpack/commit/a0f8f5650f3855361914fc6f8303035dc3abfb8d))

## [0.3.1](https://github.com/JamieMason/syncpack/compare/0.3.0...0.3.1) (2017-08-23)

### Bug Fixes

* **copy-values:** write results to disk ([a641de4](https://github.com/JamieMason/syncpack/commit/a641de41faaf6851cf9177ff87acd0d3e16494fb))

## [0.3.0](https://github.com/JamieMason/syncpack/compare/0.2.1...0.3.0) (2017-08-22)

### Features

* **cli:** add copy-values command ([b51a2c9](https://github.com/JamieMason/syncpack/commit/b51a2c96e133a1b5020577cf3c6bef31e79de850))

## [0.2.1](https://github.com/JamieMason/syncpack/compare/0.2.0...0.2.1) (2017-08-20)

### Bug Fixes

* **core:** update dependencies, fix lint warnings ([a65eef7](https://github.com/JamieMason/syncpack/commit/a65eef765d868a27913e173543dcbda43a2202a5))

## [0.2.0](https://github.com/JamieMason/syncpack/compare/0.1.0...0.2.0) (2017-08-20)

### Features

* **sync:** synchronise versions across multiple package.json ([7d5848a](https://github.com/JamieMason/syncpack/commit/7d5848a0edbe0c0a312be323cc8d9a4a8ed0ea30))

## [0.1.0](https://github.com/JamieMason/syncpack/compare/f6dada7aae149b7d0299206308347c8497e249d0...0.1.0) (2017-08-18)

### Features

* **cli:** create scaffold cli ([f6dada7](https://github.com/JamieMason/syncpack/commit/f6dada7aae149b7d0299206308347c8497e249d0))
