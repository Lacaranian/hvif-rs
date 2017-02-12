//! Parser for HVIF styles
use nom::*;
use types::*;

named_attr!(#[doc = "Parses an HVIF style"], pub hvif_style<&[u8], HVIFStyle>,
  do_parse!(
    style_type: be_u8 >>
    style: apply!(style_type_to_parser, style_type) >>
    (style)
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
  let grayscale = HVIF_GRADIENT_FLAG_GRAYS.is_set_on(flags);
  let p = match grayscale {
    true => hvif_style_gradient_color_rgb_grays,
    false => hvif_style_gradient_color_rgb_nongrays,
  };
  p(input)
}
named!(hvif_style_gradient_color_rgb_nongrays<&[u8], (u8,u8,u8)>, tuple!(be_u8, be_u8, be_u8));
named!(hvif_style_gradient_color_rgb_grays<&[u8], (u8,u8,u8)>, do_parse!(value: be_u8 >> (value, value, value)));

fn hvif_style_gradient_color_alpha_parser(input: &[u8], flags: u8) -> IResult<&[u8], u8>
{
  let no_alpha = HVIF_GRADIENT_FLAG_NO_ALPHA.is_set_on(flags);
  let (rem_input, alpha) = match no_alpha {
    true  => (input, 0b1111_1111),
    false => try_parse!(input, be_u8),
  };

  return IResult::Done(rem_input, alpha)
}
