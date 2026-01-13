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

pub open spec fn is_bst(t: Tree<nat>, lo: nat, hi: nat) -> bool
    decreases t
{
    match t {
        Tree::Leaf => true,
        Tree::Node { left, value, right } =>
            lo <= value && value < hi &&
            is_bst(*left, lo, value) &&
            is_bst(*right, value + 1, hi),
    }
}


pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}

pub open spec fn gen_bst_outputs(max_size: nat, lo: nat, hi: nat) -> Set<Tree<nat>> {
    Set::new(|t: Tree<nat>|
        tree_size(t) <= max_size &&
        is_bst(t, lo, hi)
    )
}


pub proof fn gen_bst_contains_leaf(max_size: nat, lo: nat, hi: nat)
    ensures gen_bst_outputs(max_size, lo, hi).contains(Tree::Leaf)
{
}

} // verus!