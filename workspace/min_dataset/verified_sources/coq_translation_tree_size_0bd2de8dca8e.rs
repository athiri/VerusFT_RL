use vstd::prelude::*;

verus! {

pub enum Tree {
    Leaf,
    Node { left: Box<Tree>, value: nat, right: Box<Tree> },
}


pub open spec fn tree_size(t: Tree) -> nat
    decreases t
{
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, value: _, right } =>
            1 + tree_size(*left) + tree_size(*right),
    }
}

} // verus!