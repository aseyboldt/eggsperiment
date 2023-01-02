use std::{rc::Rc, collections::HashMap, hash::Hash, slice::{from_ref, from_mut}};

use egg::{Language, Id};
use smallvec::SmallVec;


#[derive(Debug)]
struct NodeDB {
    names: HashMap<u32, String>,
}


#[derive(Debug, Clone)]
pub struct SimpleNode {
    db: Rc<NodeDB>,
    node_id: u32,
    parents: SmallVec<[Id; 8]>,
}

impl PartialEq for SimpleNode {
    fn eq(&self, other: &Self) -> bool {
        assert!(Rc::ptr_eq(&self.db, &other.db));
        self.node_id == other.node_id && self.parents == other.parents
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
        self.parents.partial_cmp(&other.parents)
    }
}


impl Ord for SimpleNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert!(Rc::ptr_eq(&self.db, &other.db));
        match self.node_id.cmp(&other.node_id) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.parents.cmp(&other.parents)
    }
}

impl Hash for SimpleNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node_id.hash(state);
        self.parents.hash(state);
    }
}


struct Variable {
    idx: u16,
    owner: Id,
}


struct Elemwise {
    /// first n_inputs should be `Node::Argument`, remaining
    /// n_outputs should be `Node::Output`.
    all_inputs: SmallVec<[Id, 8]>,
    n_inputs: u64,
    n_outputs: u64,
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    /// A node in the pytensor sense, that doesn't have an inner fgraph
    SimpleNode(SimpleNode),

    /// A elemwise node, that needs to also store the scalar fgraph.
    /// The children should be the inputs to the scalar fgraph (as
    /// `Node::Argument`), and the outputs as `Node::Variable`.
    Elemwise(SimpleNode),

    /// Should contain two grahps, one for the if and one for
    /// the else, and a variable for the condition.
    IfElse(IfElse),
    Output(Variable),
    Argument(Variable),
}


impl Language for Node {
    fn matches(&self, other: &Self) -> bool {
        use Node::*;
        match (self, other) {
            (SimpleNode(_), Variable(_)) => false,
            (Variable(_), SimpleNode(_)) => false,
            (Variable(_), Variable(_)) => true,
            (SimpleNode(node1), SimpleNode(node2)) => node1.node_id == node2.node_id,
        }
    }

    fn children(&self) -> &[Id] {
        use Node::*;
        match self {
            Variable(id) => from_ref(id),
            SimpleNode(node) => &node.parents
        }
    }

    fn children_mut(&mut self) -> &mut [Id] {
        use Node::*;
        match self {
            Variable(id) => from_mut(id),
            SimpleNode(node) => &mut node.parents
        }
    }
}


fn main() {
    println!("Hello, world!");
}
