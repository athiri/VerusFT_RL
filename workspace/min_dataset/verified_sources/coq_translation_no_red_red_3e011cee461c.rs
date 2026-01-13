use vstd::prelude::*;

verus! {

pub open spec fn get_color(t: RBTree) -> Color {
    match t {
        RBTree::E => Color::Black,
        RBTree::T { color, .. } => color,
    }
}


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


pub open spec fn no_red_red(t: RBTree) -> bool
    decreases t
{
    match t {
        RBTree::E => true,
        RBTree::T { color, left, key: _, value: _, right } => {
            let left_ok = no_red_red(*left);
            let right_ok = no_red_red(*right);
            let this_ok = if color == Color::Red {
                get_color(*left) == Color::Black && get_color(*right) == Color::Black
            } else {
                true
            };
            this_ok && left_ok && right_ok
        }
    }
}

} // verus!