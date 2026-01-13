use vstd::prelude::*;

verus! {

pub enum Tree {
    E,  // Empty
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}


pub open spec fn tree_size(t: Tree) -> nat
    decreases t
{
    match t {
        Tree::E => 0,
        Tree::T { left, key: _, value: _, right } =>
            1 + tree_size(*left) + tree_size(*right),
    }
}

} // verus!