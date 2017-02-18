//! Parser for HVIF shapes
use types::*;
use nom::*;

use parser::util::*;

named_attr!(#[doc = "Parses an HVIF shape"], pub hvif_shape<&[u8], HVIFShape>,
  do_parse!(
    tag!(&[0x0a]) >> // There is only one shape type, SHAPE_TYPE_PATH_SOURCE - should always be this!
    style_index: be_u8 >>
    path_indices: length_count!(be_u8, be_u8) >>
    shape_flags: be_u8 >>
    shape_modifiers: apply!(hvif_shape_modifier_parser_from_flags, shape_flags) >>
    (HVIFShape {
      style_index: style_index,
      path_indices: path_indices,
      modifiers: shape_modifiers
    })
  )
);

fn hvif_shape_modifier_parser_from_flags(input: &[u8], flags: u8) -> IResult<&[u8], Vec<HVIFShapeModifier>>
{
  let mut cur_input = input;
  let mut cur_modifiers = Vec::new();

  let parsers_per_flags : Vec<(HVIFFlag, fn(&[u8]) -> IResult<&[u8], HVIFShapeModifier>)> = vec![
    // Order of hinting doesn't matter, it doesn't parse anything!
    (HVIF_SHAPE_FLAG_HINTING         , hvif_shape_modifier_hinting),
    // Order of these parsers matter!
    (HVIF_SHAPE_FLAG_TRANSFORM       , hvif_shape_modifier_transform),
    (HVIF_SHAPE_FLAG_TRANSLATION     , hvif_shape_modifier_translation),
    (HVIF_SHAPE_FLAG_LOD_SCALE       , hvif_shape_modifier_lod_scale),
    (HVIF_SHAPE_FLAG_HAS_TRANSFORMERS, hvif_shape_modifier_has_transformers),
  ];
  let mut ppfiter = parsers_per_flags.into_iter();

  // Run all of the modifier parsers for set flags in order
  while let Some((flag, parser)) = ppfiter.next() {
    match flag.is_set_on(flags) {
      true  => {
        let (rem_input, new_mod) = try_parse!(cur_input, parser);
        cur_input = rem_input;
        cur_modifiers.push(new_mod);
      },
      false => ()
    }
  }

  return IResult::Done(cur_input, cur_modifiers)
}

named!(hvif_shape_modifier_hinting<&[u8], HVIFShapeModifier>,
  do_parse!(
    (HVIFShapeModifier::HVIFHinting)
  )
);
named!(hvif_shape_modifier_transform<&[u8], HVIFShapeModifier>,
  do_parse!(
    matrix: hvif_shape_matrix >>
    (HVIFShapeModifier::HVIFTransformMatrix(
      matrix
    ))
  )
);
named!(hvif_shape_modifier_translation<&[u8], HVIFShapeModifier>,
  do_parse!(
    point: hvif_point >>
    (HVIFShapeModifier::HVIFTranslation(
      point
    ))
  )
);
named!(hvif_shape_modifier_lod_scale<&[u8], HVIFShapeModifier>,
  do_parse!(
    min_int: be_u8 >>
    max_int: be_u8 >>
    (HVIFShapeModifier::HVIFLODScale {
      min: (min_int as f32) / 63.75,
      max: (max_int as f32) / 63.75,
    })
  )
);
named!(hvif_shape_modifier_has_transformers<&[u8], HVIFShapeModifier>,
  do_parse!(
    transformers: length_count!(be_u8, hvif_shape_modifier_transformer) >>
    (HVIFShapeModifier::HVIFTransformerList(transformers))
  )
);

named!(hvif_shape_modifier_transformer<&[u8], HVIFTransformer>,
  do_parse!(
    transformer_type: be_u8 >>
    transformer: apply!(hvif_shape_modifier_transformer_parser_from_flags,transformer_type) >>
    (transformer)
  )
);

fn hvif_shape_modifier_transformer_parser_from_flags(input: &[u8], transformer_type: u8) -> IResult<&[u8], HVIFTransformer>
{
  let maybe_parser: Option<fn(&[u8]) -> IResult<&[u8], HVIFTransformer>> = match transformer_type {
    20 => { // Affine matrix
      Some(hvif_shape_modifier_transformer_matrix)
    },
    21 => { // Contour
      Some(hvif_shape_modifier_transformer_contour)
    },
    22 => { // Perspective (unused?)
      Some(hvif_shape_modifier_transformer_perspective)
    },
    23 => { // Stroke
      Some(hvif_shape_modifier_transformer_stroke)

    },
    _ => None
  };

  let result = match maybe_parser {
    Some(parser) => {
      let (rem_input, transformer) = try_parse!(input, parser);
      IResult::Done(rem_input, transformer)
    }
    None => IResult::Error(ErrorKind::Custom(1))
  };

  return result
}

named!(hvif_shape_modifier_transformer_matrix<&[u8], HVIFTransformer>,
  do_parse!(
    matrix: hvif_shape_matrix >>
    (HVIFTransformer::Affine(matrix))
  )
);
named!(hvif_shape_modifier_transformer_contour<&[u8], HVIFTransformer>,
  do_parse!(
    width_int: be_u8 >>
    lj: be_u8 >>
    ml : be_u8 >>
    (HVIFTransformer::Contour {
      width: (width_int as f32) - 128.0,
      line_join: lj,
      miter_limit: ml
    })
  )
);
named!(hvif_shape_modifier_transformer_perspective<&[u8], HVIFTransformer>,
  do_parse!(
    (HVIFTransformer::Perspective)
  )
);
named!(hvif_shape_modifier_transformer_stroke<&[u8], HVIFTransformer>,
  do_parse!(
    width_int: be_u8 >>
    line_opts: be_u8 >>
    ml : be_u8 >>
    (HVIFTransformer::Stroke {
      width: (width_int as f32) - 128.0,
      line_join: line_opts & 15,
      line_cap: line_opts << 4,
      miter_limit: ml
    })
  )
);
