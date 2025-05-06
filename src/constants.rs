// Constants
pub const INF: f64 = f64::INFINITY;
pub const NEG_INF: f64 = f64::NEG_INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
