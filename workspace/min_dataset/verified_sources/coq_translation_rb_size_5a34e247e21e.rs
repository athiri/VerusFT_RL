use vstd::prelude::*;

verus! {

pub enum Color {
    Red,
    Black,
}

pub enum RBTree {
    E,  // Empty (implicitly black)
    T {
        color: Color,
        left: Box<RBTree>,
        key: int,
        value: nat,
        right: Box<RBTree>,
    },
}


pub open spec fn rb_size(t: RBTree) -> nat
    decreases t
{
    match t {
        RBTree::E => 0,
        RBTree::T { color: _, left, key: _, value: _, right } =>
            1 + rb_size(*left) + rb_size(*right),
    }
}

} // verus!