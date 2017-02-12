use types::path::*;

#[derive(Debug)]
/// An HVIF shape, consisting of a single style, one or more paths, and optional additional transformation data
pub struct HVIFShape {
  /// The index of the style used in the shape
  pub style_index: u8,
  /// The indices of the paths that use this shape's style
  pub path_indices: Vec<u8>,
  /// Flags that modify the nature of this shape
  pub flags: u8,
  /// An optional transformation matrix
  pub transform: Option<HVIFMatrix>,
  /// An optional linear translation of the shape (offset)
  pub translate: Option<HVIFPoint>,
  /// An optional LOD range (limit on the scales the shape should be used in)
  pub lod_scale: Option<HVIFLODScale>,
  /// A list of optional transformers that affect this list
  pub transformer_list: Vec<HVIFTransformer>
}

#[derive(Debug, Copy, Clone)]
/// A 2D affine transformation matrix
/// As per HVIF spec, consists of six f24 values
pub struct HVIFMatrix {
  x1: HVIFf24, y1: HVIFf24, z1: HVIFf24,
  x2: HVIFf24, y2: HVIFf24, z2: HVIFf24,
}
#[derive(Debug, Copy, Clone)]
/// A temporary implementation of an f24 value, as three bytes
pub struct HVIFf24 { fst: u8, snd: u8, thr: u8 }

#[derive(Debug, Copy, Clone)]
/// Level of Detail scales
/// Represents the allowable scales at which this shape will appear correctly
pub struct HVIFLODScale { min: f32, max: f32 }

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
