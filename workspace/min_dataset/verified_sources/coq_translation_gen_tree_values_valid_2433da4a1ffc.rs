use vstd::prelude::*;

verus! {

pub open spec fn tree_size<T>(t: Tree<T>) -> nat
    decreases t
{
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, value: _, right } =>
            1 + tree_size(*left) + tree_size(*right),
    }
}


pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}

pub open spec fn gen_tree_outputs<T>(inner_outputs: Set<T>, max_size: nat) -> Set<Tree<T>> {
    Set::new(|t: Tree<T>|
        tree_size(t) <= max_size &&
        all_values_from(t, inner_outputs)
    )
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


pub proof fn gen_tree_values_valid<T>(inner_outputs: Set<T>, max_size: nat, t: Tree<T>)
    requires gen_tree_outputs(inner_outputs, max_size).contains(t)
    ensures all_values_from(t, inner_outputs)
{
}

} // verus!