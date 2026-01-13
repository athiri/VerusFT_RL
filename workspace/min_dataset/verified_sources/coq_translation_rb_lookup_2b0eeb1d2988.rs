use vstd::prelude::*;

verus! {

pub enum Color { Red, Black }

pub enum RBTree {
    E,
    T { color: Color, left: Box<RBTree>, key: int, value: nat, right: Box<RBTree> },
}


pub open spec fn rb_lookup(d: nat, k: int, t: RBTree) -> nat
    decreases t
{
    match t {
        RBTree::E => d,
        RBTree::T { left, key, value, right, .. } =>
            if k < key { rb_lookup(d, k, *left) }
            else if k > key { rb_lookup(d, k, *right) }
            else { value }
    }
}

} // verus!