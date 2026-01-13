use vstd::prelude::*;

verus! {

pub enum Tree {
    E,  // Empty
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}


pub open spec fn bound(k: nat, t: Tree) -> bool
    decreases t
{
    match t {
        Tree::E => false,
        Tree::T { left, key, value: _, right } =>
            if k < key {
                bound(k, *left)
            } else if k > key {
                bound(k, *right)
            } else {
                true
            }
    }
}

} // verus!