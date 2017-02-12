//! Types for HVIF styles

#[derive(Debug)]
/// A single HVIF style
pub enum HVIFStyle {
  /// A solid aRGB color
  SolidColor {
    #[doc="red channel"] red: u8,
    #[doc="green channel"] green: u8,
    #[doc="blue channel"] blue: u8,
    #[doc="alpha channel"] alpha: u8,
  },
  /// A gradient between multiple aRGB colors
  Gradient(HVIFGradient),
  /// A solid opaque RGB color
  SolidColorNoAlpha {
    #[doc="red channel"] red: u8,
    #[doc="green channel"] green: u8,
    #[doc="blue channel"] blue: u8,
  },
  /// A solid greyscale color with an alpha channel
  SolidGray {
    #[doc="value on red, green, and blue channels"] value: u8,
    #[doc="alpha channel"] alpha: u8,
  },
  /// A solid greyscale color without an alpha channel
  SolidGrayNoAlpha {
    #[doc="value on red, green, and blue channels"] value: u8
  }
}

#[derive(Debug)]
/// A gradient between mutiple aRGB colors
pub struct HVIFGradient {
  /// The type of the gradient
  pub gradient_type: HVIFGradientType,
  /// A collection of the colors making up the gradient
  pub colors: Vec<HVIFGradientColor>
}

#[derive(Debug, Copy, Clone)]
/// The type of a gradient - determines how the gradient renders spatially
pub enum HVIFGradientType {
  /// A linear gradient; follows a line from one point to another
  Linear   = 0,
  /// A circular gradient; changes radially from the center to the edge of a circular region
  Circular = 1,
  /// A diamond graident; changes linearly from the center to the edge of a diagonal region
  Diamond  = 2,
  /// A conic gradient; changes angularly across a circular region
  Conic    = 3,
  /// An XY gradient; changes linearly in both the X and Y directions
  XY       = 4,
  /// A square root XY gradient: changes quadratically in both the X and Y directions
  SqrtXY   = 5
}
/// Convert a u8 into a gradient type
pub fn gradient_type_from_u8(num: u8) -> Option<HVIFGradientType> {
  match num {
    0 => Some(HVIFGradientType::Linear),
    1 => Some(HVIFGradientType::Circular),
    2 => Some(HVIFGradientType::Diamond),
    3 => Some(HVIFGradientType::Conic),
    4 => Some(HVIFGradientType::XY),
    5 => Some(HVIFGradientType::SqrtXY),
    _ => None
  }
}





#[derive(Debug, Copy, Clone)]
/// Color of a gradient, along with a stop offset
pub struct HVIFGradientColor {
  /// The stop offset for this color - the "position" it holds relative to other colors in the gradient
  pub stop_offset: u8,
  /// The alpha channel
  pub alpha: u8,
  /// The red channel
  pub red: u8,
  /// The green channel
  pub green: u8,
  /// The blue channel
  pub blue: u8
}
