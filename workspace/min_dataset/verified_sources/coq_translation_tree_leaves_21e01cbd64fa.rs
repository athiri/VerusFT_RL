use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


pub open spec fn tree_leaves<T>(t: Tree<T>) -> nat
    decreases t
{
    match t {
        Tree::Leaf => 1,
        Tree::Node { left, value: _, right } =>
            tree_leaves(*left) + tree_leaves(*right),
    }
}

} // verus!