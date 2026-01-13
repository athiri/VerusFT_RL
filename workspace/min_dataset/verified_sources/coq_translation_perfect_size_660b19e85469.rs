use vstd::prelude::*;

verus! {

pub proof fn pow2_double(n: nat)
    ensures pow2(n) + pow2(n) == pow2(n + 1)
{
    reveal_with_fuel(pow2, 2);
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

pub open spec fn pow2(n: nat) -> nat
    decreases n
{
    if n == 0 { 1 } else { 2 * pow2((n - 1) as nat) }
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


pub proof fn perfect_size<T>(t: Tree<T>)
    requires is_perfect(t)
    ensures tree_size(t) == pow2(tree_height(t)) - 1
    decreases t
{
    reveal_with_fuel(tree_size, 2);
    reveal_with_fuel(tree_height, 2);
    reveal_with_fuel(is_perfect, 2);
    reveal_with_fuel(pow2, 2);
    match t {
        Tree::Leaf => {}
        Tree::Node { left, value: _, right } => {
            perfect_size(*left);
            perfect_size(*right);
            let h = tree_height(*left);
            // size = 1 + (2^h - 1) + (2^h - 1) = 2^(h+1) - 1
            pow2_double(h);
        }
    }
}

} // verus!