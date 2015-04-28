use libfab::tree::math;

/////////////////// Opcodes

pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Pow,

    Abs,
    Square,
    Sqrt,
    Sin,
    Cos,
    Tan,
    ArcSin,
    ArcCos,
    ArcTan,
    Neg,

    X,
    Y,
    Z,
    Const,

    LastOP,
}


//////////////////// Results

const MIN_VOLUME:usize = 64;

struct Interval {
    lower: f32,
    upper: f32,
}

struct Results {
    f: f32,
    i: Interval,
    r: [f32; MIN_VOLUME],
}

pub fn fill_results(n: &mut Node, value: f32) {
    n.results.f = value.clone();
    n.results.i = Interval { lower: value.clone(), upper: value.clone() };

    for q in 0..MIN_VOLUME {
        n.results.r[q] = value.clone();
    }
}

////////////////////// Node

pub struct Node {

    // Node operation
    opcode: Opcode,

    // Saved results from the most recent evaluation
    results: Results,

    // rank of the node in the tree
    rank: i32,

    // flags
    is_constant: bool,
    is_ignored: bool,
    is_boolean: bool,
    is_in_tree: bool,


    // left hand side of tree
    lhs: Option<Box<Node>>,

    // right hand side of tree
    rhs: Option<Box<Node>>,

    clone_address: Option<Box<Node>>,
}



//pub fn clone_node(n: Node) {
//}

fn binary_n<F>(lhs: Node, rhs: Node, func: F, op: Opcode) -> Node 
    where F : Fn(f32, f32) -> f32 {

    let is_const = lhs.is_constant && rhs.is_constant;
    
    let result = Results {
        f: 0.0,
        i: Interval { lower: 0.0, upper: 1.0 },
        r: [0.0; MIN_VOLUME],
    };

    let mut n = Node {
        opcode: if is_const { Opcode::Const } else { op },
        results: result,
        rank: if is_const {
            0
        } else {
            if lhs.rank > rhs.rank {
                1 + lhs.rank
            } else {
                1 + rhs.rank
            }
        },

        is_constant: is_const.clone(),
        is_ignored: false,
        is_boolean: false,
        is_in_tree: false,

        lhs: Option::None,
        rhs: Option::None,
        clone_address: Option::None
    };

    if is_const {
        fill_results(&mut n, func(lhs.results.f, rhs.results.f));
    } else {
        n.lhs = Option::Some(Box::new(lhs));
        n.rhs = Option::Some(Box::new(rhs));
    }

    n
}

pub fn add_n(lhs: Node, rhs: Node) -> Node {
    binary_n(lhs, rhs, math::add_f, Opcode::Add)
}
pub fn sub_n(lhs: Node, rhs: Node) -> Node {
    binary_n(lhs, rhs, math::sub_f, Opcode::Sub)
}
pub fn mul_n(lhs: Node, rhs: Node) -> Node {
    binary_n(lhs, rhs, math::mul_f, Opcode::Mul)
}
pub fn div_n(lhs: Node, rhs: Node) -> Node {
    binary_n(lhs, rhs, math::div_f, Opcode::Div)
}
pub fn min_n(lhs: Node, rhs: Node) -> Node {
    binary_n(lhs, rhs, math::min_f, Opcode::Min)
}
pub fn max_n(lhs: Node, rhs: Node) -> Node {
    binary_n(lhs, rhs, math::max_f, Opcode::Max)
}
pub fn pow_n(lhs: Node, rhs: Node) -> Node {
    binary_n(lhs, rhs, math::pow_f, Opcode::Pow)
}

fn unary_n<F>(arg: Node, func: F, op: Opcode) -> Node
    where F : Fn(f32) -> f32 {

    let result = Results {
        f: 0.0,
        i: Interval { lower: 0.0, upper: 1.0 },
        r: [0.0; MIN_VOLUME],
    };

    let mut n = Node {
        opcode: if arg.is_constant { Opcode::Const } else { op },
        results: result,
        rank: if arg.is_constant { 0 } else { arg.rank + 1 },

        is_constant: arg.is_constant.clone(),
        is_ignored: false,
        is_boolean: false,
        is_in_tree: false,

        lhs: Option::None,
        rhs: Option::None,
        clone_address: Option::None,
    };

    if arg.is_constant {
        fill_results(&mut n, func(arg.results.f))
    } else {
        n.lhs = Option::Some(Box::new(arg))
    }

    n
}

pub fn abs_n(child: Node) -> Node { unary_n(child, math::abs_f, Opcode::Abs) }
pub fn square_n(child: Node) -> Node { unary_n(child, math::square_f, Opcode::Square) }
pub fn sqrt_n(child: Node) -> Node { unary_n(child, math::sqrt_f, Opcode::Sqrt) }
pub fn sin_n(child: Node) -> Node { unary_n(child, math::sin_f, Opcode::Sin) }
pub fn cos_n(child: Node) -> Node { unary_n(child, math::cos_f, Opcode::Cos) }
pub fn tan_n(child: Node) -> Node { unary_n(child, math::tan_f, Opcode::Tan) }
pub fn asin_n(child: Node) -> Node { unary_n(child, math::asin_f, Opcode::ArcSin) }
pub fn acos_n(child: Node) -> Node { unary_n(child, math::acos_f, Opcode::ArcCos) }
pub fn atan_n(child: Node) -> Node { unary_n(child, math::atan_f, Opcode::ArcTan) }
pub fn neg_n(child: Node) -> Node { unary_n(child, math::neg_f, Opcode::Neg) }

