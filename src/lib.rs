#![crate_name = "hvif_rs"]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

//! This is documentation for the hvif_parser

/// Test function
pub fn main() {
    println!("Hello world!")
}


#[cfg(test)]
mod tests {
    use main;
    #[test]
    fn it_works() {
        main()
    }
}
