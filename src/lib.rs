#![crate_name = "hvif_rs"]
#![allow(dead_code)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]



//! This is documentation for the hvif_rs module
//!
//! This module provides facilities for reading or writing Haiku Vector Icon Format images

use std::collections::VecDeque;
use std::io::{Bytes, Read, Result};

/// Read an image from bytes
fn read_from_bytes<R: Read>(readable: R) -> Result<HVIFImage> {
  //let bytes: Bytes<R> = readable.bytes();
  unimplemented!()
}

/// Types
struct HVIFImage {
  styles: VecDeque<HVIFStyle>,
  paths: VecDeque<HVIFPath>,
  shapes: VecDeque<HVIFShape>
}

/// Style types
enum HVIFStyle {
  SolidColor { alpha: u8, red: u8, green: u8, blue: u8 },
  Gradient { gradient: HVIFGradient },
  SolidColorNoAlpha,
  SolidGray,
  SolidGrayNoAlpha
}

struct HVIFGradient {
  gradient_type: HVIFGradientType,
  flags: u8,
  colors: VecDeque<HVIFGradientColor>
}

enum HVIFGradientType {
  Linear,
  Circular,
  Diamond,
  Conic,
  XY,
  SqrtXY
}

enum GradientFlags {
  Transform       = 0b0000_0001,
  NoAlpha         = 0b0000_0010,
  Colors16Bit     = 0b0000_0100,
  Grays           = 0b0000_1000,
}

struct HVIFGradientColor {
  stop_offset: u8,
  alpha: u8,
  red: u8,
  green: u8,
  blue: u8
}

/// Path types
struct HVIFPath {
  flags: u8,
  points: VecDeque<HVIFPointCommand>
}

enum HVIFPathFlags {
  Closed       = 0b0000_0001,
  UsesCommands = 0b0000_0010,
  NoCurves     = 0b0000_0100,
}

enum HVIFPointCommand {
  HLine { y_coord: f32 },
  VLine { x_coord: f32 },
  Line  { point: HVIFPoint },
  Curve { point_in: HVIFPoint, point: HVIFPoint, point_out: HVIFPoint}, // Cubic curve
}

struct HVIFPoint { x: f32, y: f32 }

/// Shape types
struct HVIFShape {
  style_index: u8,
  path_indices: VecDeque<u8>,
  flags: u8,
  transform: Option<HVIFMatrix>,
  translate: Option<HVIFPoint>,
  lod_scale: Option<HVIFLODScale>,
  transformer_list: VecDeque<HVIFTransformer>
}

enum HVIFShapeFlags {
  Transform       = 0b0000_0001,
  Hinting         = 0b0000_0010,
  LODScale        = 0b0000_0100,
  HasTransformers = 0b0000_1000,
  Translation     = 0b0001_0000,
}

struct HVIFMatrix {
  x1: HVIFf24, y1: HVIFf24, z1: HVIFf24,
  x2: HVIFf24, y2: HVIFf24, z2: HVIFf24,
}
struct HVIFf24 { fst: u8, snd: u8, thr: u8 }

struct HVIFLODScale { min: f32, max: f32 }

enum HVIFTransformer {
  Affine { matrix: HVIFMatrix },
  Contour { width: f32, line_join: u8, miter_limit: u8 },
  Perspective,
  Stroke { width: f32, line_join: u8, line_cap: u8, miter_limit: u8 }
}

/// Test function
fn empty_image() -> HVIFImage {
  HVIFImage {
    styles: VecDeque::new(),
    paths:  VecDeque::new(),
    shapes: VecDeque::new()
  }
}

#[cfg(test)]
mod tests {
    use empty_image;
    #[test]
    fn it_works() {
        empty_image();
    }
}
