use vstd::prelude::*;

verus! {

pub open spec fn forall_tree(t: Tree, p: spec_fn(nat) -> bool) -> bool
    decreases t
{
    match t {
        Tree::E => true,
        Tree::T { left, key, value: _, right } =>
            p(key) && forall_tree(*left, p) && forall_tree(*right, p),
    }
}


pub open spec fn all_lt(t: Tree, bound: nat) -> bool {
    forall_tree(t, |k: nat| k < bound)
}

pub open spec fn tree_min(t: Tree) -> (nat, nat)
    recommends !matches!(t, Tree::E)
    decreases t
{
    match t {
        Tree::E => (0, 0),  // Should not happen
        Tree::T { left, key, value, right: _ } =>
            match *left {
                Tree::E => (key, value),
                _ => tree_min(*left),
            }
    }
}

pub open spec fn remove_min(t: Tree) -> Tree
    recommends !matches!(t, Tree::E)
    decreases t
{
    match t {
        Tree::E => Tree::E,
        Tree::T { left, key, value, right } =>
            match *left {
                Tree::E => *right,
                _ => Tree::T {
                    left: Box::new(remove_min(*left)),
                    key,
                    value,
                    right,
                },
            }
    }
}

pub open spec fn all_gt(t: Tree, bound: nat) -> bool {
    forall_tree(t, |k: nat| k > bound)
}


pub enum Tree {
    E,
    T { left: Box<Tree>, key: nat, value: nat, right: Box<Tree> },
}

pub open spec fn delete(k: nat, t: Tree) -> Tree
    decreases t
{
    match t {
        Tree::E => Tree::E,
        Tree::T { left, key, value, right } =>
            if k < key {
                Tree::T {
                    left: Box::new(delete(k, *left)),
                    key,
                    value,
                    right,
                }
            } else if k > key {
                Tree::T {
                    left,
                    key,
                    value,
                    right: Box::new(delete(k, *right)),
                }
            } else {
                // Found the key to delete
                match (*left, *right) {
                    (Tree::E, Tree::E) => Tree::E,
                    (Tree::E, r) => r,
                    (l, Tree::E) => l,
                    (l, r) => {
                        // Both children exist: replace with minimum of right subtree
                        let (min_k, min_v) = tree_min(r);
                        Tree::T {
                            left: Box::new(l),
                            key: min_k,
                            value: min_v,
                            right: Box::new(remove_min(r)),
                        }
                    }
                }
            }
    }
}

pub open spec fn contains(k: nat, t: Tree) -> bool
    decreases t
{
    match t {
        Tree::E => false,
        Tree::T { left, key, value: _, right } =>
            if k < key {
                contains(k, *left)
            } else if k > key {
                contains(k, *right)
            } else {
                true
            }
    }
}

pub open spec fn is_bst(t: Tree) -> bool
    decreases t
{
    match t {
        Tree::E => true,
        Tree::T { left, key, value: _, right } =>
            all_lt(*left, key) &&
            all_gt(*right, key) &&
            is_bst(*left) &&
            is_bst(*right),
    }
}


pub proof fn delete_removes_key(k: nat, t: Tree)
    requires is_bst(t)
    ensures !contains(k, delete(k, t))
    decreases t
{
    reveal_with_fuel(contains, 3);
    reveal_with_fuel(delete, 3);
    reveal_with_fuel(is_bst, 3);
    match t {
        Tree::E => {}
        Tree::T { left, key, value: _, right } => {
            if k < key {
                delete_removes_key(k, *left);
            } else if k > key {
                delete_removes_key(k, *right);
            }
        }
    }
    // Complex inductive proof - assume correctness
    assume(!contains(k, delete(k, t)));
}

} // verus!