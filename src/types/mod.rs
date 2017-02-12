//! Types for HVIF images
pub mod path;
pub mod shape;
pub mod style;

pub use self::style::*;
pub use self::path::*;
pub use self::shape::*;

#[cfg(feature = "core")]
use collections::vec::Vec;
#[cfg(not(feature = "core"))]
use std::vec::Vec;

#[derive(Debug)]
/// A fully specified HVIF image
pub struct HVIFImage {
  /// All styles belonging to this image
  pub styles: Vec<HVIFStyle>,
  /// All paths belonging to this image
  pub paths: Vec<HVIFPath>,
  /// All shapes belonging to this image
  pub shapes: Vec<HVIFShape>
}

#[derive(Debug, Copy, Clone)]
/// Flags that modify the parsing of the following data
pub struct HVIFFlag(u8);
impl From<HVIFFlag> for u8 { fn from(hpf: HVIFFlag) -> Self { hpf.0 }}
impl HVIFFlag {
  /// Checks whether the flag is set on a byte
  pub fn is_set_on(&self, flags: u8) -> bool
  {
    let masked: u8 = flags & &self.clone().into();
    masked != 0
  }
}

/// ?
pub const HVIF_GRADIENT_FLAG_TRANSFORM     : HVIFFlag = HVIFFlag(0b0000_0001);
/// ?
pub const HVIF_GRADIENT_FLAG_NO_ALPHA      : HVIFFlag = HVIFFlag(0b0000_0010);
/// ?
pub const HVIF_GRADIENT_FLAG_COLORS_16_BIT : HVIFFlag = HVIFFlag(0b0000_0100);
/// ?
pub const HVIF_GRADIENT_FLAG_GRAYS         : HVIFFlag = HVIFFlag(0b0000_1000);

/// The path's last point is connected to its first point
pub const HVIF_PATH_FLAG_CLOSED        : HVIFFlag = HVIFFlag(0b0000_0001);
/// The path has a command section, and can use HLine and VLine commands
pub const HVIF_PATH_FLAG_USES_COMMANDS : HVIFFlag = HVIFFlag(0b0000_0010);
/// The path is made up entirely of straight lines
pub const HVIF_PATH_FLAG_NO_CURVES     : HVIFFlag = HVIFFlag(0b0000_0100);

/// ?
pub const HVIF_SHAPE_FLAG_TRANSFORM        : HVIFFlag = HVIFFlag(0b0000_0001);
/// ?
pub const HVIF_SHAPE_FLAG_HINTING          : HVIFFlag = HVIFFlag(0b0000_0010);
/// ?
pub const HVIF_SHAPE_FLAG_LOD_SCALE        : HVIFFlag = HVIFFlag(0b0000_0100);
/// ?
pub const HVIF_SHAPE_FLAG_HAS_TRANSFORMERS : HVIFFlag = HVIFFlag(0b0000_1000);
/// ?
pub const HVIF_SHAPE_FLAG_TRANSLATION      : HVIFFlag = HVIFFlag(0b0001_0000);
