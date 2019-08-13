# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.3.0](https://github.com/jsonprima/jsonprima/compare/v0.2.0...v0.3.0) (2019-08-13)


### Features

* catch duplicate object names as errors ([9bee3d7](https://github.com/jsonprima/jsonprima/commit/9bee3d7))



## [0.2.0](https://github.com/jsonprima/jsonprima/compare/v0.1.1...v0.2.0) (2019-07-23)


### Features

* allow arrays to be nested ([6e5a733](https://github.com/jsonprima/jsonprima/commit/6e5a733))
* allow false as array element ([732c332](https://github.com/jsonprima/jsonprima/commit/732c332))
* allow null as array element ([5337d2f](https://github.com/jsonprima/jsonprima/commit/5337d2f))
* allow number as array element ([d8aa10d](https://github.com/jsonprima/jsonprima/commit/d8aa10d))
* allow string as array element ([89178d7](https://github.com/jsonprima/jsonprima/commit/89178d7))
* allow true as array element ([0e49d92](https://github.com/jsonprima/jsonprima/commit/0e49d92))
* check for invalid characters in JSON document ([a971ce7](https://github.com/jsonprima/jsonprima/commit/a971ce7))
* export Error struct to public interface ([b7b6b18](https://github.com/jsonprima/jsonprima/commit/b7b6b18))
* ignore Byte Order Mark if present ([15be546](https://github.com/jsonprima/jsonprima/commit/15be546))
* implement Tokens struct ([afbe98b](https://github.com/jsonprima/jsonprima/commit/afbe98b))
* return serialized results from CLI tool as JSON array ([0950912](https://github.com/jsonprima/jsonprima/commit/0950912))
* validate array JSON value as root value ([ba331ff](https://github.com/jsonprima/jsonprima/commit/ba331ff))
* validate begin-array ([cf0a10d](https://github.com/jsonprima/jsonprima/commit/cf0a10d))
* validate false JSON value as root value ([a76c059](https://github.com/jsonprima/jsonprima/commit/a76c059))
* validate insignificant whitespace characters ([cf53390](https://github.com/jsonprima/jsonprima/commit/cf53390))
* validate null JSON value as root value ([0398653](https://github.com/jsonprima/jsonprima/commit/0398653))
* validate number JSON value as root value ([6a50b62](https://github.com/jsonprima/jsonprima/commit/6a50b62))
* validate object ([c3c3ef4](https://github.com/jsonprima/jsonprima/commit/c3c3ef4))
* validate string JSON value as root value ([da74f8e](https://github.com/jsonprima/jsonprima/commit/da74f8e))
* validate true JSON value as root value ([3e248d4](https://github.com/jsonprima/jsonprima/commit/3e248d4))
* validate value-separator ([cb7ed54](https://github.com/jsonprima/jsonprima/commit/cb7ed54))


### Tests

* add missing tests on array ([525a85a](https://github.com/jsonprima/jsonprima/commit/525a85a))
* add test to validate root value false after true ([5045a35](https://github.com/jsonprima/jsonprima/commit/5045a35))
* add test to validate root value null after false ([3cb03d5](https://github.com/jsonprima/jsonprima/commit/3cb03d5))
* add test to validate root value null after true ([c443df2](https://github.com/jsonprima/jsonprima/commit/c443df2))
* add test to validate root value number after false ([b2a0746](https://github.com/jsonprima/jsonprima/commit/b2a0746))
* add test to validate root value number after null ([146f8e7](https://github.com/jsonprima/jsonprima/commit/146f8e7))
* add test to validate root value number after true ([ea87208](https://github.com/jsonprima/jsonprima/commit/ea87208))
* add test to validate root value string after false ([a801f65](https://github.com/jsonprima/jsonprima/commit/a801f65))
* add test to validate root value string after null ([7e88d96](https://github.com/jsonprima/jsonprima/commit/7e88d96))
* add test to validate root value string after number ([8ecf3b1](https://github.com/jsonprima/jsonprima/commit/8ecf3b1))
* add test to validate root value string after true ([380eb79](https://github.com/jsonprima/jsonprima/commit/380eb79))
* add trailing comma tests on array ([362f5d6](https://github.com/jsonprima/jsonprima/commit/362f5d6))
* implement test helper macro ([7a01180](https://github.com/jsonprima/jsonprima/commit/7a01180))



### [0.1.1](https://github.com/jsonprima/jsonprima/compare/v0.1.0...v0.1.1) (2019-07-05)


### Bug Fixes

* fix release:publish script ([984f07d](https://github.com/jsonprima/jsonprima/commit/984f07d)), closes [#3](https://github.com/jsonprima/jsonprima/issues/3)



## 0.1.0 (2019-07-05)


### Bug Fixes

* fix error on .travis.yml format ([ce231de](https://github.com/jsonprima/jsonprima/commit/ce231de))


### Build System

* add a release script ([0612541](https://github.com/jsonprima/jsonprima/commit/0612541))
* add bump script ([c689e08](https://github.com/jsonprima/jsonprima/commit/c689e08))
* add commitizen ([f14215b](https://github.com/jsonprima/jsonprima/commit/f14215b))
* add pre commit hook ([b3aa9e8](https://github.com/jsonprima/jsonprima/commit/b3aa9e8))
* fix breaking format script ([28e8e05](https://github.com/jsonprima/jsonprima/commit/28e8e05))
* require signed commits and tags on releases ([b0d0cd1](https://github.com/jsonprima/jsonprima/commit/b0d0cd1))


### Features

* create the public API of this package ([c53cb52](https://github.com/jsonprima/jsonprima/commit/c53cb52))
* introduce a binary file for CLI integration ([b471248](https://github.com/jsonprima/jsonprima/commit/b471248))
