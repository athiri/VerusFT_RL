use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


pub open spec fn tree_any<T>(t: Tree<T>, p: spec_fn(T) -> bool) -> bool
    decreases t
{
    match t {
        Tree::Leaf => false,
        Tree::Node { left, value, right } =>
            p(value) ||
            tree_any(*left, p) ||
            tree_any(*right, p),
    }
}

} // verus!