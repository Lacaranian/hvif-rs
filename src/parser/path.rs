//! Parser for HVIF paths
use nom::*;
use types::*;

use parser::util::*;

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
        true  => count!(input, hvif_path_point_line, point_count as usize),
        false => count!(input, hvif_path_point_curve, point_count as usize),
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
  let parsers: Vec<fn(&[u8]) -> IResult<&[u8], HVIFPointCommand>> = command_bytes.iter().flat_map(|&command_byte| {
      let parser: Option<fn(&[u8]) -> IResult<&[u8], HVIFPointCommand>> = match command_byte {
        0 => Some(hvif_path_point_horizontal_line),
        1 => Some(hvif_path_point_vertical_line),
        2 => Some(hvif_path_point_line),
        3 => Some(hvif_path_point_curve),
        _ => None,
      };

      parser
  }).collect();

  let result = if parsers.len() == command_bytes.len() {
    // We don't have any incorrect commands! Parse away
    // Imperative style required because nom's macros don't play well inside the closure of a fold
    let mut piter = parsers.into_iter();
    let mut points = Vec::new();
    let mut rem_input = input;

    while let Some(parser) = piter.next() {
      let (next_input, point) = try_parse!(rem_input, parser);
      rem_input = next_input;
      points.push(point);
    }

    IResult::Done(rem_input, points)
  } else {
    // Incorrect command! Abort the parse, give an error
    IResult::Error(ErrorKind::Custom(0))
  };

  result
}

named!(hvif_path_point_horizontal_line<&[u8], HVIFPointCommand>,
  do_parse!(
    x: hvif_path_coord >>
    (HVIFPointCommand::HLine { x: x })
  )
);
named!(hvif_path_point_vertical_line<&[u8], HVIFPointCommand>,
  do_parse!(
    y: hvif_path_coord >>
    (HVIFPointCommand::VLine { y: y })
  )
);
named!(hvif_path_point_line<&[u8], HVIFPointCommand>,
  do_parse!(
    point: hvif_point >>
    (HVIFPointCommand::Line { point: point })
  )
);
named!(hvif_path_point_curve<&[u8], HVIFPointCommand>,
  do_parse!(
    point: hvif_point >>
    point_in: hvif_point >>
    point_out: hvif_point >>
    (HVIFPointCommand::Curve { point_in: point_in, point: point, point_out: point_out } )
  )
);





