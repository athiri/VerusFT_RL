use vstd::prelude::*;

verus! {

pub open spec fn avl_height(t: AVL) -> nat {
    match t { AVL::E => 0, AVL::T { height, .. } => height }
}


pub enum AVL {
    E,
    T { height: nat, left: Box<AVL>, key: int, value: nat, right: Box<AVL> },
}

pub open spec fn balance_factor(t: AVL) -> int {
    match t {
        AVL::E => 0,
        AVL::T { left, right, .. } => avl_height(*right) as int - avl_height(*left) as int
    }
}


pub open spec fn is_balanced(t: AVL) -> bool decreases t {
    match t {
        AVL::E => true,
        AVL::T { left, right, .. } =>
            -1 <= balance_factor(t) <= 1 && is_balanced(*left) && is_balanced(*right)
    }
}

} // verus!