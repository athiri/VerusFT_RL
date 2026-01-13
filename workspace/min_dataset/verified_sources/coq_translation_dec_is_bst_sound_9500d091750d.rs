use vstd::prelude::*;

verus! {

pub open spec fn is_bst_bounded(t: Tree<nat>, lo: int, hi: int) -> bool
    decreases t
{
    match t {
        Tree::Leaf => true,
        Tree::Node { left, value, right } =>
            lo < value && (value as int) < hi &&
            is_bst_bounded(*left, lo, value as int) &&
            is_bst_bounded(*right, value as int, hi),
    }
}

pub open spec fn bool_to_dec(b: bool) -> Dec {
    if b { Dec::Yes } else { Dec::No }
}


pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}

pub enum Dec {
    Yes,
    No,
}

pub open spec fn dec_is_bst(t: Tree<nat>) -> Dec {
    bool_to_dec(is_bst(t))
}

pub open spec fn dec_to_bool(d: Dec) -> bool {
    match d {
        Dec::Yes => true,
        Dec::No => false,
    }
}

pub open spec fn is_bst(t: Tree<nat>) -> bool {
    is_bst_bounded(t, -1, 0x7FFFFFFF) // Use wide bounds
}


pub proof fn dec_is_bst_sound(t: Tree<nat>)
    ensures dec_to_bool(dec_is_bst(t)) <==> is_bst(t)
{
}

} // verus!