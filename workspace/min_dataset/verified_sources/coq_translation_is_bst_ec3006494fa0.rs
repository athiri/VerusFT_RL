use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


pub open spec fn is_bst(t: Tree<nat>, lo: nat, hi: nat) -> bool
    decreases t
{
    match t {
        Tree::Leaf => true,
        Tree::Node { left, value, right } =>
            lo <= value && value < hi &&
            is_bst(*left, lo, value) &&
            is_bst(*right, value + 1, hi),
    }
}

} // verus!