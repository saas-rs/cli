# SaaS RS :: CLI Changelog

## [0.3.5] - 2025-09-19
### Changed
- [#78](https://github.com/saas-rs/cli/issues/78) Upgrade Rust from 1.86.0 → 1.88.0

## [0.2.9] - 2025-08-01
### Added
- [#76](https://github.com/saas-rs/cli/issues/76) Support enabling new Postgres storage provider

## [0.2.8] - 2025-07-25
### Added
- [#74](https://github.com/saas-rs/cli/issues/74) When generating a service, support a "without authentication" flag
- [#72](https://github.com/saas-rs/cli/issues/72) Support enabling LocalFileSystem and S3 storage providers

### Changed
- [#70](https://github.com/saas-rs/cli/issues/70) Introduce an "enable" command to de-hyphenate some existing long commands

### Fixed
- [#68](https://github.com/saas-rs/cli/issues/68) The generate feature subcommand fails to require an id or name arg

## [0.2.7] - 2025-07-18
### Added
- [#60](https://github.com/saas-rs/cli/issues/60) Add a generator to add use of one of the supported identity providers
- [#58](https://github.com/saas-rs/cli/issues/58) Add a generator to add use of one of the supported storage adapters

### Changed
- [#67](https://github.com/saas-rs/cli/issues/67) Update crate homepage to point to new documentation site docs.saas-rs.com
- [#65](https://github.com/saas-rs/cli/issues/65) Upgrade 3 crates

## [0.2.6] - 2025-07-11
### Added
- [#53](https://github.com/saas-rs/cli/issues/53) Support reading an API Key from an environment variable

### Changed
- [#51](https://github.com/saas-rs/cli/issues/51) GenerationFeature has been renamed to Generator

### Fixed
- [#56](https://github.com/saas-rs/cli/issues/56) Creating a git snapshot is missing detection of non-zero exit status

## [0.2.5] - 2025-07-04
### Added
- [#49](https://github.com/saas-rs/cli/issues/49) Add a history subcommand to list mutate actions taken
- [#47](https://github.com/saas-rs/cli/issues/47) Support initializing a git repo using a subdirectory for a multi-language MonoRepo

## [0.2.3] - 2025-06-19
### Added
- [#40](https://github.com/saas-rs/cli/issues/40) Support looking up a generation feature by name
- [#39](https://github.com/saas-rs/cli/issues/39) Support creating an issue

### Changed
- [#45](https://github.com/saas-rs/cli/issues/45) Upgrade 4 crates

### Fixed
- [#42](https://github.com/saas-rs/cli/issues/42) GitHub security alert digest says remove_dir_all has a vulnerability
- [#22](https://github.com/saas-rs/cli/issues/22) version subcommand panics when installed via cargo

## [0.1.9] - 2025-06-13
### Added
- [#30](https://github.com/saas-rs/cli/issues/30) Support listing all features available for generation
- [#29](https://github.com/saas-rs/cli/issues/29) Support generation of a feature

### Fixed
- [#32](https://github.com/saas-rs/cli/issues/32) The archive uploaded in a generate request should not be buffered in memory
- [#31](https://github.com/saas-rs/cli/issues/31) The archive downloaded from a generate response should not be buffered in memory

## [0.1.8] - 2025-06-06
### Changed
- [#27](https://github.com/saas-rs/cli/issues/27) Upgrade 2 crates, and protocol
- [#25](https://github.com/saas-rs/cli/issues/25) In the README, document how to retry a failed login with Chrome

## [0.1.6] - 2025-05-23
### Added
- [#19](https://github.com/saas-rs/cli/issues/19) Suggest invoking make after a successful initialize or generate command leaves the workspace dirty

### Changed
- [#23](https://github.com/saas-rs/cli/issues/23) Upgrade 4 crates

## [0.1.5] - 2025-05-17
### Added
- [#15](https://github.com/saas-rs/cli/issues/15) When logging in, support requesting a specific browser
- [#11](https://github.com/saas-rs/cli/issues/11) When logging in, support Console URL and API URL overrides

### Changed
- [#17](https://github.com/saas-rs/cli/issues/17) Upgrade 8 crates

### Fixed
- [#12](https://github.com/saas-rs/cli/issues/12) Services list ps format references non-existent displayName field and fails to reference description
- [#9](https://github.com/saas-rs/cli/issues/9) Crate fails to hyperlink to this Git repo

## [0.1.4] - 2025-05-10
### Added
- [#4](https://github.com/saas-rs/cli/issues/4) Provide a version subcommand
- [#3](https://github.com/saas-rs/cli/issues/3) Document installation

### Changed
- [#7](https://github.com/saas-rs/cli/issues/7) Debian APT packaging metadata is obsolete, now that publishing is to crates.io

## [0.1.3] - 2025-05-04
### Added
- [#1](https://github.com/saas-rs/cli/issues/1) Initial public release
