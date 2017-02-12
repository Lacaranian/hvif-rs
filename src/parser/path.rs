//! Parser for HVIF paths
use nom::*;
use types::*;

named_attr!(#[doc = "Parses an HVIF path"], pub hvif_path<&[u8], HVIFPath>,
  do_parse!(
    flags: be_u8 >>
    point_count: be_u8 >>
    path: apply!(hvif_path_parser_from_flags, flags, point_count) >>
    (HVIFPath { points: path })
  )
);

fn hvif_path_parser_from_flags(input: &[u8], flags: u8, point_count: u8) -> IResult<&[u8], Vec<HVIFPointCommand>>
{
  let using_commands = HVIF_PATH_FLAG_USES_COMMANDS.is_set_on(flags);
  match using_commands {
    true  => {
      let (rem_input, command_bytes) = try_parse!(input, apply!(hvif_path_command_headers, point_count));
      hvif_path_with_commands(rem_input, command_bytes)
    },
    false => {
      let no_curves = HVIF_PATH_FLAG_NO_CURVES.is_set_on(flags);
      match no_curves {
        true  => hvif_path_with_no_curves(input),
        false => hvif_path_with_curves(input),
      }
    },
  }
}

fn hvif_path_command_headers(input: &[u8], point_count: u8) -> IResult<&[u8], Vec<u8>>
{
  let command_byte_count : u8 = (point_count / 4) + (if point_count % 4 > 0 { 1 } else { 0 });
  let (rem_input, command_chunks) = try_parse!(input, count!(hvif_path_command_header_chunk, command_byte_count as usize));
  let ordered_commands = command_chunks.iter()
    .flat_map(|chunk| vec![chunk[3], chunk[2], chunk[1], chunk[0]])
    .take(point_count as usize)
    .collect();

  return IResult::Done(rem_input, ordered_commands)
}
named!(hvif_path_command_header_chunk<&[u8], Vec<u8>>,
  bits!(
    count!(take_bits!(u8, 2), 4)
  )
);

fn hvif_path_with_commands(input: &[u8], command_bytes: Vec<u8>) -> IResult<&[u8], Vec<HVIFPointCommand>>
{
  let size = command_bytes.len();
  return IResult::Done(input, (vec![HVIFPointCommand::HLine { x: size as f32 }]))
}

named!(hvif_path_with_no_curves<&[u8], Vec<HVIFPointCommand>>,
  do_parse!(
    (vec![HVIFPointCommand::HLine { x: 0.0 }])
  )
);

named!(hvif_path_with_curves<&[u8], Vec<HVIFPointCommand>>,
  do_parse!(
    (vec![HVIFPointCommand::HLine { x: 0.0 }])
  )
);


named!(hvif_path_point<&[u8], HVIFPoint>,
  do_parse!(
    x: hvif_path_coord >>
    y: hvif_path_coord >>
    (HVIFPoint { x: x, y: y})
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
named!(hvif_path_coord<&[u8], f32>,
  do_parse!(
    first_coord_byte: be_u8 >>
    coord: apply!(hvif_coord_parser, first_coord_byte) >>
    (coord)
  )
);
