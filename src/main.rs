pub mod libfab;
use libfab::tree::math;
use libfab::tree::node;

fn main() {
    let result = math::add_f(3.0, 4.0);
    println!("sum: {}", result); 

    let node_x = node::x_n();
    let node_y = node::y_n();
    let node_1 = node::constant_n(1.0);
    let node_2 = node::constant_n(2.3);
    let node_a = node::add_n(node_x, node_1);
    let node_s = node::sub_n(node_y, node_2);
    let node_m = node::mul_n(node_a, node_s);

    node_m.print();

    println!("");

}

