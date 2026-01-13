use vstd::prelude::*;

verus! {

pub enum Tree {
    Leaf,
    Node { left: Box<Tree>, right: Box<Tree> },
}


pub open spec fn tree_size(t: Tree) -> nat
    decreases t
{
    match t {
        Tree::Leaf => 1,
        Tree::Node { left, right } => 1 + tree_size(*left) + tree_size(*right),
    }
}

} // verus!