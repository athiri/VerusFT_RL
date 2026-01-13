use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


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

} // verus!