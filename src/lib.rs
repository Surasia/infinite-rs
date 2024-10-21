#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![warn(clippy::missing_safety_doc)]
#![allow(clippy::module_name_repetitions)]
#![warn(clippy::all)]
/*!
Simple and fast deserialization library for Halo Infinite.

## Getting Started: Loading a Module file
Modules are the file format that store "tags" in Halo Infinite. These files are used to store all the assets in the game, including models, textures, metadata, and more. `infinite-rs` provides a simple interface to load these tags, starting with loading the module files themselves.

```rust
use infinite_rs::ModuleFile;
use anyhow::Result;

fn load_modules() -> Result<()> {
    // Create new instance of a Module file.
    // These are the main archive files used in Halo Infinite.
    let mut module = ModuleFile::new();
    // Reads to the module file given a file path.
    module.read(String::from("C:/XboxGames/Halo Infinite/Content/deploy/any/globals-rtx-new.module"))?;
    Ok(())
}
```

## Loading a tag file
After we have loaded a module file, we can now use the `read_tag` function to load a specific tag by index from the module file. This populates the `data_stream` and `tag_info` properties in a module entry that we can use later.

The `read_tag_from_id` function is also available to load a tag by its global ID, returning the index in which it was found in the module file.

```rust
use infinite_rs::ModuleFile;
use anyhow::Result;

fn load_tags() -> Result<()> {
    // Create new instance of a Module file.
    // These are the main archive files used in Halo Infinite.
    let mut module = ModuleFile::new();
    // Reads to the module file given a file path.
    module.read(String::from("C:/XboxGames/Halo Infinite/Content/deploy/any/globals-rtx-new.module"))?;
    // Load a specific tag from the module file.
    let tag_index = 0;
    module.read_tag(tag_index)?;
    // We can now access the data stream and tag info.
    let tag_data = module.files[tag_index as usize].data_stream.as_ref().unwrap();
    let tag_info = module.files[tag_index as usize].tag_info.as_ref().unwrap();
    Ok(())
}
```
*/

pub mod common;
pub mod module;
pub mod tag;

#[doc(inline)]
pub use crate::{module::loader::ModuleFile, tag::loader::TagFile};
