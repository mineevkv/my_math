# Math Module - Rust Mathematics Library

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A comprehensive Rust mathematics module providing traditional mathematical syntax and functions for common 2D/3D operations, trigonometry, linear algebra, and more.

## 🚀 Features

### 📐 **Basic Mathematics**
- **Trigonometry**: `sin()`, `cos()`, `tan()`, `asin()`, `acos()`, `atan()`, `atan2()`
- **Hyperbolic Functions**: `sinh()`, `cosh()`, `tanh()`, `asinh()`, `acosh()`, `atanh()`
- **Exponential & Logarithmic**: `exp()`, `ln()`, `log10()`, `log2()`, `log()`
- **Powers & Roots**: `sqrt()`, `cbrt()`, `pow()`, `powi()`, `hypot()`, `hypot3()`
- **Rounding & Absolute**: `abs()`, `ceil()`, `floor()`, `round()`, `trunc()`, `fract()`

### 🔢 **Mathematical Constants**
- `PI`, `TAU`, `E`, `SQRT_2`, `SQRT_3`, `FRAC_PI_2`, `FRAC_PI_3`, `FRAC_PI_4`, `LN_2`, `LN_10`, etc.

### 📍 **2D Geometry**
- **Point2D**: 2D point with rotation, distance calculations
- **Vector2D**: 2D vector with length, normalization, dot/cross products
- **Complex Numbers**: Complex number operations useful for rotations

### 🔧 **Special Functions**
- `normalize_angle()`: Normalize angles to [0, 2π) or [-π, π]
- `lerp()`: Linear interpolation
- `slerp()`: Spherical linear interpolation (for angles)
- `sin_cos()`: Simultaneous sine and cosine calculation
- `sqr()`: Square function

### 🎯 **Convenience Features**
- **MathExt Trait**: Method syntax for common operations (e.g., `x.sq()`, `x.s()`, `x.c()`)
- **Macros**: `sqr!()`, `expr!()`, `complex_from_angle!()` for concise syntax
- **Angle Conversions**: `to_radians()`, `to_degrees()`

## 📦 Installation

Simply copy the `math.rs` file to your project and include it as a module:

```rust
// In your main.rs or lib.rs
mod math;
use math::*;