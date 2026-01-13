use vstd::prelude::*;

verus! {

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

pub open spec fn tree_leaves<T>(t: Tree<T>) -> nat
    decreases t
{
    match t {
        Tree::Leaf => 1,
        Tree::Node { left, value: _, right } =>
            tree_leaves(*left) + tree_leaves(*right),
    }
}


pub proof fn leaves_nodes_relation<T>(t: Tree<T>)
    ensures tree_leaves(t) == tree_size(t) + 1
    decreases t
{
    reveal_with_fuel(tree_size, 2);
    reveal_with_fuel(tree_leaves, 2);
    match t {
        Tree::Leaf => {}
        Tree::Node { left, value: _, right } => {
            leaves_nodes_relation(*left);
            leaves_nodes_relation(*right);
        }
    }
}

} // verus!