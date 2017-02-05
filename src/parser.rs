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
    style_type: be_u8 >>
    apply!(style_type_to_parser, style_type) >>
    (HVIFStyle::SolidGrayNoAlpha { value: 0 })
  )
);

fn style_type_to_parser(input: &[u8], t: u8) -> IResult<&[u8], HVIFStyle> {
  let p = match t {
    1 => hvif_style_solid,
    2 => hvif_style_gradient,
    3 => hvif_style_solid_no_alpha,
    4 => hvif_style_gray,
    5 => hvif_style_gray_no_alpha,
    _ => unimplemented!() //failure
  };
  p(input)
}

named!(hvif_style_solid<&[u8], HVIFStyle>,
  do_parse!(
    a: be_u8 >>
    r: be_u8 >>
    g: be_u8 >>
    b: be_u8 >>
    (HVIFStyle::SolidColor { alpha: a, red: r, green: g, blue: b })
  )
);
named!(hvif_style_gradient<&[u8], HVIFStyle>,
  do_parse!(
    gradient_type: map_opt!(be_u8, gradient_type_from_u8) >>
    flags : be_u8 >>
    colors: length_count!(be_u8, match flags {
      0 /* aRGB */      => hvif_style_gradient_color,
      2 /* Transform */ => unimplemented!(),
      4 /* No alpha */  => unimplemented!(),
      6                 => unimplemented!(),
      16 /* Grays */    => unimplemented!(),
      18                => unimplemented!(),
      20                => unimplemented!(),
      22                => unimplemented!(),
      _                 => unimplemented!()
    }) >>
    (HVIFStyle::Gradient(HVIFGradient {
      gradient_type: gradient_type,
      flags: flags,
      colors: colors
    }))
  )
);
named!(hvif_style_solid_no_alpha<&[u8], HVIFStyle>,
  do_parse!(
    r: be_u8 >>
    g: be_u8 >>
    b: be_u8 >>
    (HVIFStyle::SolidColorNoAlpha { red: r, green: g, blue: b })
  )
);
named!(hvif_style_gray<&[u8], HVIFStyle>,
  do_parse!(
    a: be_u8 >>
    v: be_u8 >>
    (HVIFStyle::SolidGray { alpha: a, value: v })
  )
);
named!(hvif_style_gray_no_alpha<&[u8], HVIFStyle>,
  do_parse!(
    v: be_u8 >>
    (HVIFStyle::SolidGrayNoAlpha { value: v })
  )
);

named!(hvif_style_gradient_color<&[u8], HVIFGradientColor>,
  do_parse!(
    so: be_u8 >>
    a: be_u8 >>
    r: be_u8 >>
    g: be_u8 >>
    b: be_u8 >>
    (HVIFGradientColor {
      stop_offset: so,
      alpha: a,
      red: r,
      green: g,
      blue: b
    })
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
