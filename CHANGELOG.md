# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased] - ReleaseDate
- Add docker_build module [#103](https://github.com/svenstaro/genact/pull/103) (thanks @Kovah)

## [0.10.0] - 2020-08-27
- Add `--exit-after-modules` options which can be used to make genact exit after running that many modules

## [0.9.0] - 2020-08-10
- Add global speed [#132](https://github.com/svenstaro/genact/issues/132)

## [0.8.1] - 2020-08-10
- Refactor entrypoint, modules, and file structure
- Use structopt instead of clap which should give better errors
- Improve wasm download size

## [0.8.0] - 2020-08-05
- Enhance download module
- Rewrite web version using wasm-bindgen

<!-- next-url -->
[Unreleased]: https://github.com/svenstaro/genact/compare/v0.10.0...HEAD
[0.10.0]: https://github.com/svenstaro/genact/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/svenstaro/genact/compare/v0.8.1...v0.9.0
[0.8.1]: https://github.com/svenstaro/genact/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/svenstaro/proby/compare/0.7.0...v0.8.0
