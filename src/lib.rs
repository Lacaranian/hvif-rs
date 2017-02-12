//! This is documentation for the hvif_rs module
//!
//! This module provides facilities for reading or writing Haiku Vector Icon Format images

#![crate_name = "hvif_rs"]
#![allow(dead_code)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#![cfg_attr(feature = "core", allow(unstable_features))]
#![cfg_attr(feature = "core", feature(no_std))]
#![cfg_attr(feature = "core", feature(collections))]
#![cfg_attr(feature = "core", no_std)]
#[cfg(feature = "core")]
extern crate collections;

#[macro_use]
extern crate nom;

#[macro_use] pub mod types;
#[macro_use] pub mod parser;
