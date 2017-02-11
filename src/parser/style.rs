//! Parser for HVIF styles
use nom::*;
use types::*;

/// Parse the style type, then run a specific parser accordingly
named_attr!(#[doc = "Parses an HVIF style"], pub hvif_style<&[u8], HVIFStyle>,
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
    colors: length_count!(be_u8, apply!(hvif_style_gradient_color_parser, flags)) >>
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

fn hvif_style_gradient_color_parser(input: &[u8], flags: u8) -> IResult<&[u8], HVIFGradientColor>
{
  let (i1, so) = try_parse!(input, be_u8);
  let (i2, (r, g, b)) = try_parse!(i1, apply!(hvif_style_gradient_color_rgb_parser, flags));
  let (i3, a) = try_parse!(i2, apply!(hvif_style_gradient_color_alpha_parser, flags));

  return IResult::Done(i3, HVIFGradientColor { stop_offset: so, alpha: a, red: r, green: g, blue: b })
}

fn hvif_style_gradient_color_rgb_parser(input: &[u8], flags: u8) -> IResult<&[u8], (u8, u8, u8)>
{
  let p = match flags & 0b0000_1000 {
    0 => hvif_style_gradient_color_rgb_nongrays,
    _ => hvif_style_gradient_color_rgb_grays
  };
  p(input)
}
named!(hvif_style_gradient_color_rgb_nongrays<&[u8], (u8,u8,u8)>, tuple!(be_u8, be_u8, be_u8));
named!(hvif_style_gradient_color_rgb_grays<&[u8], (u8,u8,u8)>, do_parse!(value: be_u8 >> (value, value, value)));

fn hvif_style_gradient_color_alpha_parser(input: &[u8], flags: u8) -> IResult<&[u8], u8>
{
  let (rem_input, alpha) = match flags & 0b0000_0010 {
    0 => try_parse!(input, be_u8),
    _ => (input, 0b1111_1111)
  };

  return IResult::Done(rem_input, alpha)
}
