//! Utility parsers used throughout the HVIF format

use types::*;
use nom::*;

named_attr!(#[doc = "Parses an HVIF point"], pub hvif_point<&[u8], HVIFPoint>,
  do_parse!(
    x: hvif_path_coord >>
    y: hvif_path_coord >>
    (HVIFPoint { x: x, y: y})
  )
);

named_attr!(#[doc = "Parses an HVIF coordinate"], pub hvif_path_coord<&[u8], f32>,
  do_parse!(
    first_coord_byte: be_u8 >>
    coord: apply!(hvif_coord_parser, first_coord_byte) >>
    (coord)
  )
);

fn hvif_coord_parser(input: &[u8], first: u8) -> IResult<&[u8], f32>
{
  let is_big = first & 0b1000_0000 != 0;
  let (rem_input, value) = match is_big {
    true  => {
      let (i1, second) = try_parse!(input, be_u8);
      // Hooray! No mem::transmute needed
      let u16value = ((second as u16) << 8) + (first as u16);
      let value = ((u16value as f32) / 102.0) - 128.0;
      (i1, value)
    },
    false => {
      let value = (first as f32) - 32.0;
      (input, value)
    },
  };

  return IResult::Done(rem_input, value)
}

named_attr!(#[doc = "Parses an HVIF affine matrix"], pub hvif_shape_matrix<&[u8], HVIFMatrix>,
  do_parse!(
    x1: hvif_shape_f24 >>
    y1: hvif_shape_f24 >>
    z1: hvif_shape_f24 >>
    x2: hvif_shape_f24 >>
    y2: hvif_shape_f24 >>
    z2: hvif_shape_f24 >>
    (HVIFMatrix {
      x1: x1, y1: y1, z1: z1,
      x2: x2, y2: y2, z2: z2,
    })
  )
);

named_attr!(#[doc = "Parses an HVIF 24-bit float"], pub hvif_shape_f24<&[u8], HVIFf24>,
  do_parse!(
    fst: be_u8 >>
    snd: be_u8 >>
    thr: be_u8 >>
    (HVIFf24 { fst: fst, snd: snd, thr: thr })
  )
);
