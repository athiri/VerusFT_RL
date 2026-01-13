use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


pub open spec fn bst_contains(t: Tree<nat>, x: nat) -> bool
    decreases t
{
    match t {
        Tree::Leaf => false,
        Tree::Node { left, value, right } =>
            if x == value {
                true
            } else if x < value {
                bst_contains(*left, x)
            } else {
                bst_contains(*right, x)
            },
    }
}

} // verus!