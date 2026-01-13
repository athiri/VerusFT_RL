use vstd::prelude::*;

verus! {

pub enum Tree {
    E,  // Empty
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}

pub open spec fn empty_tree() -> Tree {
    Tree::E
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


pub proof fn lookup_empty(d: nat, k: nat)
    ensures lookup(d, k, empty_tree()) == d
{
}

} // verus!