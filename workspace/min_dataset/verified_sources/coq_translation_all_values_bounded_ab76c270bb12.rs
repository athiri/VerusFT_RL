use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


pub open spec fn all_values_bounded(t: Tree<nat>, bound: nat) -> bool
    decreases t
{
    match t {
        Tree::Leaf => true,
        Tree::Node { left, value, right } =>
            value <= bound &&
            all_values_bounded(*left, bound) &&
            all_values_bounded(*right, bound),
    }
}

} // verus!