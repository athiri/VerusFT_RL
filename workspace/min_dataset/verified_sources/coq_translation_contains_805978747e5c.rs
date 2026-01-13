use vstd::prelude::*;

verus! {

pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}


pub open spec fn contains(k: nat, t: Tree) -> bool
    decreases t
{
    match t {
        Tree::E => false,
        Tree::T { left, key, value: _, right } =>
            if k < key {
                contains(k, *left)
            } else if k > key {
                contains(k, *right)
            } else {
                true
            }
    }
}

} // verus!