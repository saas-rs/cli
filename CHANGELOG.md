# SaaS RS :: CLI Changelog

## [Unreleased]
### Added
- [#40](https://github.com/saas-rs/cli/issues/40) Support looking up a generation feature by name

### Fixed
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
