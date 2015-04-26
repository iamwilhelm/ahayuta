use std::num::Float;

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

//pub fn abs_f(a: f32) -> f32 {
//}

pub fn square_f(a: f32) -> f32 {
    a * a
}

//pub fn sqrt_f(a: f32) -> f32 {
//    return 
//}

pub fn neg_f(a: f32) -> f32 {
    -a
}

