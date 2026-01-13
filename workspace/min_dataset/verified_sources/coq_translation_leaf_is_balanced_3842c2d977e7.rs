use vstd::prelude::*;

verus! {

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


pub enum Tree<T> {
    Leaf,
    Node { left: Box<Tree<T>>, value: T, right: Box<Tree<T>> },
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


pub proof fn leaf_is_balanced<T>()
    ensures is_balanced::<T>(Tree::Leaf)
{
}

} // verus!