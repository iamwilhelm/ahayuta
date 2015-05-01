use libfab::tree::math;

/////////////////// Switches

/// The minimum volume for interval evaluation
const MIN_VOLUME:usize = 64;

/// Remove duplicate nodes when combining MathTrees
#[allow(dead_code)]
const DEDUPLICATE:bool = true;

/// Deactivate inactive tree branches
#[allow(dead_code)]
const PRUNE:bool = true;

/// Print debug data
#[allow(dead_code)]
const DEBUG:bool = false;

/////////////////// Opcodes

#[derive(PartialEq, Debug)]
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
    Constant,

    LastOP,
}

impl Opcode {
    pub fn clone(&self) -> Opcode {
        match *self {
            Opcode::Add => Opcode::Add,
            Opcode::Sub => Opcode::Sub,
            Opcode::Mul => Opcode::Mul,
            Opcode::Div => Opcode::Div,
            Opcode::Min => Opcode::Min,
            Opcode::Max => Opcode::Max,
            Opcode::Pow => Opcode::Pow,

            Opcode::Abs => Opcode::Abs,
            Opcode::Square => Opcode::Square,
            Opcode::Sqrt => Opcode::Sqrt,
            Opcode::Sin => Opcode::Sin,
            Opcode::Cos => Opcode::Cos,
            Opcode::Tan => Opcode::Pow,
            Opcode::ArcSin => Opcode::ArcSin,
            Opcode::ArcCos => Opcode::ArcCos,
            Opcode::ArcTan => Opcode::ArcTan,
            Opcode::Neg => Opcode::Neg,

            Opcode::X => Opcode::X,
            Opcode::Y => Opcode::Y,
            Opcode::Z => Opcode::Z,
            Opcode::Constant => Opcode::Constant,

            Opcode::LastOP => Opcode::LastOP,
        }
    }
}


//////////////////// Results


#[allow(dead_code)]
struct Interval {
    lower: f32,
    upper: f32,
}

impl Interval {
    pub fn clone(&self) -> Interval {
        Interval {
            lower: self.lower.clone(),
            upper: self.upper.clone(),
        }
    }
}

#[allow(dead_code)]
struct Results {
    f: f32,
    i: Interval,
    r: [f32; MIN_VOLUME],
}

impl Results {
    pub fn clone(&self) -> Results {
        let mut results = Results {
            f: self.f.clone(),
            i: self.i.clone(),
            r: [0.0; MIN_VOLUME],
        };

        // because I don't know how to clone arrays
        for i in 0..MIN_VOLUME {
            results.r[i] = self.r[i];
        }

        results
    }
}

pub fn fill_results(n: &mut Node, value: f32) {
    n.results.f = value.clone();
    n.results.i = Interval { lower: value.clone(), upper: value.clone() };

    for q in 0..MIN_VOLUME {
        n.results.r[q] = value.clone();
    }
}

////////////////////// Node

#[allow(dead_code)]
pub struct Node {

    /// Node operation
    opcode: Opcode,

    /// Saved results from the most recent evaluation
    results: Results,

    /// rank of the node in the tree
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

impl Node {
    pub fn clone(&self) -> Node {
        let mut clone = Node {
            opcode: self.opcode.clone(),
            results: self.results.clone(),

            rank: self.rank.clone(),

            is_constant: self.is_constant.clone(),
            is_ignored: self.is_ignored.clone(),
            is_boolean: self.is_boolean.clone(),
            is_in_tree: self.is_in_tree.clone(),

            lhs: Option::None,
            rhs: Option::None,

            clone_address: Option::None,
        };

        match self.lhs {
            Some(ref left) => clone.lhs = Option::Some(Box::new((*left).clone())),
            _ => {},
        };

        match self.rhs {
            Some(ref right) => clone.rhs = Option::Some(Box::new((*right).clone())),
            _ => {},
        };

        clone
    }

    pub fn print(&self) {
        match self.opcode {
            Opcode::Add => {
                print!("(");
                match self.lhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(" + ");
                match self.rhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(")");
            }
            Opcode::Sub => {
                print!("(");
                match self.lhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(" - ");
                match self.rhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(")");
            }
            Opcode::Mul => {
                print!("(");
                match self.lhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(" * ");
                match self.rhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(")");
            }
            Opcode::Div => {
                print!("(");
                match self.lhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(" / ");
                match self.rhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(")");
            }
            Opcode::Min => {
                print!("min[");
                match self.lhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(", ");
                match self.rhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!("]");
            }
            Opcode::Max => {
                print!("max[");
                match self.lhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(", ");
                match self.rhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!("]");
            }
            Opcode::Pow => {
                print!("(");
                match self.lhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(" ^ ");
                match self.rhs {
                    Some(ref node) => {
                        node.print();
                    },
                    _ => { },
                }
                print!(")");
            }

            Opcode::Abs => {
                match self.lhs {
                    Some(ref node) => {
                        print!("abs(");
                        node.print();
                        print!(")");
                    },
                    _ => { },
                }
            },
            Opcode::Square => {
                match self.lhs {
                    Some(ref node) => {
                        print!("square(");
                        node.print();
                        print!(")");
                    },
                    _ => { },
                }
            },
            Opcode::Sqrt => {
                match self.lhs {
                    Some(ref node) => {
                        print!("sqrt(");
                        node.print();
                        print!(")");
                    },
                    _ => { },
                }
            },
            Opcode::Sin => {
                match self.lhs {
                    Some(ref node) => {
                        print!("sin(");
                        node.print();
                        print!(")");
                    },
                    _ => { },
                }
            },
            Opcode::Cos => {
                match self.lhs {
                    Some(ref node) => {
                        print!("cos(");
                        node.print();
                        print!(")");
                    },
                    _ => { },
                }
            },
            Opcode::Tan => {
                match self.lhs {
                    Some(ref node) => {
                        print!("tan(");
                        node.print();
                        print!(")");
                    },
                    _ => { },
                }
            },
            Opcode::ArcSin => {
                match self.lhs {
                    Some(ref node) => {
                        print!("arcsin(");
                        node.print();
                        print!(")");
                    },
                    _ => { },
                }
            },
            Opcode::ArcCos => {
                match self.lhs {
                    Some(ref node) => {
                        print!("arccos(");
                        node.print();
                        print!(")");
                    },
                    _ => { },
                }
            },
            Opcode::ArcTan => {
                match self.lhs {
                    Some(ref node) => {
                        print!("arctan(");
                        node.print();
                        print!(")");
                    },
                    _ => { },
                }
            },
            Opcode::Neg => {
                match self.lhs {
                    Some(ref node) => {
                        print!("-");
                        node.print();
                    },
                    _ => { },
                }
            },

            Opcode::X => { print!("X"); },
            Opcode::Y => { print!("Y"); },
            Opcode::Z => { print!("Z"); },
            Opcode::Constant => { print!("{}", self.results.f); },
            _ => {
                print!(" ");
            },
        }
    }
}

fn binary_n<F>(lhs: Node, rhs: Node, func: F, op: Opcode) -> Node 
    where F : Fn(f32, f32) -> f32 {

    let is_const = lhs.is_constant && rhs.is_constant;
    
    let result = Results {
        f: 0.0,
        i: Interval { lower: 0.0, upper: 1.0 },
        r: [0.0; MIN_VOLUME],
    };

    let mut n = Node {
        opcode: if is_const { Opcode::Constant } else { op },
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
        opcode: if arg.is_constant { Opcode::Constant } else { op },
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

/// Creates absolute value node
///
/// # Examples
///
/// ```
/// assert_eq!(3, 2);
/// ```
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

pub fn noary_n(opcode: Opcode) -> Node {
    let interval = Interval {
        lower: 0.0,
        upper: 0.0,
    };

    let results = Results {
        f: 0.0,
        i: interval,
        r: [0.0; MIN_VOLUME],
    };

    Node {
        opcode: opcode,
        rank: 0,
        results: results,

        is_constant: false,
        is_ignored: false,
        is_boolean: false,
        is_in_tree: false,

        lhs: Option::None,
        rhs: Option::None,
        clone_address: Option::None,
    }
}

pub fn constant_n(value: f32) -> Node {
    let mut n = noary_n(Opcode::Constant);
    n.is_constant = true;
    fill_results(&mut n, value);

    n
}

pub fn x_n() -> Node {
    noary_n(Opcode::X)
}

pub fn y_n() -> Node {
    noary_n(Opcode::Y)
}

pub fn z_n() -> Node {
    noary_n(Opcode::Z)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn binary_n_constant_test() {
        let a_node = constant_n(1.0);
        let b_node = constant_n(2.0);
        let add_node = add_n(a_node, b_node);
        assert_eq!(Opcode::Constant, add_node.opcode);
    }

    #[test]
    fn add_n_test() {
        let x = x_n();
        let y = y_n();
        let add_node = add_n(x, y);
        assert_eq!(Opcode::Add, add_node.opcode);
    }

    #[test]
    fn sub_n_test() {
        let x = x_n();
        let y = y_n();
        let sub_node = sub_n(x, y);
        assert_eq!(Opcode::Sub, sub_node.opcode);
    }

    #[test]
    fn mul_n_test() {
        let x = x_n();
        let y = y_n();
        let mul_node = mul_n(x, y);
        assert_eq!(Opcode::Mul, mul_node.opcode);
    }

    #[test]
    fn div_n_test() {
        let x = x_n();
        let y = y_n();
        let div_node = div_n(x, y);
        assert_eq!(Opcode::Div, div_node.opcode);
    }

    #[test]
    fn min_n_test() {
        let x = x_n();
        let y = y_n();
        let min_node = min_n(x, y);
        assert_eq!(Opcode::Min, min_node.opcode);
    }

    #[test]
    fn max_n_test() {
        let x = x_n();
        let y = y_n();
        let max_node = max_n(x, y);
        assert_eq!(Opcode::Max, max_node.opcode);
    }

    #[test]
    fn pow_n_test() {
        let x = x_n();
        let y = y_n();
        let pow_node = pow_n(x, y);
        assert_eq!(Opcode::Pow, pow_node.opcode);
    }

    #[test]
    fn unary_n_constant_test() {
        let const_node = constant_n(2.0);
        let abs_node = abs_n(const_node);
        assert_eq!(Opcode::Constant, abs_node.opcode);
    }

    #[test]
    fn abs_n_test() {
        let x_node = x_n();
        let abs_node = abs_n(x_node);
        assert_eq!(Opcode::Abs, abs_node.opcode);
    }

    #[test]
    fn square_n_test() {
        let x_node = x_n();
        let square_node = square_n(x_node);
        assert_eq!(Opcode::Square, square_node.opcode);
    }

    #[test]
    fn sqrt_n_test() {
        let x_node = x_n();
        let sqrt_node = sqrt_n(x_node);
        assert_eq!(Opcode::Sqrt, sqrt_node.opcode);
    }

    #[test]
    fn sin_n_test() {
        let x_node = x_n();
        let sin_node = sin_n(x_node);
        assert_eq!(Opcode::Sin, sin_node.opcode);
    }

    #[test]
    fn cos_n_test() {
        let x_node = x_n();
        let cos_node = cos_n(x_node);
        assert_eq!(Opcode::Cos, cos_node.opcode);
    }

    #[test]
    fn tan_n_test() {
        let x_node = x_n();
        let tan_node = tan_n(x_node);
        assert_eq!(Opcode::Tan, tan_node.opcode);
    }

    #[test]
    fn asin_n_test() {
        let x_node = x_n();
        let asin_node = asin_n(x_node);
        assert_eq!(Opcode::ArcSin, asin_node.opcode);
    }

    #[test]
    fn acos_n_test() {
        let x_node = x_n();
        let acos_node = acos_n(x_node);
        assert_eq!(Opcode::ArcCos, acos_node.opcode);
    }

    #[test]
    fn atan_n_test() {
        let x_node = x_n();
        let atan_node = atan_n(x_node);
        assert_eq!(Opcode::ArcTan, atan_node.opcode);
    }

    #[test]
    fn neg_n_test() {
        let x_node = x_n();
        let neg_node = neg_n(x_node);
        assert_eq!(Opcode::Neg, neg_node.opcode);
    }

    #[test]
    fn constant_n_test() {
        let node = constant_n(2.0);
        assert_eq!(Opcode::Constant, node.opcode);
    }

    #[test]
    fn x_n_test() {
        let node = x_n();
        assert_eq!(Opcode::X, node.opcode);
    }

    #[test]
    fn y_n_test() {
        let node = y_n();
        assert_eq!(Opcode::Y, node.opcode);
    }

    #[test]
    fn z_n_test() {
        let node = z_n();
        assert_eq!(Opcode::Z, node.opcode);
    }


}
