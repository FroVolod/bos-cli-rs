# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.20](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.19...v0.3.20) - 2024-12-20

### Other

- restored to default dist workflow config (#107)

## [0.3.19](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.18...v0.3.19) - 2024-12-19

### Other

- updates near-* dependencies to 0.28 release (#105)

## [0.3.18](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.17...v0.3.18) - 2024-11-19

### Other

- updates near-* dependencies to 0.27 release ([#104](https://github.com/bos-cli-rs/bos-cli-rs/pull/104))
- Updated near-* dependencies to 0.26 release ([#102](https://github.com/bos-cli-rs/bos-cli-rs/pull/102))

## [0.3.17](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.16...v0.3.17) - 2024-09-03

### Added
- Added TEACH-ME mode (activate it with `bos --teach-me`) ([#101](https://github.com/bos-cli-rs/bos-cli-rs/pull/101))

### Other
- devcontainer and deployment e2e test ([#99](https://github.com/bos-cli-rs/bos-cli-rs/pull/99))

## [0.3.16](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.15...v0.3.16) - 2024-08-21

### Other
- Updated near-* dependencies to the latest versions (nearcore crates 0.24.1 and near-cli-rs 0.14.1) ([#98](https://github.com/bos-cli-rs/bos-cli-rs/pull/98))
- updated deps to 0.23 version ([#96](https://github.com/bos-cli-rs/bos-cli-rs/pull/96))

## [0.3.15](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.14...v0.3.15) - 2024-06-21

### Fixed
- Brings support for deploying nested folders on Windows (account for \ path delimiter) ([#94](https://github.com/bos-cli-rs/bos-cli-rs/pull/94))
- Updated the starter project template to fix links in `npm run dev`

## [0.3.14](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.13...v0.3.14) - 2024-05-28

### Added
- Upgraded the new project template to fix multiple imports and exports in a single file and introduce Continuous Deployment workflows

### Fixed
- Fixed a syntax error in CI (publish-to-npm.yml)

## [0.3.13](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.12...v0.3.13) - 2024-02-03

### Other
- Updated binary releases pipeline to use cargo-dist v0.9.0 (previously v0.7.2) ([#91](https://github.com/bos-cli-rs/bos-cli-rs/pull/91))
- Fixed NPM_PACKAGE_NAME configuration in publish-to-npm.yml
- Enable manual triggering for publish-to-npm.yml

## [0.3.12](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.11...v0.3.12) - 2024-01-30

### Other
- Create releases using a private access token to trigger downstream workflows (publish-to-npm)

## [0.3.11](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.10...v0.3.11) - 2024-01-23

### Other
- Upgraded NEAR crates to 0.20.0 release ([#88](https://github.com/bos-cli-rs/bos-cli-rs/pull/88))
- Updated binary releases pipeline to use cargo-dist v0.7.2 (previously v0.1.0-prerelease.3)  ([#87](https://github.com/bos-cli-rs/bos-cli-rs/pull/87))
- Added the documentation for `--social-db-folder` option in components subcommand ([#85](https://github.com/bos-cli-rs/bos-cli-rs/pull/85))

## [0.3.10](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.9...v0.3.10) - 2024-01-16

### Added
- Updated new project template ot the latest version

## [0.3.9](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.8...v0.3.9) - 2024-01-15

### Added
- Added ability to change social-db folder where components are get and set ([#82](https://github.com/bos-cli-rs/bos-cli-rs/pull/82))

## [0.3.8](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.7...v0.3.8) - 2024-01-11

### Other
- Added automatic publishing to npmjs ([#81](https://github.com/bos-cli-rs/bos-cli-rs/pull/81))
- Changed NearBalance to NearToken. ([#77](https://github.com/bos-cli-rs/bos-cli-rs/pull/77))

## [0.3.7](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.6...v0.3.7) - 2023-12-19

### Added
- New command to initialize a new BOS project ([#69](https://github.com/bos-cli-rs/bos-cli-rs/pull/69))
- Added self-update ([#67](https://github.com/bos-cli-rs/bos-cli-rs/pull/67))

### Fixed
- Updated installation instructions ([#76](https://github.com/bos-cli-rs/bos-cli-rs/pull/76))

## [0.3.6](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.5...v0.3.6) - 2023-10-22

### Added
- A new command for diff-ing the widgets code ([#56](https://github.com/bos-cli-rs/bos-cli-rs/pull/56))
- New command to manage BOS profile in SocialDB ([#61](https://github.com/bos-cli-rs/bos-cli-rs/pull/61))

### Other
- Updating components commands ([#66](https://github.com/bos-cli-rs/bos-cli-rs/pull/66))
- Update input_account_id() ([#65](https://github.com/bos-cli-rs/bos-cli-rs/pull/65))
- Adds dependencies for compiling from source on Fedora Linux to the README ([#63](https://github.com/bos-cli-rs/bos-cli-rs/pull/63))

## [0.3.5](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.4...v0.3.5) - 2023-08-05

### Fixed
- Fixed `components download` command for accounts with 30+ components ([#59](https://github.com/bos-cli-rs/bos-cli-rs/pull/59))

## [0.3.4](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.3...v0.3.4) - 2023-06-27

### Other
- update Rust to 1.70.0 on release binaries pipeline ([#53](https://github.com/bos-cli-rs/bos-cli-rs/pull/53))

## [0.3.3](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.2...v0.3.3) - 2023-06-27

### Added
- Create deploy-testnet.yml for BOS apps to use in their CI pipelines ([#50](https://github.com/bos-cli-rs/bos-cli-rs/pull/50))
- New commands to manage data in SocialDB ([#38](https://github.com/bos-cli-rs/bos-cli-rs/pull/38))

### Fixed
- Fixed funcion key permission check to be less restrictive ([#48](https://github.com/bos-cli-rs/bos-cli-rs/pull/48))

### Other
- Feature community-maintained homebrew installation ([#52](https://github.com/bos-cli-rs/bos-cli-rs/pull/52))
- Update "near-cli-rs" dependency to version 0.5 ([#49](https://github.com/bos-cli-rs/bos-cli-rs/pull/49))
- Clarify the function-call-only access keys usage for restricted components deployment ([#47](https://github.com/bos-cli-rs/bos-cli-rs/pull/47))
- Added release-plz.toml to let cargo-dist to create GitHub releases

## [0.3.2](https://github.com/bos-cli-rs/bos-cli-rs/compare/v0.3.1...v0.3.2) - 2023-05-24

### Added
- New command to delete components (#35)
- New command to grant access permission to SocialDB keys (#34)

### Fixed
- Trimmed extra space at the beginning of a line in interactive queries (#39) (#40)
- fixed consts

### Other
- Added test and release-plz workflows to CI (#41)
- Fixed the link in the GitHub Actions snippet to point to `master` branch
- Merge pull request #33 from FroVolod/update-near-library
- rustup update: 1.68.2
- updated near-cli-rs
- Updated reusable GitHub Actions Workflow to use bos-cli 0.3.1 version by default

## [0.3.1] - 2023-04-28

Fixes:
* Support large codebases

## [0.3.0] - 2023-04-20

Breaking changes:
* Renamed CLI to bos (crate name is bos-cli, and repo name is bos-cli-rs)
* Restructured the commands

## [0.2.4] - 2023-03-02

* Upgraded cargo-dist to 0.0.4-prelease.2 to reduce the Linux release binary size

## [0.2.3] - 2023-02-27

* Allow `null`-able tags in the widgets metadata to be able remove the tags on deployment

## [0.2.2] - 2023-02-18

* Added Apple M1/M2 binary release support (ARM64 Darwin target)

## [0.2.1] - 2023-02-18

* Reverted an accidental breaking change of renaming `network-config` to `network-for-transaction` subcommand

## [0.2.0] - 2023-02-17

* **BREAKING CHANGE:** Improved access keys handling in `deploy` command (allow using zero deposit for write-granted access), so now you can have a signer account ID that is different from the account ID where you want to deploy the widgets

## [0.1.1] - 2023-02-02

* Added shell script installers via cargo-dist

## [0.1.0] - 2023-02-02

* Initial version with only two commands: `deploy` and `download`
