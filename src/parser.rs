use nom::*;
use types::*;

/// Parse an entire hvif image
named!(hvif_image<&[u8], HVIFImage>,
  do_parse!(
    tag!("ncif") >>
    styles: hvif_styles >>
    paths: hvif_paths >>
    shapes: hvif_shapes >>
    (HVIFImage { styles: styles, paths: paths, shapes: shapes })
  )
);

/// Parse the magic number at the beginning of any hvif file
named!(hvif_magic_number, tag!("ncif"));

/// Parse a single byte to obtain a count, then run the style parser that many times
named!(hvif_styles<&[u8], Vec<HVIFStyle>>,
  length_count!(be_u8, hvif_style)
);

/// Parse a single byte to obtain a count, then run the path parser that many times
named!(hvif_paths<&[u8], Vec<HVIFPath>>,
  length_count!(be_u8, hvif_path)
);

/// Parse a single byte to obtain a count, then run the shape parser that many times
named!(hvif_shapes<&[u8], Vec<HVIFShape>>,
  length_count!(be_u8, hvif_shape)
);

/// Parse the style type, then run a specific parser accordingly
named!(hvif_style<&[u8], HVIFStyle>,
  do_parse!(
    style_type: take!(1) >>
    (HVIFStyle::SolidGrayNoAlpha { value: 0 })
  )
);

/// Parse a path
named!(hvif_path<&[u8], HVIFPath>,
  do_parse!(
    take!(1) >>
    (HVIFPath { flags: 0, points: Vec::new() })
  )
);

/// Parse a shape
named!(hvif_shape<&[u8], HVIFShape>,
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
