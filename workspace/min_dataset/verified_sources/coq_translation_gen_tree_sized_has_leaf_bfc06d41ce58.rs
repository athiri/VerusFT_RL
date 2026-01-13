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


pub proof fn gen_tree_sized_has_leaf<T>(inner_outputs: Set<T>, size: nat)
    ensures gen_tree_sized(inner_outputs, size).contains(Tree::Leaf)
    decreases size
{
    if size == 0 {
        // Base case: set![Tree::Leaf] contains Tree::Leaf
    } else {
        // Inductive case: Leaf matches the first branch of the match
    }
}

} // verus!