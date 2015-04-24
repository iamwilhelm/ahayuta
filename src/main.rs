mod english;
use english::greeting;

mod libfab;
use libfab::tree::math;

fn main() {
    let result = math::add_f(3.0, 4.0);
    println!("sum: {}", result); 

    println!("hello world");
    println!("Hello in english: {}", greeting::hello());
}

