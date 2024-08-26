infinite-rs
===
Simple and fast deserialization library for Halo Infinite.

Provides tools to load module files, read and decompress tags within them. Translates built-in types into rust-native structures, bridging the gap for Halo Infinite modding projects.

This crate currently is in early-development. Please let me know via Github issues about any issues you encounter using this project.

[![Crates.io](https://img.shields.io/crates/v/infinite-rs.svg)](https://crates.io/crates/infinite-rs)

### Documentation
- Documentation on this project can be found at [docs.rs](https://docs.rs/infinite.rs).

### Usage/Example

```rust
use infinite_rs::ModuleFile

fn load_modules() -> std::io::Result<()> {
    // Create new instance of a Module file.
    // These are the main archive files used in Halo Infinite.
    let mut module = ModuleFile::new();
    // Reads to the module file given a file path.
    module.read("path/to/.module/file");

    // The tag file is made up of a header and other structures.
    // This is an "asset" stored in the module.
    // Reads file at index 0 of the module.
    println!("{:#?}", module.files[0]);
    /*
    ModuleFileEntry {
        resource_count: 0,
        parent_index: 7,
        unknown: 788,
        block_count: 2,
        block_index: 7,
        resource_index: 0,
        tag_group: "LigS",
        ...
    */

    // Read tag at index 0 into "tag info".
    module.read_tag(0)
    println!("{:#?}", module.files[0].tag_info);
    /*
    TagFile {
        header: TagHeader {
            magic: "ucsh",
            version: 27,
            root_struct_guid: -8339828558656697538,
            checksum: 7960564650044378456,
            ...
    */

    Ok(())
}
```
