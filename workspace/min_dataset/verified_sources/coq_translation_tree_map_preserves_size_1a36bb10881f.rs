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


pub proof fn tree_map_preserves_size<T, U>(t: Tree<T>, f: spec_fn(T) -> U)
    ensures tree_size(tree_map(t, f)) == tree_size(t)
    decreases t
{
    reveal_with_fuel(tree_size, 2);
    reveal_with_fuel(tree_map, 2);
    match t {
        Tree::Leaf => {}
        Tree::Node { left, value: _, right } => {
            tree_map_preserves_size(*left, f);
            tree_map_preserves_size(*right, f);
        }
    }
}

} // verus!