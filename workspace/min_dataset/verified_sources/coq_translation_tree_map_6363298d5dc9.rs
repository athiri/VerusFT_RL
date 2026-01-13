use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


pub open spec fn tree_map<T, U>(t: Tree<T>, f: spec_fn(T) -> U) -> Tree<U>
    decreases t
{
    match t {
        Tree::Leaf => Tree::Leaf,
        Tree::Node { left, value, right } =>
            Tree::Node {
                left: Box::new(tree_map(*left, f)),
                value: f(value),
                right: Box::new(tree_map(*right, f)),
            },
    }
}

} // verus!