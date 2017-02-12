use types::HVIFFlag;

#[derive(Debug)]
/// An HVIF path, drawn between points
pub struct HVIFPath {
  /// A list of commands that represent the sequence of points for this path
  pub points: Vec<HVIFPointCommand>
}

#[derive(Debug, Copy, Clone)]
/// One or more points, and a command that specifies how the point/s are to be interpreted
pub enum HVIFPointCommand {
  /// A horizontal line from the previous x-coordinate to this one
  HLine { #[doc="target x-coordinate"] x: f32 },
  /// A vertical line from the previous y-coordinate to this one
  VLine { #[doc="target y-coordinate"] y: f32 },
  /// A straight line from the previous point to this one
  Line  { #[doc="target point"] point: HVIFPoint },
  /// A cubic Bezier curve from the previous point to this one
  Curve { #[doc="inital control point"] point_in: HVIFPoint, #[doc="target point"] point: HVIFPoint, #[doc="final control point"] point_out: HVIFPoint},
}

#[derive(Debug, Copy, Clone)]
/// A simple 2D point in the XY plane, where each coordinate is a floating point value
pub struct HVIFPoint {
  /// The x-coordinate
  pub x: f32,
  /// The y-coordinate
  pub y: f32,
}
