//! This is documentation for the hvif_rs module
//!
//! This module provides facilities for reading or writing Haiku Vector Icon Format images

#![crate_name = "hvif_rs"]
#![allow(dead_code)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]


use std::io::{Bytes, Read, Result};

pub use self::types::*;
#[macro_use] mod types;

/// Read an image from bytes
fn read_from_bytes<R: Read>(readable: R) -> Result<HVIFImage> {
  let bytes: Bytes<R> = readable.bytes();
  unimplemented!()
}



