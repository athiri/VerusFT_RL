use vstd::prelude::*;

verus! {

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


pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}

pub open spec fn tree_size<T>(t: Tree<T>) -> nat
    decreases t
{
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, value: _, right } =>
            1 + tree_size(*left) + tree_size(*right),
    }
}

pub open spec fn gen_tree_exact_size<T>(inner_outputs: Set<T>, size: nat) -> Set<Tree<T>> {
    Set::new(|t: Tree<T>|
        tree_size(t) == size &&
        all_values_from(t, inner_outputs)
    )
}


pub proof fn gen_tree_size_0_is_leaf<T>(inner_outputs: Set<T>, t: Tree<T>)
    requires gen_tree_exact_size(inner_outputs, 0nat).contains(t)
    ensures t == Tree::<T>::Leaf
{
    reveal_with_fuel(tree_size, 2);
}

} // verus!