//! Simple and fast deserializer for Halo Infinite.
//!
//! Provides tools to load module files, read and decompress tags within them. Translates built-in types into rust-native structures, bridging the gap for Halo Infinite modding projects.

pub mod common;
pub mod module;
pub mod tag;
