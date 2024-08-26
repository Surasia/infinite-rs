/*!
Simple and fast deserialization library for Halo Infinite.

Provides tools to load module files, read and decompress tags within them. Translates built-in types into rust-native structures, bridging the gap for Halo Infinite modding projects.

This crate currently is in early-development. Please let me know via Github issues about any issues you encounter using this project.


```rust
use libinfinite::ModuleFile

fn load_modules() {
    // Create new instance of a Module file.
    // These are the main archive files used in Halo Infinite.
    let mut module = ModuleFile::new();
    // Reads to the module file given a file path.
    module.read("path/to/.module/file");

    // The tag file is made up of a header and other structures.
    // This is an "asset" stored in the module.
    println!("{:#?}", module.files[0].tag_file);


    """
    TagFile {
        header: TagHeader {
            magic: "ucsh",
            version: 27,
            root_struct_guid: -8339828558656697538,
            checksum: 7960564650044378456,
            ...
    """
}
```
*/
pub mod common;
pub mod module;
pub mod tag;

#[doc(inline)]
pub use crate::{module::loader::ModuleFile, tag::loader::TagFile};
