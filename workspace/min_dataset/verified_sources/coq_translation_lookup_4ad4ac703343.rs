use vstd::prelude::*;

verus! {

pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}


pub open spec fn lookup(d: nat, k: nat, t: Tree) -> nat
    decreases t
{
    match t {
        Tree::E => d,
        Tree::T { left, key, value, right } =>
            if k < key {
                lookup(d, k, *left)
            } else if k > key {
                lookup(d, k, *right)
            } else {
                value
            }
    }
}

} // verus!