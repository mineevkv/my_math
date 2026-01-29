// math.rs - Mathematical functions with traditional syntax

use std::f64::consts;

// ========== BASIC CONSTANTS ==========
/// Pi (π)
pub const PI: f64 = consts::PI;
/// 2π (tau)
pub const TAU: f64 = consts::TAU;
/// π/2
pub const FRAC_PI_2: f64 = consts::FRAC_PI_2;
/// π/3
pub const FRAC_PI_3: f64 = consts::FRAC_PI_3;
/// π/4
pub const FRAC_PI_4: f64 = consts::FRAC_PI_4;
/// π/6
pub const FRAC_PI_6: f64 = consts::FRAC_PI_6;
/// Euler's number (e)
pub const E: f64 = consts::E;
/// √2
pub const SQRT_2: f64 = consts::SQRT_2;
/// 1/√2
pub const FRAC_1_SQRT_2: f64 = consts::FRAC_1_SQRT_2;
/// √3
pub const SQRT_3: f64 = 1.7320508075688772;
/// ln(2)
pub const LN_2: f64 = consts::LN_2;
/// ln(10)
pub const LN_10: f64 = consts::LN_10;
/// log₁₀(e)
pub const LOG10_E: f64 = consts::LOG10_E;
/// log₂(e)
pub const LOG2_E: f64 = consts::LOG2_E;

// ========== TRIGONOMETRY ==========
/// Sine of angle in radians
pub fn sin(x: f64) -> f64 {
    x.sin()
}

/// Cosine of angle in radians
pub fn cos(x: f64) -> f64 {
    x.cos()
}

/// Tangent of angle in radians
pub fn tan(x: f64) -> f64 {
    x.tan()
}

/// Arcsine, returns angle in radians in range [-π/2, π/2]
pub fn asin(x: f64) -> f64 {
    x.asin()
}

/// Arccosine, returns angle in radians in range [0, π]
pub fn acos(x: f64) -> f64 {
    x.acos()
}

/// Arctangent, returns angle in radians in range [-π/2, π/2]
pub fn atan(x: f64) -> f64 {
    x.atan()
}

/// Arctangent of y/x with quadrant adjustment, returns angle in radians in range [-π, π]
pub fn atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

// ========== HYPERBOLIC FUNCTIONS ==========
/// Hyperbolic sine
pub fn sinh(x: f64) -> f64 {
    x.sinh()
}

/// Hyperbolic cosine
pub fn cosh(x: f64) -> f64 {
    x.cosh()
}

/// Hyperbolic tangent
pub fn tanh(x: f64) -> f64 {
    x.tanh()
}

/// Inverse hyperbolic sine
pub fn asinh(x: f64) -> f64 {
    x.asinh()
}

/// Inverse hyperbolic cosine
pub fn acosh(x: f64) -> f64 {
    x.acosh()
}

/// Inverse hyperbolic tangent
pub fn atanh(x: f64) -> f64 {
    x.atanh()
}

// ========== EXPONENTIAL AND LOGARITHMIC ==========
/// Exponential function: e^x
pub fn exp(x: f64) -> f64 {
    x.exp()
}

/// Exponential function: 2^x
pub fn exp2(x: f64) -> f64 {
    x.exp2()
}

/// Exponential minus one: e^x - 1 (more accurate for small x)
pub fn exp_m1(x: f64) -> f64 {
    x.exp_m1()
}

/// Natural logarithm
pub fn ln(x: f64) -> f64 {
    x.ln()
}

/// Base-10 logarithm
pub fn log10(x: f64) -> f64 {
    x.log10()
}

/// Base-2 logarithm
pub fn log2(x: f64) -> f64 {
    x.log2()
}

/// Logarithm with arbitrary base
pub fn log(base: f64, x: f64) -> f64 {
    x.log(base)
}

/// Natural logarithm of 1 + x (more accurate for small x)
pub fn ln_1p(x: f64) -> f64 {
    x.ln_1p()
}

// ========== POWERS AND ROOTS ==========
/// Square root
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

/// Cube root
pub fn cbrt(x: f64) -> f64 {
    x.cbrt()
}

/// Power function: x^y
pub fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}

/// Integer power: x^n
pub fn powi(x: f64, n: i32) -> f64 {
    x.powi(n)
}

/// Hypotenuse: sqrt(x² + y²)
pub fn hypot(x: f64, y: f64) -> f64 {
    x.hypot(y)
}

/// Hypotenuse for three dimensions
pub fn hypot3(x: f64, y: f64, z: f64) -> f64 {
    (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
}

// ========== ROUNDING AND ABSOLUTE VALUE ==========
/// Absolute value
pub fn abs(x: f64) -> f64 {
    x.abs()
}

/// Sign function: -1.0, 0.0, or 1.0
pub fn signum(x: f64) -> f64 {
    x.signum()
}

/// Ceiling (round up)
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}

/// Floor (round down)
pub fn floor(x: f64) -> f64 {
    x.floor()
}

/// Round to nearest integer
pub fn round(x: f64) -> f64 {
    x.round()
}

/// Truncate fractional part
pub fn trunc(x: f64) -> f64 {
    x.trunc()
}

/// Fractional part
pub fn fract(x: f64) -> f64 {
    x.fract()
}

/// Remainder of division
pub fn rem(x: f64, y: f64) -> f64 {
    x % y
}

/// Remainder with rounding toward zero
pub fn rem_euclid(x: f64, y: f64) -> f64 {
    x.rem_euclid(y)
}

// ========== MINIMUM/MAXIMUM ==========
/// Maximum of two numbers
pub fn max(x: f64, y: f64) -> f64 {
    x.max(y)
}

/// Minimum of two numbers
pub fn min(x: f64, y: f64) -> f64 {
    x.min(y)
}

/// Clamp value to range [min, max]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.clamp(min, max)
}

// ========== SPECIAL FUNCTIONS ==========
/// Simultaneous sine and cosine (returns (sin, cos))
pub fn sin_cos(x: f64) -> (f64, f64) {
    x.sin_cos()
}

/// Square of a number
pub fn sqr(x: f64) -> f64 {
    x * x
}

/// Normalize angle to range [0, 2π)
pub fn normalize_angle(angle: f64) -> f64 {
    let mut normalized = angle % TAU;
    if normalized < 0.0 {
        normalized += TAU;
    }
    normalized
}

/// Normalize angle to range [-π, π]
pub fn normalize_angle_signed(angle: f64) -> f64 {
    let normalized = angle % TAU;
    if normalized > PI {
        normalized - TAU
    } else if normalized <= -PI {
        normalized + TAU
    } else {
        normalized
    }
}

/// Linear interpolation between a and b
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Inverse linear interpolation
pub fn inverse_lerp(a: f64, b: f64, value: f64) -> f64 {
    if (b - a).abs() < 1e-10 {
        0.0
    } else {
        ((value - a) / (b - a)).clamp(0.0, 1.0)
    }
}

/// Spherical linear interpolation (for angles)
pub fn slerp(start: f64, end: f64, t: f64) -> f64 {
    let diff = normalize_angle_signed(end - start);
    normalize_angle(start + diff * t.clamp(0.0, 1.0))
}

// ========== CONVERSIONS ==========
/// Degrees to radians
pub fn to_radians(degrees: f64) -> f64 {
    degrees.to_radians()
}

/// Radians to degrees
pub fn to_degrees(radians: f64) -> f64 {
    radians.to_degrees()
}

// ========== ADDITIONAL STRUCTURES ==========

/// 2D point in Cartesian coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    /// Rotate point around origin
    pub fn rotate(&self, angle_rad: f64) -> Self {
        let (sin_a, cos_a) = sin_cos(angle_rad);
        Self {
            x: self.x * cos_a - self.y * sin_a,
            y: self.x * sin_a + self.y * cos_a,
        }
    }
    
    /// Rotate point around specified center
    pub fn rotate_around(&self, center: &Point2D, angle_rad: f64) -> Self {
        let dx = self.x - center.x;
        let dy = self.y - center.y;
        let (sin_a, cos_a) = sin_cos(angle_rad);
        
        Self {
            x: center.x + dx * cos_a - dy * sin_a,
            y: center.y + dx * sin_a + dy * cos_a,
        }
    }
    
    /// Distance to another point
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        hypot(self.x - other.x, self.y - other.y)
    }
    
    /// Squared distance (faster, no square root)
    pub fn distance_squared(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

/// 2D vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    /// Create a new vector
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    /// Vector length (magnitude)
    pub fn length(&self) -> f64 {
        hypot(self.x, self.y)
    }
    
    /// Squared length (faster)
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    
    /// Normalized vector (unit length)
    pub fn normalized(&self) -> Option<Self> {
        let len = self.length();
        if len > 0.0 {
            Some(Self {
                x: self.x / len,
                y: self.y / len,
            })
        } else {
            None
        }
    }
    
    /// Angle of vector relative to X-axis
    pub fn angle(&self) -> f64 {
        atan2(self.y, self.x)
    }
    
    /// Dot product
    pub fn dot(&self, other: &Vector2D) -> f64 {
        self.x * other.x + self.y * other.y
    }
    
    /// Cross product (scalar in 2D)
    pub fn cross(&self, other: &Vector2D) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

/// Complex number (useful for rotations)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    /// Create complex number from real and imaginary parts
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    
    /// Create complex number from angle (phase)
    pub fn from_angle(angle_rad: f64) -> Self {
        Self {
            real: cos(angle_rad),
            imag: sin(angle_rad),
        }
    }
    
    /// Magnitude (absolute value)
    pub fn abs(&self) -> f64 {
        hypot(self.real, self.imag)
    }
    
    /// Angle (phase) of complex number
    pub fn angle(&self) -> f64 {
        atan2(self.imag, self.real)
    }
    
    /// Complex multiplication
    pub fn mul(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
    
    /// Rotate by angle (multiply by e^(iθ))
    pub fn rotate(&self, angle_rad: f64) -> Complex {
        self.mul(&Complex::from_angle(angle_rad))
    }
}

/// Extension trait for adding mathematical methods to numbers
pub trait MathExt {
    /// Sine (alternative syntax)
    fn s(&self) -> f64;
    /// Cosine (alternative syntax)
    fn c(&self) -> f64;
    /// Tangent (alternative syntax)
    fn t(&self) -> f64;
    /// Square
    fn sq(&self) -> f64;
    /// Cube
    fn cb(&self) -> f64;
}

impl MathExt for f64 {
    fn s(&self) -> f64 {
        self.sin()
    }
    
    fn c(&self) -> f64 {
        self.cos()
    }
    
    fn t(&self) -> f64 {
        self.tan()
    }
    
    fn sq(&self) -> f64 {
        self * self
    }
    
    fn cb(&self) -> f64 {
        self * self * self
    }
}

// ========== CONVENIENCE MACROS ==========

/// Macro for creating complex number from angle
#[macro_export]
macro_rules! complex_from_angle {
    ($angle:expr) => {
        $crate::Complex::from_angle($angle)
    };
}

/// Macro for quick square calculation
#[macro_export]
macro_rules! sqr {
    ($x:expr) => {
        $x * $x
    };
}

/// Macro for writing mathematical expressions in traditional notation
#[macro_export]
macro_rules! expr {
    (sin $x:expr) => { $crate::sin($x) };
    (cos $x:expr) => { $crate::cos($x) };
    (tan $x:expr) => { $crate::tan($x) };
    (exp $x:expr) => { $crate::exp($x) };
    (ln $x:expr) => { $crate::ln($x) };
    (sqrt $x:expr) => { $crate::sqrt($x) };
    (pow $x:expr, $y:expr) => { $crate::pow($x, $y) };
}