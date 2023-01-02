use std::{rc::Rc, collections::HashMap, hash::Hash, slice::{from_ref, from_mut}};

use egg::{Language, Id};
use smallvec::SmallVec;


#[derive(Debug)]
struct NodeDB {
    /// Store python objects in something like this?
    names: HashMap<u32, String>,
}


#[derive(Debug, Clone)]
pub struct SimpleNode {
    db: Rc<NodeDB>,
    node_id: u32,
    /// The inputs of the node
    children: SmallVec<[Id; 4]>,
}

impl PartialEq for SimpleNode {
    fn eq(&self, other: &Self) -> bool {
        assert!(Rc::ptr_eq(&self.db, &other.db));
        self.node_id == other.node_id && self.children == other.children
    }
}

impl Eq for SimpleNode { }

impl PartialOrd for SimpleNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        assert!(Rc::ptr_eq(&self.db, &other.db));
        match self.node_id.partial_cmp(&other.node_id) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.children.partial_cmp(&other.children)
    }
}


impl Ord for SimpleNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert!(Rc::ptr_eq(&self.db, &other.db));
        match self.node_id.cmp(&other.node_id) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.children.cmp(&other.children)
    }
}

impl Hash for SimpleNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node_id.hash(state);
        self.children.hash(state);
    }
}


enum ScalarType {
    Float64,
    Float32,
    UInt32,
    Int32,
}


struct Dim {
    size_hint: usize,
}

type Dims = SmallVec<[Dim; 2]>;


enum Type {
    DenseTensor(ScalarType, Dims),
    Scalar(ScalarType),
}


struct BufferedVariable {
    /// First is the buffer, second the source of the state (ie the owner node).
    /// TODO state source could be None
    children: [Id; 2],
    value_type: Type,
}


struct UnbufferedVariable {
    owner: Option<Id>,
    value_type: Type,
}


enum Variable {
    Buffered(BufferedVariable),
    Unbuffered(UnbufferedVariable),
}


struct Buffer {}


struct Elemwise {
    /// first n_inputs should be `Node::Argument`, remaining
    /// n_outputs should be `Node::Output`.
    children: SmallVec<[Id; 8]>,
    n_inputs: u64,
    // TODO inplace_map
}


struct IfElse {
    /// The first child is the condition, then we have `n_inputs`
    /// input children, and another `n_input` output children.
    children: SmallVec<[Id; 8]>,
    n_inputs: usize,
}


enum Node {
    /// A node in the pytensor sense, that doesn't have an inner fgraph
    Simple(SimpleNode),
    /// A elemwise node, that needs to also store the scalar fgraph.
    /// The children should be the inputs to the scalar fgraph (as
    /// `Node::Argument`), and the outputs as `Node::Variable`.
    Elemwise(Elemwise),
    /// Should contain two grahps, one for the if and one for
    /// the else, and a variable for the condition.
    IfElse(IfElse),
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ENode {
    Node(Node),

    Output(usize, Variable),
    Argument(usize, Variable),

    Buffer(Buffer),
    Constant(),
}


impl Language for ENode {
    // TODO update
    fn matches(&self, other: &Self) -> bool {
        use ENode::*;
        match (self, other) {
            (SimpleNode(_), Variable(_)) => false,
            (Variable(_), SimpleNode(_)) => false,
            (Variable(_), Variable(_)) => true,
            (SimpleNode(node1), SimpleNode(node2)) => node1.node_id == node2.node_id,
        }
    }

    fn children(&self) -> &[Id] {
        use ENode::*;
        match self {
            Variable(id) => from_ref(id),
            SimpleNode(node) => &node.parents
        }
    }

    fn children_mut(&mut self) -> &mut [Id] {
        use ENode::*;
        match self {
            Variable(id) => from_mut(id),
            SimpleNode(node) => &mut node.parents
        }
    }
}


fn main() {
    println!("Hello, world!");
}
