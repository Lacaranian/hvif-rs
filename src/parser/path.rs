//! Parser for HVIF paths
use nom::*;
use types::*;

named_attr!(#[doc = "Parses an HVIF path"], pub hvif_path<&[u8], HVIFPath>,
  do_parse!(
    flags: be_u8 >>
    apply!(hvif_path_parser_from_flags, flags) >>
    (HVIFPath { flags: 0, points: Vec::new() })
  )
);

fn hvif_path_parser_from_flags(input: &[u8], flags: u8) -> IResult<&[u8], HVIFPointCommand>
{
  let using_commands = HVIF_PATH_FLAG_USES_COMMANDS.is_set_on(flags);
  let p = match using_commands {
    true => hvif_path_with_commands,
    false => {
      let no_curves = HVIF_PATH_FLAG_NO_CURVES.is_set_on(flags);
      match no_curves {
        true => hvif_path_with_no_curves,
        false => hvif_path_with_curves,
      }
    },
  };
  p(input)
}

named!(hvif_path_with_commands<&[u8], HVIFPointCommand>,
  do_parse!(
    (HVIFPointCommand::HLine { x_coord: 0.0 })
  )
);

named!(hvif_path_with_no_curves<&[u8], HVIFPointCommand>,
  do_parse!(
    (HVIFPointCommand::HLine { x_coord: 0.0 })
  )
);

named!(hvif_path_with_curves<&[u8], HVIFPointCommand>,
  do_parse!(
    (HVIFPointCommand::HLine { x_coord: 0.0 })
  )
);
