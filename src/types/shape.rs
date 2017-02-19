//! Types for HVIF shapes

use types::path::*;

#[derive(Debug)]
/// An HVIF shape, consisting of a single style, one or more paths, and optional additional transformation data
pub struct HVIFShape {
  /// The index of the style used in the shape
  pub style_index: u8,
  /// The indices of the paths that use this shape's style
  pub path_indices: Vec<u8>,
  /// A list of any optional modifications to this shape
  pub modifiers: Vec<HVIFShapeModifier>,

}

#[derive(Debug)]
/// A modifier to an HVIF shape
pub enum HVIFShapeModifier {
  /// A single affine transformation matrix
  HVIFTransformMatrix(HVIFMatrix),
  /// The shape uses hinting
  HVIFHinting,
  /// Level of Detail scales
  /// Represents the allowable scales at which this shape will appear correctly
  HVIFLODScale {
    #[doc = "Minimum LOD for this shape"] min: f32,
    #[doc = "Maximum LOD for this shape"] max: f32,
  },
  /// An optional linear translation of the shape (offset)
  HVIFTranslation(HVIFPoint),
  /// A list of optional transformers
  HVIFTransformerList(Vec<HVIFTransformer>)

}

#[derive(Debug, Copy, Clone)]
/// A temporary implementation of an f24 value, as three bytes
pub struct HVIFf24 {
  #[doc = "First (most significant) bits"] pub fst: u8,
  #[doc = "Middle bits"] pub snd: u8,
  #[doc = "Last (least significant) bits"] pub thr: u8
}
impl From<HVIFf24> for f32 {
  // Not quite exact in its current form - be warned!
  fn from(hvif_f24: HVIFf24) -> Self {
    let f24_as_u32: u32 = ((hvif_f24.fst as u32) << 16) | ((hvif_f24.snd as u32) << 8) | (hvif_f24.thr as u32);

    // Get the raw bits for each independent component
    let sign_bit      = (f24_as_u32 & 0b100000000000000000000000) >> 23;
    let exponent_bits = (f24_as_u32 & 0b011111100000000000000000) >> 17;
    let mantissa_bits =  f24_as_u32 & 0b000000011111111111111111;

    // The sign is just -1 to the power of this bit
    let sign : i32 = match sign_bit {
      0 =>  1,
      _ => -1,
    };

    // The exponent is actually only 6 bits, so unsigned values range from 0 to 63
    // Make it a signed exponent, and offset it to range from -31 to 32
    let exponent_pre = (exponent_bits as i32) - 31;

    // The mantissa is 1.(bits), so we can divite the exponentiation into whole and fractional parts
    let mantissa_base = f32::powf(2.0, exponent_pre as f32);
    let mantissa_frac = (mantissa_bits as f32) * f32::powf(2.0, (exponent_pre -17) as f32);

    (sign as f32) * (mantissa_base + mantissa_frac)
  }
}


#[derive(Debug, Copy, Clone)]
/// A 2D affine transformation matrix
/// As per HVIF spec, consists of six f24 values
pub struct HVIFMatrix {
  #[doc = "x1"] pub x1: HVIFf24, #[doc = "y1"] pub y1: HVIFf24, #[doc = "z1"] pub z1: HVIFf24,
  #[doc = "x2"] pub x2: HVIFf24, #[doc = "y2"] pub y2: HVIFf24, #[doc = "z2"] pub z2: HVIFf24,
}

#[derive(Debug, Copy, Clone)]
/// Some transformation on a basic HVIF shape
pub enum HVIFTransformer {
  /// An affine transformation using a single matrix
  Affine(HVIFMatrix),
  /// ?
  Contour { #[doc="?"] width: f32, #[doc="?"] line_join: u8, #[doc="?"] miter_limit: u8 },
  /// ?
  Perspective,
  /// ?
  Stroke { #[doc="?"] width: f32, #[doc="?"] line_join: u8, #[doc="?"] line_cap: u8, #[doc="?"] miter_limit: u8 }
}
