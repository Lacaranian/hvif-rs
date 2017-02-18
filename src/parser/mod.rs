//! Parsers for HVIF images
pub mod style;
pub mod path;
pub mod shape;
pub mod util;

use nom::*;
use types::*;

use self::style::hvif_style;
use self::path::hvif_path;
use self::shape::hvif_shape;

named_attr!(#[doc = "Parses an entire HVIF image"], pub hvif_image<&[u8], HVIFImage>,
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
