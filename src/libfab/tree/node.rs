
/////////////////// Opcodes

enum Opcode {
    ADD,
    SUB,
    MUL,
    DIV,
    MIN,
    MAX,
    POW,

    ABS,
    SQUARE,
    SQRT,
    SIN,
    COS,
    TAN,
    ASIN,
    ACOS,
    ATAN,
    NEG,

    X,
    Y,
    Z,
    CONST,

    LAST_OP,
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

//fn binary_n() -> Node {
//}

fn unary_n<F>(arg: Node, func: F, op: Opcode) -> Node
    where F : Fn(f32) -> f32 {

    let result = Results {
        f: 0.0,
        i: Interval { lower: 0.0, upper: 1.0 },
        r: [0.0; MIN_VOLUME],
    };

    let mut n = Node {
        opcode: if arg.is_constant { Opcode::CONST } else { op },
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
