use std::f32;

// binary operators

pub fn add_f(a: f32, b: f32) -> f32 {
    a + b
}

pub fn sub_f(a: f32, b: f32) -> f32 {
    a - b
}

pub fn mul_f(a: f32, b: f32) -> f32 {
    a * b
}

pub fn div_f(a: f32, b: f32) -> f32 {
    a / b
}

pub fn min_f(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

pub fn max_f(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

pub fn pow_f(a: f32, b: f32) -> f32 {
    a.powf(b)    
}

// unary operators

pub fn abs_f(a: f32) -> f32 {
    a.abs()
}

pub fn square_f(a: f32) -> f32 {
    a * a
}

pub fn sqrt_f(a: f32) -> f32 {
    a.sqrt()
}

pub fn sin_f(a: f32) -> f32 {
    a.sin()
}

pub fn cos_f(a: f32) -> f32 {
    a.cos()
}

pub fn tan_f(a: f32) -> f32 {
    a.tan()
}

pub fn asin_f(a: f32) -> f32 {
    if a < -1.0 {
        -f32::consts::PI / 2.0
    } else if a > 1.0 {
        f32::consts::PI / 2.0
    } else {
        a.asin()
    }
}

pub fn acos_f(a: f32) -> f32 {
    if a < -1.0 {
        f32::consts::PI
    } else if a > 1.0 {
        0.0
    } else {
        a.acos()
    }
}

pub fn atan_f(a: f32) -> f32 {
    a.atan()
}

pub fn neg_f(a: f32) -> f32 {
    -a
}

//////////////// access

pub fn X_f(x: f32) -> f32 {
    x
}

pub fn Y_f(y: f32) -> f32 {
    y
}

pub fn Z_f(z: f32) -> f32 {
    z
}

