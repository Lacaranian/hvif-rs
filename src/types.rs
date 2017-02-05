//! Types for HVIF images

use std::collections::VecDeque;

#[derive(Debug)]
/// A fully specified HVIF image
pub struct HVIFImage {
  styles: VecDeque<HVIFStyle>,
  paths: VecDeque<HVIFPath>,
  shapes: VecDeque<HVIFShape>
}

#[derive(Debug)]
/// A single HVIF style
pub enum HVIFStyle {
  /// A solid aRGB color
  SolidColor { #[doc="alpha channel"] alpha: u8, #[doc="red channel"] red: u8, #[doc="green channel"] green: u8, #[doc="blue channel"] blue: u8 },
  /// A gradient between multiple aRGB colors
  Gradient(HVIFGradient),
  /// A solid opaque RGB color
  SolidColorNoAlpha { #[doc="red channel"] red: u8, #[doc="green channel"] green: u8, #[doc="blue channel"] blue: u8 },
  /// A solid greyscale color with an alpha channel
  SolidGray { #[doc="alpha channel"] alpha: u8, #[doc="value on red, green, and blue channels"] value: u8},
  /// A solid greyscale color without an alpha channel
  SolidGrayNoAlpha { #[doc="value on red, green, and blue channels"] value: u8 }
}

#[derive(Debug)]
/// A gradient between mutiple aRGB colors
pub struct HVIFGradient {
  gradient_type: HVIFGradientType,
  flags: u8,
  colors: VecDeque<HVIFGradientColor>
}

#[derive(Debug, Copy, Clone)]
/// The type of a gradient - determines how the gradient renders spatially
pub enum HVIFGradientType {
  /// A linear gradient; follows a line from one point to another
  Linear,
  /// A circular gradient; changes radially from the center to the edge of a circular region
  Circular,
  /// A diamond graident; changes linearly from the center to the edge of a diagonal region
  Diamond,
  /// A conic gradient; changes angularly across a circular region
  Conic,
  /// An XY gradient; changes linearly in both the X and Y directions
  XY,
  /// A square root XY gradient: changes quadratically in both the X and Y directions

  SqrtXY
}

#[derive(Debug, Copy, Clone)]
/// Flags that modify the nature of a gradient
pub enum GradientFlags {
  /// ?
  Transform       = 0b0000_0001,
  /// ?
  NoAlpha         = 0b0000_0010,
  /// ?
  Colors16Bit     = 0b0000_0100,
  /// ?
  Grays           = 0b0000_1000,
}

#[derive(Debug, Copy, Clone)]
/// Color of a gradient, along with a stop offset (distance between colors)
pub struct HVIFGradientColor {
  stop_offset: u8,
  alpha: u8,
  red: u8,
  green: u8,
  blue: u8
}


#[derive(Debug)]
/// An HVIF path, drawn between points
pub struct HVIFPath {
  flags: u8,
  points: VecDeque<HVIFPointCommand>
}

#[derive(Debug, Copy, Clone)]
/// Flags that modify the nature of a path
pub enum HVIFPathFlags {
  /// The path's last point is connected to its first point
  Closed       = 0b0000_0001,
  /// The path has a command section, and can use HLine and VLine commands
  UsesCommands = 0b0000_0010,
  /// The path is made up entirely of straight lines
  NoCurves     = 0b0000_0100,
}

#[derive(Debug, Copy, Clone)]
/// One or more points, and a command that specifies how the point/s are to be interpreted
pub enum HVIFPointCommand {
  /// A horizontal line from the previous x-coordinate to this one
  HLine { #[doc="target x-coordinate"] x_coord: f32 },
  /// A vertical line from the previous y-coordinate to this one
  VLine { #[doc="target y-coordinate"] y_coord: f32 },
  /// A straight line from the previous point to this one
  Line  { #[doc="target point"] point: HVIFPoint },
  /// A cubic Bezier curve from the previous point to this one
  Curve { #[doc="inital control point"] point_in: HVIFPoint, #[doc="target point"] point: HVIFPoint, #[doc="final control point"] point_out: HVIFPoint},
}

#[derive(Debug, Copy, Clone)]
/// A simple 2D point in the XY plane, where each coordinate is a floating point value
pub struct HVIFPoint { x: f32, y: f32 }

#[derive(Debug)]
/// An HVIF shape, consisting of a single style, one or more paths, and optional additional transformation data
pub struct HVIFShape {
  style_index: u8,
  path_indices: VecDeque<u8>,
  flags: u8,
  transform: Option<HVIFMatrix>,
  translate: Option<HVIFPoint>,
  lod_scale: Option<HVIFLODScale>,
  transformer_list: VecDeque<HVIFTransformer>
}

#[derive(Debug, Copy, Clone)]
/// Flags that modify the nature of a shape
pub enum HVIFShapeFlags {
  /// ?
  Transform       = 0b0000_0001,
  /// ?
  Hinting         = 0b0000_0010,
  /// ?
  LODScale        = 0b0000_0100,
  /// ?
  HasTransformers = 0b0000_1000,
  /// ?
  Translation     = 0b0001_0000,
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
