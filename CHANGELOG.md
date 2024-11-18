# infinite-rs Changelog

## 0.4.0 - 2024-11-18
- Kraken decompressor can now build and link on linux
- Added support for loading custom tag structures
- Added new derive macro crate to generate tag structures
- Field blocks can now be read properly
- Miscellanous changes and improvements.

## 0.3.1 - 2024-10-21
- Resource tags can now be read using ModuleFile::read_tag.

## 0.3.0 - 2024-10-21
- Reworked build script to run on linux
- Updated dependencies
- Added `serde` feature to enable serialization for custom structs
- Implemented proper error types instead of using `anyhow`
- Changed `read_fixed_string` to not allow non-UTF8 characters
- Added visibility modifiers to most structs and functions
- Made public API more concise
- Updated documentation
- Changed modules to not do a syscall by keeping file stream in memory
- Added support for HD1 modules
- Removed `types` module, providing an example in `load_all_modules` instead.
## 0.2.2 - 2024-09-09
- Minor changes, implemented `Readable` type for other enumerables.
## 0.2.1 - 2024-09-09
- Added `read_enumerables` for BufReaderExt
- Updated documentation.
## 0.2.0 - 2024-09-05
- Fixed module items not being read correctly.
## 0.1.0 - 2024-08-26
- Initial Release
