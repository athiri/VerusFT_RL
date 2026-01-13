use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}


pub open spec fn gen_tree_sized<T>(inner_outputs: Set<T>, size: nat) -> Set<Tree<T>>
    decreases size
{
    if size == 0 {
        set![Tree::Leaf]
    } else {
        Set::new(|t: Tree<T>| match t {
            Tree::Leaf => true,
            Tree::Node { left, value, right } =>
                inner_outputs.contains(value) &&
                gen_tree_sized(inner_outputs, (size - 1) as nat).contains(*left) &&
                gen_tree_sized(inner_outputs, (size - 1) as nat).contains(*right),
        })
    }
}

} // verus!