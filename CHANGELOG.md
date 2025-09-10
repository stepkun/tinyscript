# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html),
especially the [Rust flavour](https://doc.rust-lang.org/cargo/reference/semver.html).

## [Schema] - 2025-??-??

### Added

### Changed

### Fixed

### Removed

## [0.4.0] - 2025-??-??

### Added
- `from` && `try_from` conversions

### Changed

### Fixed

### Removed

## [0.3.1] - 2025-09-08

### Fixed
- embedded

## [0.3.0] - 2025-09-07

### Added
- `cargo vet` files

### Changed
- Errorhandling reworked, it no longer uses `thiserror`
- public `ScriptingValue::as_bool(...)` replaced with `bool::try_from(...)`

### Removed
- several unused errors

## [0.2.1] - 2025-09-03

### Changed
- replaced `expect(SHOULD_NOT_HAPPEN)` with better error handling

### Fixed
- benchmarks

## [0.2.0] - 2025-08-29

### Added
- error handling necessary for interacting with `Environment`s

### Changed
- contribution modalities
- moved `Environment` into own submodule

### Fixed
- solved open todo!()'s
- non static lifetime for enum registration function

## [0.1.4] - 2025-08-11

### Changed
- function 'clear()' always available

## [0.1.3] - 2025-08-08

### Added
- added function to fetch discriminant of a registered enum value

## [0.1.2] - 2025-08-05

### Added
- embedded environment

### Changed
- replaced 'parking_lot' with 'spin'

## [0.1.1] - 2025-07-29

### Added
- Usage eample in README
- build profiles

### Changed
- centralized literales
- updated dependencies

### Fixed
- links to documentation

## [0.1.0] - 2025-07-19

Version 0.1.0 focussed on implementation of the core language.
Usage and exchange wwith the outside world is a very rough design, that will change.
