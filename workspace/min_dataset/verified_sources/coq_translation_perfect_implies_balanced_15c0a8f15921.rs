use vstd::prelude::*;

verus! {

pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
}

pub open spec fn is_perfect<T>(t: Tree<T>) -> bool
    decreases t
{
    match t {
        Tree::Leaf => true,
        Tree::Node { left, value: _, right } =>
            tree_height(*left) == tree_height(*right) &&
            is_perfect(*left) &&
            is_perfect(*right),
    }
}

pub open spec fn tree_height<T>(t: Tree<T>) -> nat
    decreases t
{
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, value: _, right } => {
            let lh = tree_height(*left);
            let rh = tree_height(*right);
            1 + if lh > rh { lh } else { rh }
        }
    }
}

pub open spec fn is_balanced<T>(t: Tree<T>) -> bool
    decreases t
{
    match t {
        Tree::Leaf => true,
        Tree::Node { left, value: _, right } => {
            let lh = tree_height(*left);
            let rh = tree_height(*right);
            is_balanced(*left) &&
            is_balanced(*right) &&
            (if lh > rh { lh - rh } else { rh - lh }) <= 1
        }
    }
}


pub proof fn perfect_implies_balanced<T>(t: Tree<T>)
    requires is_perfect(t)
    ensures is_balanced(t)
    decreases t
{
    reveal_with_fuel(is_perfect, 2);
    reveal_with_fuel(is_balanced, 2);
    reveal_with_fuel(tree_height, 2);
    match t {
        Tree::Leaf => {}
        Tree::Node { left, value: _, right } => {
            perfect_implies_balanced(*left);
            perfect_implies_balanced(*right);
        }
    }
}

} // verus!