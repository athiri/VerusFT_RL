use vstd::prelude::*;

verus! {

pub proof fn pow2_double(n: nat)
    ensures pow2(n) + pow2(n) == pow2(n + 1)
{
    reveal_with_fuel(pow2, 2);
}

pub proof fn pow2_monotonic(a: nat, b: nat)
    requires a <= b
    ensures pow2(a) <= pow2(b)
    decreases b
{
    reveal_with_fuel(pow2, 2);
    if a < b {
        pow2_monotonic(a, (b - 1) as nat);
    }
}

pub proof fn pow2_pos(n: nat)
    ensures pow2(n) > 0
    decreases n
{
    reveal_with_fuel(pow2, 2);
    if n > 0 {
        pow2_pos((n - 1) as nat);
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


pub proof fn height_bounds_size<T>(t: Tree<T>)
    ensures tree_size(t) < pow2(tree_height(t))
    decreases t
{
    reveal_with_fuel(tree_size, 2);
    reveal_with_fuel(tree_height, 2);
    match t {
        Tree::Leaf => {
            assert(tree_size(t) == 0);
            assert(tree_height(t) == 0);
            pow2_pos(0);
        }
        Tree::Node { left, value: _, right } => {
            height_bounds_size(*left);
            height_bounds_size(*right);
            let lh = tree_height(*left);
            let rh = tree_height(*right);
            let h = if lh > rh { lh } else { rh };
            pow2_monotonic(lh, h);
            pow2_monotonic(rh, h);
            pow2_double(h);
        }
    }
}

} // verus!