
enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Min,
    Max,
    Pow,
}


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
    n.results.f = value;
    n.results.i = Interval { lower: value, upper: value };

    //for q in 0..MIN_VOLUME {
    //    n.results.r[q] = value;
    //}
}


pub struct Node {

    // Node operation
    opcode: Opcode,

    // Saved results from the most recent evaluation
    results: Results,

    // rank of the node in the tree
    rank: i32,

    // flags
    flags: i8,

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

fn unary_n<F>(arg: Node, func: F, opcode: Opcode) -> Node
    where F : Fn(f32) -> f32 {

    let constant = 0;

    let result = Results {
        f: 0.0,
        i: Interval { lower: 0.0, upper: 1.0 },
        r: [0.0; MIN_VOLUME],
    };

    let n = Node {
        opcode: opcode,
        results: result,
        rank: 1,
        flags: 0,
        lhs: Option::None,
        rhs: Option::None,
        clone_address: Option::None,
    };

    if (constant != 0) {
    }

    n
}
