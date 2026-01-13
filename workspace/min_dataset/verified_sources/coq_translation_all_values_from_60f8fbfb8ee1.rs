use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


pub open spec fn all_values_from<T>(t: Tree<T>, values: Set<T>) -> bool
    decreases t
{
    match t {
        Tree::Leaf => true,
        Tree::Node { left, value, right } =>
            values.contains(value) &&
            all_values_from(*left, values) &&
            all_values_from(*right, values),
    }
}

} // verus!