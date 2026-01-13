use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}

pub open spec fn bst_contains(t: Tree<nat>, x: nat) -> bool
    decreases t
{
    match t {
        Tree::Leaf => false,
        Tree::Node { left, value, right } =>
            if x == value {
                true
            } else if x < value {
                bst_contains(*left, x)
            } else {
                bst_contains(*right, x)
            },
    }
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


pub proof fn bst_contains_implies_tree_contains(t: Tree<nat>, x: nat)
    requires bst_contains(t, x)
    ensures tree_contains(t, x, |a: nat, b: nat| a == b)
    decreases t
{
    reveal_with_fuel(bst_contains, 3);
    reveal_with_fuel(tree_contains, 3);

    match t {
        Tree::Leaf => {}
        Tree::Node { left, value, right } => {
            if x == value {
            } else if x < value {
                bst_contains_implies_tree_contains(*left, x);
            } else {
                bst_contains_implies_tree_contains(*right, x);
            }
        }
    }
}

} // verus!