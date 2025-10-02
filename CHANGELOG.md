# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0](https://github.com/spotpilot/grid-tariffs/compare/grid-tariffs-v0.4.2...grid-tariffs-v0.5.0) - 2025-10-02

### Fixed

- Feed-in revenue info wasn't being added...
- Don't allow None-values for TransferFeeSimplified
- [**breaking**] Feed-in revenue simplified info + format change
- Rename module to feed_in_revenue

## [0.4.2](https://github.com/spotpilot/grid-tariffs/compare/grid-tariffs-v0.4.1...grid-tariffs-v0.4.2) - 2025-10-01

### Fixed

- rename_all, not rename

## [0.4.1](https://github.com/spotpilot/grid-tariffs/compare/grid-tariffs-v0.4.0...grid-tariffs-v0.4.1) - 2025-10-01

### Fixed

- Didn't support passing in English as language

## [0.4.0](https://github.com/spotpilot/grid-tariffs/compare/grid-tariffs-v0.3.0...grid-tariffs-v0.4.0) - 2025-10-01

### Added

- Add info translations for simplified structs
- *(SE Hoganas)* Add basic info ([#127](https://github.com/spotpilot/grid-tariffs/pull/127))

### Fixed

- [**breaking**] Don't use tagged serde enums - problem to use with openapi-generator

### Other

- Update package name in CI as well

## [0.3.0](https://github.com/spotpilot/grid-tariffs/compare/grid-tariffs-v0.2.0...grid-tariffs-v0.3.0) - 2025-09-16

### Added

- [**breaking**] PowerTariffSimplified + some breaking serde / jsonschema changes

### Fixed

- Add JSON Schema printing

### Other

- Re-organize a bit

## [0.2.0](https://github.com/spotpilot/grid-tariffs/compare/grid-tariffs-v0.1.0...grid-tariffs-v0.2.0) - 2025-09-15

### Fixed

- [**breaking**] Use symmetric FromStr and Display values

### Other

- release-plz
