use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


pub open spec fn tree_contains<T>(t: Tree<T>, x: T, eq: spec_fn(T, T) -> bool) -> bool
    decreases t
{
    match t {
        Tree::Leaf => false,
        Tree::Node { left, value, right } =>
            eq(value, x) ||
            tree_contains(*left, x, eq) ||
            tree_contains(*right, x, eq),
    }
}

} // verus!