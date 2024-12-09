# infinite-rs Changelog

## 0.6.2 - 2024-12-10
- Made some fields in `TagHeader` public.
- Added example `load_scripts`.

## 0.6.1 - 2024-12-03
- Added `PartialEq` and `Eq` implementations for `DataOffsetType`, `FieldEntryFlags` and `TagSectionType`.

## 0.6.0 - 2024-11-29
- FIX: HD1 tags are now properly identified.
- Multiple fields in `ModuleFileEntry` have been made public.
- Added `use_hd1` field to `ModuleFile`
- Removed `read_resources` from `ModuleFile`.

## 0.5.3 - 2024-11-22
- Internal: Each block does not create its own `BufReader` anymore.
- Fixed major issue affecting `FieldBlock` reads.

## 0.5.2 - 2024-11-21
- All tuple-like field types in `common_types` are now public.

## 0.5.1 - 2024-11-21
- Fixed docs.rs issues.

## 0.5.0 - 2024-11-21
- Many documentation items have been improved and inner links added
- Internal: `Readable` has been renamed to `Enumerable`
- Internal: Removed unnecessary trait bounds
- A `ModuleFile` can now be instantiated using `from_path`
- `Error` is now a crate-level export.
- Added support for enums and bitflags for common types
- Removed unused common types
- Many primitive common types now are tuple-like structs
- Reduced allocations with `read_enumerables` and Kraken decompressor
- Kraken decompressor has been vendored, now does not include large `oodle.txt` file.
- Updated dependencies.

## 0.4.2 - 2024-11-19
-  `module.read()` now supports any `AsRef<Path>` as a filepath argument
- The `datablock` and `structs` fields in `TagFile` have been renamed to `datablock_definitions` and `struct_definitions`.
- In `ModuleFile`, the `file_path` field has been removed, and `resources` renamed to `resource_indices`.
- Warning about `ds` modules has been removed.
- `AnyTag` now reads its contents directly

## 0.4.1 - 2024-11-19
- Fixed Kraken decompressor not working on windows
- Added new error types for type conversions
- Added github CI.

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
