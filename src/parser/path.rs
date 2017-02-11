//! Parser for HVIF paths
use types::*;

/// Parse a path
named_attr!(#[doc = "Parses an HVIF path"], pub hvif_path<&[u8], HVIFPath>,
  do_parse!(
    take!(1) >>
    (HVIFPath { flags: 0, points: Vec::new() })
  )
);
