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


pub open spec fn rb_height(t: RBTree) -> nat
    decreases t
{
    match t {
        RBTree::E => 0,
        RBTree::T { color: _, left, key: _, value: _, right } => {
            let lh = rb_height(*left);
            let rh = rb_height(*right);
            1 + if lh > rh { lh } else { rh }
        }
    }
}

} // verus!