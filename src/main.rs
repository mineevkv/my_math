// main.rs - Demonstration of the math module with traditional syntax

mod math;

// Import everything from our math module
use math::*;

fn main() {
    println!("=== MATH MODULE DEMONSTRATION ===\n");
    
    // Example 1: Basic trigonometry
    println!("1. BASIC TRIGONOMETRY");
    let x = PI / 4.0; // π/4 radians (45 degrees)
    println!("   x = π/4 = {:.3} rad", x);
    println!("   sin(x) = {:.3}", sin(x));
    println!("   cos(x) = {:.3}", cos(x));
    println!("   tan(x) = {:.3}", tan(x));
    println!("   sin²(x) + cos²(x) = {:.6}", sqr(sin(x)) + sqr(cos(x))); // Should be ~1.0
    
    // Example 2: Angle conversions
    println!("\n2. ANGLE CONVERSIONS");
    let degrees = 180.0;
    let radians = to_radians(degrees);
    println!("   {}° = {:.3} rad", degrees, radians);
    println!("   {} rad = {:.3}°", radians, to_degrees(radians));
    
    // Example 3: Complex mathematical expressions
    println!("\n3. COMPLEX EXPRESSIONS");
    let a = 2.0;
    let b = 3.0;
    let expr_result = sin(pow(a, b) + cos(b)) * exp(-0.5 * a);
    println!("   sin(a^b + cos(b)) * exp(-0.5*a)");
    println!("   where a = {}, b = {}", a, b);
    println!("   Result = {:.6}", expr_result);
    
    // Example 4: Using macros
    println!("\n4. USING MACROS");
    let val = 3.0;
    println!("   sqr!({}) = {}", val, sqr!(val));
    println!("   expr!(sin PI/6) = {:.3}", expr!(sin PI/6.0));
    
    // Example 5: Point operations
    println!("\n5. 2D POINT OPERATIONS");
    let p1 = Point2D::new(1.0, 0.0);
    let p2 = Point2D::new(0.0, 1.0);
    println!("   Point 1: ({:.1}, {:.1})", p1.x, p1.y);
    println!("   Point 2: ({:.1}, {:.1})", p2.x, p2.y);
    println!("   Distance between points: {:.3}", p1.distance_to(&p2));
    println!("   Distance squared: {:.3}", p1.distance_squared(&p2));
    
    // Rotate point 1 by 90 degrees (π/2 radians)
    let p1_rotated = p1.rotate(PI / 2.0);
    println!("   Point 1 rotated 90°: ({:.1}, {:.1})", p1_rotated.x, p1_rotated.y);
    
    // Example 6: Vector operations
    println!("\n6. VECTOR OPERATIONS");
    let v1 = Vector2D::new(3.0, 4.0);
    let v2 = Vector2D::new(1.0, 2.0);
    println!("   Vector 1: ({:.1}, {:.1})", v1.x, v1.y);
    println!("   Vector 2: ({:.1}, {:.1})", v2.x, v2.y);
    println!("   Length of v1: {:.3}", v1.length());
    println!("   Dot product v1·v2: {:.3}", v1.dot(&v2));
    println!("   Angle of v1: {:.3} rad ({:.1}°)", v1.angle(), to_degrees(v1.angle()));
    
    // Example 7: Complex numbers for rotations
    println!("\n7. COMPLEX NUMBERS FOR ROTATIONS");
    let angle1 = PI / 4.0; // 45 degrees
    let angle2 = PI / 2.0; // 90 degrees
    let c1 = Complex::from_angle(angle1);
    let c2 = Complex::from_angle(angle2);
    let product = c1.mul(&c2);
    println!("   e^(iπ/4) * e^(iπ/2) = e^(i3π/4)");
    println!("   Product angle: {:.3}π rad", product.angle() / PI);
    
    // Example 8: Interpolation examples
    println!("\n8. INTERPOLATION");
    let start = 0.0;
    let end = 100.0;
    println!("   Linear interpolation from {} to {}:", start, end);
    for &t in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        println!("     t={:.2}: lerp = {:.1}", t, lerp(start, end, t));
    }
    
    // Example 9: Special functions
    println!("\n9. SPECIAL FUNCTIONS");
    let angle = 5.0 * PI; // 5π rad
    let normalized = normalize_angle(angle);
    println!("   Original angle: {:.3}π rad", angle / PI);
    println!("   Normalized to [0, 2π): {:.3}π rad", normalized / PI);
    
    // Example 10: Short method syntax using MathExt trait
    println!("\n10. SHORT METHOD SYNTAX (MathExt trait)");
    let num = 2.5;
    println!("   {:.1}.sq() = {:.2}", num, num.sq());
    println!("   {:.1}.cb() = {:.2}", num, num.cb());
    println!("   π/3.s() = {:.3} (sin)", (PI / 3.0).s());
    println!("   π/3.c() = {:.3} (cos)", (PI / 3.0).c());
    
    // Example 11: Constants
    println!("\n11. MATHEMATICAL CONSTANTS");
    println!("   π = {:.10}", PI);
    println!("   e = {:.10}", E);
    println!("   √2 = {:.10}", SQRT_2);
    println!("   √3 = {:.10}", SQRT_3);
    
    // Example 12: Exponential and logarithmic functions
    println!("\n12. EXPONENTIAL AND LOGARITHMIC FUNCTIONS");
    let value = 2.0;
    println!("   exp({}) = {:.6}", value, exp(value));
    println!("   ln({}) = {:.6}", E, ln(E));
    println!("   log10(1000) = {:.6}", log10(1000.0));
    
    // Example 13: Advanced vector operations
    println!("\n13. ADVANCED VECTOR OPERATIONS");
    let v = Vector2D::new(5.0, 0.0);
    if let Some(normalized) = v.normalized() {
        println!("   Original vector: ({:.1}, {:.1})", v.x, v.y);
        println!("   Normalized: ({:.3}, {:.3})", normalized.x, normalized.y);
        println!("   Length of normalized vector: {:.6}", normalized.length());
    }
    
    // Example 14: Using hypot functions
    println!("\n14. HYPOTENUSE FUNCTIONS");
    let side_a = 3.0;
    let side_b = 4.0;
    let side_c = 5.0;
    println!("   2D hypotenuse of ({}, {}): {:.1}", side_a, side_b, hypot(side_a, side_b));
    println!("   3D hypotenuse of ({}, {}, {}): {:.1}", 
             side_a, side_b, side_c, hypot3(side_a, side_b, side_c));
    
    // Example 15: Rotation around a point
    println!("\n15. ROTATION AROUND A POINT");
    let point = Point2D::new(2.0, 3.0);
    let center = Point2D::new(1.0, 1.0);
    let rotated_point = point.rotate_around(&center, PI / 2.0);
    println!("   Point: ({:.1}, {:.1})", point.x, point.y);
    println!("   Center: ({:.1}, {:.1})", center.x, center.y);
    println!("   After 90° rotation around center: ({:.1}, {:.1})", 
             rotated_point.x, rotated_point.y);
    
    println!("\n=== DEMONSTRATION COMPLETE ===");
}