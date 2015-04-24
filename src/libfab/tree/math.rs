use std::num::Float;

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


