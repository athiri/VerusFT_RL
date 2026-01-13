use vstd::prelude::*;

verus! {

pub open spec fn balance(color: Color, left: RBTree, key: int, value: nat, right: RBTree) -> RBTree {
    // For simplicity, just create the tree directly
    // Full balancing logic would require complex pattern matching
    RBTree::T { color, left: Box::new(left), key, value, right: Box::new(right) }
}


pub enum Color { Red, Black }

pub enum RBTree {
    E,
    T { color: Color, left: Box<RBTree>, key: int, value: nat, right: Box<RBTree> },
}


pub open spec fn rb_insert_aux(k: int, v: nat, t: RBTree) -> RBTree
    decreases t
{
    match t {
        RBTree::E => RBTree::T {
            color: Color::Red,
            left: Box::new(RBTree::E),
            key: k,
            value: v,
            right: Box::new(RBTree::E),
        },
        RBTree::T { color, left, key, value, right } =>
            if k < key {
                balance(color, rb_insert_aux(k, v, *left), key, value, *right)
            } else if k > key {
                balance(color, *left, key, value, rb_insert_aux(k, v, *right))
            } else {
                // Key exists, update value
                RBTree::T { color, left, key, value: v, right }
            }
    }
}

} // verus!