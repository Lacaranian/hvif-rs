//! Parser for HVIF shapes
use types::*;

named_attr!(#[doc = "Parses an HVIF shape"], pub hvif_shape<&[u8], HVIFShape>,
  do_parse!(
    take!(1) >>
    (HVIFShape {
      style_index: 0,
      path_indices: Vec::new(),
      flags: 0,
      transform: None,
      translate: None,
      lod_scale: None,
      transformer_list: Vec::new()
    })
  )
);
