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


pub open spec fn black_height(t: RBTree) -> nat
    decreases t
{
    match t {
        RBTree::E => 0,
        RBTree::T { color, left, key: _, value: _, right: _ } => {
            let lbh = black_height(*left);
            let inc = if color == Color::Black { 1nat } else { 0nat };
            inc + lbh
        }
    }
}

} // verus!