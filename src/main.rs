mod libfab;
use libfab::tree::math;

fn main() {
    let result = math::add_f(3.0, 4.0);
    println!("sum: {}", result); 
}

